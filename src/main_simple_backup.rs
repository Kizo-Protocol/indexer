use anyhow::{Context, Result};
use clap::Parser;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use serde::Deserialize;
use std::fs;
use tracing::info;

pub mod models;
#[path = "db/schema.rs"]
pub mod schema;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Parser, Debug)]
#[command(name = "kizo-indexer")]
#[command(about = "Kizo Prediction Market Indexer", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Config {
    grpc_data_stream_endpoint: String,
    grpc_auth_token: Option<String>,
    database_url: String,
    starting_version: u64,
    #[serde(default)]
    contract_address: Option<String>,
}

fn establish_connection_pool(database_url: &str) -> Result<DbPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .context("Failed to create connection pool")
}

fn run_migrations(pool: &DbPool) -> Result<()> {
    let mut conn = pool.get().context("Failed to get database connection")?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Failed to run migrations: {}", e))?;
    info!("Database migrations completed successfully");
    Ok(())
}

// Event type strings from your Move contract
const _MARKET_CREATED_EVENT: &str = "kizo::kizo_prediction_market::MarketCreatedEvent";
const _BET_PLACED_EVENT: &str = "kizo::kizo_prediction_market::BetPlacedEvent";
const _MARKET_RESOLVED_EVENT: &str = "kizo::kizo_prediction_market::MarketResolvedEvent";
const _WINNINGS_CLAIMED_EVENT: &str = "kizo::kizo_prediction_market::WinningsClaimedEvent";
const _YIELD_DEPOSITED_EVENT: &str = "kizo::kizo_prediction_market::YieldDepositedEvent";
const _PROTOCOL_FEE_COLLECTED_EVENT: &str =
    "kizo::kizo_prediction_market::ProtocolFeeCollectedEvent";

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    // Load configuration
    let config_str = fs::read_to_string(&args.config)
        .with_context(|| format!("Failed to read config file: {}", args.config))?;
    let config: Config = serde_yaml::from_str(&config_str)
        .context("Failed to parse config file")?;

    info!("Kizo Prediction Market Indexer starting");
    info!("Config: {:?}", config);

    // Establish database connection
    let pool = establish_connection_pool(&config.database_url)?;

    // Run migrations
    run_migrations(&pool)?;

    info!("Indexer initialized successfully");
    info!("Database schema is ready for indexing");
    info!("Note: This version only sets up the database.");
    info!("For production use, implement gRPC streaming to populate data.");

    Ok(())
}
