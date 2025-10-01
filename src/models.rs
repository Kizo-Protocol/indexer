#![allow(clippy::extra_unused_lifetimes)]

use crate::schema::{bets, market_resolutions, markets, protocol_fees, winnings_claims, yield_deposits};
use aptos_indexer_processor_sdk::{
    aptos_protos::transaction::v1::Event as EventPB,
    utils::convert::standardize_address,
};
use diesel::{Identifiable, Insertable, Queryable};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

// ===== Markets =====

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize, Queryable)]
#[diesel(primary_key(market_id))]
#[diesel(table_name = markets)]
pub struct Market {
    pub market_id: i64,
    pub question: String,
    pub end_time: i64,
    pub yield_protocol_addr: String,
    pub transaction_version: i64,
    pub transaction_block_height: i64,
    pub inserted_at: chrono::NaiveDateTime,
    pub resolved: Option<bool>,
    pub outcome: Option<bool>,
    pub total_yield_earned: Option<i64>,
    pub resolution_transaction_version: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketCreatedEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub market_id: u64,
    pub question: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub end_time: u64,
    pub yield_protocol_addr: String,
}

fn deserialize_string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

impl Market {
    pub fn from_event(
        event: &MarketCreatedEvent,
        transaction_version: i64,
        transaction_block_height: i64,
    ) -> Self {
        Market {
            market_id: event.market_id as i64,
            question: event.question.clone(),
            end_time: event.end_time as i64,
            yield_protocol_addr: standardize_address(&event.yield_protocol_addr),
            transaction_version,
            transaction_block_height,
            inserted_at: chrono::Utc::now().naive_utc(),
            resolved: Some(false),
            outcome: None,
            total_yield_earned: Some(0),
            resolution_transaction_version: None,
        }
    }
}

// ===== Bets =====

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize, Queryable)]
#[diesel(primary_key(bet_id))]
#[diesel(table_name = bets)]
pub struct Bet {
    pub bet_id: i64,
    pub market_id: i64,
    pub user_addr: String,
    pub position: bool,
    pub amount: i64,
    pub transaction_version: i64,
    pub transaction_block_height: i64,
    pub inserted_at: chrono::NaiveDateTime,
    pub claimed: Option<bool>,
    pub winning_amount: Option<i64>,
    pub yield_share: Option<i64>,
    pub claim_transaction_version: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BetPlacedEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub bet_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub market_id: u64,
    pub user: String,
    pub position: bool,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub amount: u64,
}

impl Bet {
    pub fn from_event(
        event: &BetPlacedEvent,
        transaction_version: i64,
        transaction_block_height: i64,
    ) -> Self {
        Bet {
            bet_id: event.bet_id as i64,
            market_id: event.market_id as i64,
            user_addr: standardize_address(&event.user),
            position: event.position,
            amount: event.amount as i64,
            transaction_version,
            transaction_block_height,
            inserted_at: chrono::Utc::now().naive_utc(),
            claimed: Some(false),
            winning_amount: Some(0),
            yield_share: Some(0),
            claim_transaction_version: None,
        }
    }
}

// ===== Market Resolutions =====

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize, Queryable)]
#[diesel(primary_key(market_id))]
#[diesel(table_name = market_resolutions)]
pub struct MarketResolution {
    pub market_id: i64,
    pub outcome: bool,
    pub total_yield_earned: i64,
    pub transaction_version: i64,
    pub transaction_block_height: i64,
    pub inserted_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketResolvedEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub market_id: u64,
    pub outcome: bool,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub total_yield_earned: u64,
}

impl MarketResolution {
    pub fn from_event(
        event: &MarketResolvedEvent,
        transaction_version: i64,
        transaction_block_height: i64,
    ) -> Self {
        MarketResolution {
            market_id: event.market_id as i64,
            outcome: event.outcome,
            total_yield_earned: event.total_yield_earned as i64,
            transaction_version,
            transaction_block_height,
            inserted_at: chrono::Utc::now().naive_utc(),
        }
    }
}

// ===== Winnings Claims =====

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize, Queryable)]
#[diesel(primary_key(claim_id))]
#[diesel(table_name = winnings_claims)]
pub struct WinningsClaim {
    pub claim_id: i64,
    pub bet_id: i64,
    pub user_addr: String,
    pub winning_amount: i64,
    pub yield_share: i64,
    pub transaction_version: i64,
    pub transaction_block_height: i64,
    pub inserted_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WinningsClaimedEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub bet_id: u64,
    pub user: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub winning_amount: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub yield_share: u64,
}

// Note: claim_id is auto-generated, so we use a NewWinningsClaim for insertion
#[derive(Clone, Debug, Deserialize, FieldCount, Insertable, Serialize)]
#[diesel(table_name = winnings_claims)]
pub struct NewWinningsClaim {
    pub bet_id: i64,
    pub user_addr: String,
    pub winning_amount: i64,
    pub yield_share: i64,
    pub transaction_version: i64,
    pub transaction_block_height: i64,
    pub inserted_at: chrono::NaiveDateTime,
}

impl NewWinningsClaim {
    pub fn from_event(
        event: &WinningsClaimedEvent,
        transaction_version: i64,
        transaction_block_height: i64,
    ) -> Self {
        NewWinningsClaim {
            bet_id: event.bet_id as i64,
            user_addr: standardize_address(&event.user),
            winning_amount: event.winning_amount as i64,
            yield_share: event.yield_share as i64,
            transaction_version,
            transaction_block_height,
            inserted_at: chrono::Utc::now().naive_utc(),
        }
    }
}

// ===== Yield Deposits =====

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize, Queryable)]
#[diesel(primary_key(deposit_id))]
#[diesel(table_name = yield_deposits)]
pub struct YieldDeposit {
    pub deposit_id: i64,
    pub market_id: i64,
    pub amount: i64,
    pub protocol_addr: String,
    pub transaction_version: i64,
    pub transaction_block_height: i64,
    pub inserted_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct YieldDepositedEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub market_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub amount: u64,
    pub protocol_addr: String,
}

// Note: deposit_id is auto-generated, so we use a NewYieldDeposit for insertion
#[derive(Clone, Debug, Deserialize, FieldCount, Insertable, Serialize)]
#[diesel(table_name = yield_deposits)]
pub struct NewYieldDeposit {
    pub market_id: i64,
    pub amount: i64,
    pub protocol_addr: String,
    pub transaction_version: i64,
    pub transaction_block_height: i64,
    pub inserted_at: chrono::NaiveDateTime,
}

impl NewYieldDeposit {
    pub fn from_event(
        event: &YieldDepositedEvent,
        transaction_version: i64,
        transaction_block_height: i64,
    ) -> Self {
        NewYieldDeposit {
            market_id: event.market_id as i64,
            amount: event.amount as i64,
            protocol_addr: standardize_address(&event.protocol_addr),
            transaction_version,
            transaction_block_height,
            inserted_at: chrono::Utc::now().naive_utc(),
        }
    }
}

// ===== Protocol Fees =====

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize, Queryable)]
#[diesel(primary_key(fee_id))]
#[diesel(table_name = protocol_fees)]
pub struct ProtocolFee {
    pub fee_id: i64,
    pub market_id: i64,
    pub fee_amount: i64,
    pub transaction_version: i64,
    pub transaction_block_height: i64,
    pub inserted_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProtocolFeeCollectedEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub market_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub fee_amount: u64,
}

// Note: fee_id is auto-generated, so we use a NewProtocolFee for insertion
#[derive(Clone, Debug, Deserialize, FieldCount, Insertable, Serialize)]
#[diesel(table_name = protocol_fees)]
pub struct NewProtocolFee {
    pub market_id: i64,
    pub fee_amount: i64,
    pub transaction_version: i64,
    pub transaction_block_height: i64,
    pub inserted_at: chrono::NaiveDateTime,
}

impl NewProtocolFee {
    pub fn from_event(
        event: &ProtocolFeeCollectedEvent,
        transaction_version: i64,
        transaction_block_height: i64,
    ) -> Self {
        NewProtocolFee {
            market_id: event.market_id as i64,
            fee_amount: event.fee_amount as i64,
            transaction_version,
            transaction_block_height,
            inserted_at: chrono::Utc::now().naive_utc(),
        }
    }
}

// ===== Helper function to parse events =====

pub fn parse_event_data<T>(event: &EventPB) -> Option<T>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_str(event.data.as_str()).ok()
}

