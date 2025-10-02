use anyhow::Result;
use aptos_indexer_processor_sdk::{
    aptos_protos::transaction::v1::transaction::TxnData,
    postgres::{
        basic_processor::process,
        utils::database::{execute_in_chunks, MAX_DIESEL_PARAM_SIZE},
    },
};
use diesel::{pg::Pg, query_builder::QueryFragment, upsert::excluded, ExpressionMethods};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use field_count::FieldCount;
use rayon::prelude::*;
use std::env;
use tracing::{error, info, warn};

pub mod models;
#[path = "db/schema.rs"]
pub mod schema;

use models::*;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// Event type strings from your Move contract
// Contract address: 0x66c4ec614f237de2470e107a17329e17d2e9d04bd6f609bdb7f7b52ae24c957c
const MARKET_CREATED_EVENT: &str = "0x66c4ec614f237de2470e107a17329e17d2e9d04bd6f609bdb7f7b52ae24c957c::kizo_prediction_market::MarketCreatedEvent";
const BET_PLACED_EVENT: &str = "0x66c4ec614f237de2470e107a17329e17d2e9d04bd6f609bdb7f7b52ae24c957c::kizo_prediction_market::BetPlacedEvent";
const MARKET_RESOLVED_EVENT: &str = "0x66c4ec614f237de2470e107a17329e17d2e9d04bd6f609bdb7f7b52ae24c957c::kizo_prediction_market::MarketResolvedEvent";
const WINNINGS_CLAIMED_EVENT: &str = "0x66c4ec614f237de2470e107a17329e17d2e9d04bd6f609bdb7f7b52ae24c957c::kizo_prediction_market::WinningsClaimedEvent";
const YIELD_DEPOSITED_EVENT: &str = "0x66c4ec614f237de2470e107a17329e17d2e9d04bd6f609bdb7f7b52ae24c957c::kizo_prediction_market::YieldDepositedEvent";
const PROTOCOL_FEE_COLLECTED_EVENT: &str = "0x66c4ec614f237de2470e107a17329e17d2e9d04bd6f609bdb7f7b52ae24c957c::kizo_prediction_market::ProtocolFeeCollectedEvent";

// Insert query builders
fn insert_markets_query(
    items_to_insert: Vec<Market>,
) -> impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send {
    use schema::markets::dsl::*;
    diesel::insert_into(schema::markets::table)
        .values(items_to_insert)
        .on_conflict(market_id)
        .do_nothing()
}

fn insert_bets_query(
    items_to_insert: Vec<Bet>,
) -> impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send {
    use schema::bets::dsl::*;
    diesel::insert_into(schema::bets::table)
        .values(items_to_insert)
        .on_conflict(bet_id)
        .do_nothing()
}

fn insert_market_resolutions_query(
    items_to_insert: Vec<MarketResolution>,
) -> impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send {
    use schema::market_resolutions::dsl::*;
    diesel::insert_into(schema::market_resolutions::table)
        .values(items_to_insert)
        .on_conflict(market_id)
        .do_update()
        .set((
            outcome.eq(excluded(outcome)),
            total_yield_earned.eq(excluded(total_yield_earned)),
            transaction_version.eq(excluded(transaction_version)),
            transaction_block_height.eq(excluded(transaction_block_height)),
        ))
}

fn insert_winnings_claims_query(
    items_to_insert: Vec<NewWinningsClaim>,
) -> impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send {
    diesel::insert_into(schema::winnings_claims::table)
        .values(items_to_insert)
        .on_conflict_do_nothing()
}

fn insert_yield_deposits_query(
    items_to_insert: Vec<NewYieldDeposit>,
) -> impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send {
    diesel::insert_into(schema::yield_deposits::table)
        .values(items_to_insert)
        .on_conflict_do_nothing()
}

fn insert_protocol_fees_query(
    items_to_insert: Vec<NewProtocolFee>,
) -> impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send {
    diesel::insert_into(schema::protocol_fees::table)
        .values(items_to_insert)
        .on_conflict_do_nothing()
}

#[tokio::main]
async fn main() -> Result<()> {
    process(
        "kizo_prediction_market_indexer".to_string(),
        MIGRATIONS,
        async |transactions, conn_pool| {
            // Process transactions in parallel and collect results
            let processed_data = transactions
                .par_iter()
                .map(|txn| {
                    let txn_version = txn.version as i64;
                    let block_height = txn.block_height as i64;

                    let txn_data = match txn.txn_data.as_ref() {
                        Some(data) => data,
                        None => {
                            warn!(
                                transaction_version = txn_version,
                                "Transaction data doesn't exist"
                            );
                            return (
                                Vec::new(),
                                Vec::new(),
                                Vec::new(),
                                Vec::new(),
                                Vec::new(),
                                Vec::new(),
                            );
                        },
                    };

                    let default = vec![];
                    let raw_events = match txn_data {
                        TxnData::BlockMetadata(tx_inner) => &tx_inner.events,
                        TxnData::Genesis(tx_inner) => &tx_inner.events,
                        TxnData::User(tx_inner) => &tx_inner.events,
                        _ => &default,
                    };

                    let mut markets = Vec::new();
                    let mut bets = Vec::new();
                    let mut market_resolutions = Vec::new();
                    let mut winnings_claims = Vec::new();
                    let mut yield_deposits = Vec::new();
                    let mut protocol_fees = Vec::new();

                    // Process each event
                    for event in raw_events {
                        let event_type = event.type_str.as_str();

                        // Debug log all kizo events
                        if event_type.contains(
                            "0x66c4ec614f237de2470e107a17329e17d2e9d04bd6f609bdb7f7b52ae24c957c",
                        ) {
                            info!(
                                "Found Kizo event at version {}: type={}, data={}",
                                txn_version, event_type, event.data
                            );
                        }

                        match event_type {
                            MARKET_CREATED_EVENT => {
                                match parse_event_data::<MarketCreatedEvent>(event) {
                                    Some(market_event) => {
                                        markets.push(Market::from_event(
                                            &market_event,
                                            txn_version,
                                            block_height,
                                        ));
                                        info!(
                                            "Successfully parsed market at version {}",
                                            txn_version
                                        );
                                    },
                                    None => {
                                        error!(
                                            "Failed to parse MarketCreatedEvent at version {}: {}",
                                            txn_version, event.data
                                        );
                                    },
                                }
                            },
                            BET_PLACED_EVENT => {
                                if let Some(bet_event) = parse_event_data::<BetPlacedEvent>(event) {
                                    bets.push(Bet::from_event(
                                        &bet_event,
                                        txn_version,
                                        block_height,
                                    ));
                                }
                            },
                            MARKET_RESOLVED_EVENT => {
                                if let Some(resolution_event) =
                                    parse_event_data::<MarketResolvedEvent>(event)
                                {
                                    market_resolutions.push(MarketResolution::from_event(
                                        &resolution_event,
                                        txn_version,
                                        block_height,
                                    ));
                                }
                            },
                            WINNINGS_CLAIMED_EVENT => {
                                if let Some(claim_event) =
                                    parse_event_data::<WinningsClaimedEvent>(event)
                                {
                                    winnings_claims.push(NewWinningsClaim::from_event(
                                        &claim_event,
                                        txn_version,
                                        block_height,
                                    ));
                                }
                            },
                            YIELD_DEPOSITED_EVENT => {
                                if let Some(deposit_event) =
                                    parse_event_data::<YieldDepositedEvent>(event)
                                {
                                    yield_deposits.push(NewYieldDeposit::from_event(
                                        &deposit_event,
                                        txn_version,
                                        block_height,
                                    ));
                                }
                            },
                            PROTOCOL_FEE_COLLECTED_EVENT => {
                                if let Some(fee_event) =
                                    parse_event_data::<ProtocolFeeCollectedEvent>(event)
                                {
                                    protocol_fees.push(NewProtocolFee::from_event(
                                        &fee_event,
                                        txn_version,
                                        block_height,
                                    ));
                                }
                            },
                            _ => {
                                // Skip non-Kizo events
                            },
                        }
                    }

                    (
                        markets,
                        bets,
                        market_resolutions,
                        winnings_claims,
                        yield_deposits,
                        protocol_fees,
                    )
                })
                .collect::<Vec<_>>();

            // Flatten all collected data
            let mut markets = Vec::new();
            let mut bets = Vec::new();
            let mut market_resolutions = Vec::new();
            let mut winnings_claims = Vec::new();
            let mut yield_deposits = Vec::new();
            let mut protocol_fees = Vec::new();

            for (m, b, mr, wc, yd, pf) in processed_data {
                markets.extend(m);
                bets.extend(b);
                market_resolutions.extend(mr);
                winnings_claims.extend(wc);
                yield_deposits.extend(yd);
                protocol_fees.extend(pf);
            }

            // Store all data in database
            if !markets.is_empty() {
                match execute_in_chunks(
                    conn_pool.clone(),
                    insert_markets_query,
                    &markets,
                    MAX_DIESEL_PARAM_SIZE / Market::field_count(),
                )
                .await
                {
                    Ok(_) => info!("Stored {} markets", markets.len()),
                    Err(e) => error!("Failed to store markets: {:?}", e),
                }
            }

            if !bets.is_empty() {
                match execute_in_chunks(
                    conn_pool.clone(),
                    insert_bets_query,
                    &bets,
                    MAX_DIESEL_PARAM_SIZE / Bet::field_count(),
                )
                .await
                {
                    Ok(_) => info!("Stored {} bets", bets.len()),
                    Err(e) => error!("Failed to store bets: {:?}", e),
                }
            }

            if !market_resolutions.is_empty() {
                match execute_in_chunks(
                    conn_pool.clone(),
                    insert_market_resolutions_query,
                    &market_resolutions,
                    MAX_DIESEL_PARAM_SIZE / MarketResolution::field_count(),
                )
                .await
                {
                    Ok(_) => info!("Stored {} market resolutions", market_resolutions.len()),
                    Err(e) => error!("Failed to store market resolutions: {:?}", e),
                }
            }

            if !winnings_claims.is_empty() {
                match execute_in_chunks(
                    conn_pool.clone(),
                    insert_winnings_claims_query,
                    &winnings_claims,
                    MAX_DIESEL_PARAM_SIZE / NewWinningsClaim::field_count(),
                )
                .await
                {
                    Ok(_) => info!("Stored {} winnings claims", winnings_claims.len()),
                    Err(e) => error!("Failed to store winnings claims: {:?}", e),
                }
            }

            if !yield_deposits.is_empty() {
                match execute_in_chunks(
                    conn_pool.clone(),
                    insert_yield_deposits_query,
                    &yield_deposits,
                    MAX_DIESEL_PARAM_SIZE / NewYieldDeposit::field_count(),
                )
                .await
                {
                    Ok(_) => info!("Stored {} yield deposits", yield_deposits.len()),
                    Err(e) => error!("Failed to store yield deposits: {:?}", e),
                }
            }

            if !protocol_fees.is_empty() {
                match execute_in_chunks(
                    conn_pool.clone(),
                    insert_protocol_fees_query,
                    &protocol_fees,
                    MAX_DIESEL_PARAM_SIZE / NewProtocolFee::field_count(),
                )
                .await
                {
                    Ok(_) => info!("Stored {} protocol fees", protocol_fees.len()),
                    Err(e) => error!("Failed to store protocol fees: {:?}", e),
                }
            }

            info!(
                "Processed transactions version [{}, {}]",
                transactions.first().map(|t| t.version).unwrap_or(0),
                transactions.last().map(|t| t.version).unwrap_or(0)
            );

            // Trigger backend sync if any new data was stored
            let total_new_items = markets.len()
                + bets.len()
                + market_resolutions.len()
                + winnings_claims.len()
                + yield_deposits.len()
                + protocol_fees.len();

            if total_new_items > 0 {
                trigger_backend_sync(total_new_items).await;
            }

            Ok(())
        },
    )
    .await?;
    Ok(())
}

/// Trigger the backend sync endpoint after new data is indexed
async fn trigger_backend_sync(total_items: usize) {
    // Get backend URL from environment variable
    let backend_url = match env::var("BACKEND_SYNC_URL") {
        Ok(url) => url,
        Err(_) => {
            // Default to localhost if not set
            "http://localhost:3002/api/sync/trigger-full-sync".to_string()
        },
    };

    info!(
        "🔔 Triggering backend sync for {} new items: {}",
        total_items, backend_url
    );

    // Create HTTP client with timeout
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            warn!("⚠️  Failed to create HTTP client for webhook: {}", e);
            return;
        },
    };

    // Trigger the sync endpoint (fire and forget, don't block indexer)
    let backend_url_clone = backend_url.clone();
    tokio::spawn(async move {
        match client
            .post(&backend_url_clone)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "source": "indexer",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    info!("✅ Backend sync triggered successfully");
                } else {
                    warn!(
                        "⚠️  Backend sync endpoint returned status: {}",
                        response.status()
                    );
                }
            },
            Err(e) => {
                warn!("⚠️  Failed to trigger backend sync: {}", e);
            },
        }
    });
}

fn parse_event_data<T: serde::de::DeserializeOwned>(
    event: &aptos_indexer_processor_sdk::aptos_protos::transaction::v1::Event,
) -> Option<T> {
    serde_json::from_str(&event.data).ok()
}
