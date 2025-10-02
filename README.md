# Kizo Protocol - Indexer

[![Rust](https://img.shields.io/badge/rust-1.78%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Network](https://img.shields.io/badge/network-Aptos%20Testnet-brightgreen.svg)](https://explorer.aptoslabs.com/)

## Overview

The Kizo Indexer is a high-performance blockchain event indexer designed for the Kizo Prediction Market protocol on the Aptos blockchain. It continuously monitors and indexes on-chain events, storing structured data in a PostgreSQL database for efficient querying and analytics.

### Key Features

- **Real-time Event Processing**: Streams and processes blockchain transactions in real-time
- **Comprehensive Event Coverage**: Indexes all critical prediction market events
- **Parallel Processing**: Leverages Rayon for efficient multi-threaded transaction processing
- **Automatic Migrations**: Built-in database schema management with Diesel migrations
- **Production-Ready**: Built on the Aptos Indexer Processor SDK with enterprise-grade reliability

## Architecture

### Event Types Indexed

The indexer monitors and processes the following smart contract events:

| Event | Description | Database Table |
|-------|-------------|----------------|
| `MarketCreatedEvent` | New prediction market creation | `markets` |
| `BetPlacedEvent` | User bet/position placement | `bets` |
| `MarketResolvedEvent` | Market outcome resolution | `market_resolutions` |
| `WinningsClaimedEvent` | Winner claim transactions | `winnings_claims` |
| `YieldDepositedEvent` | Yield generation tracking | `yield_deposits` |
| `ProtocolFeeCollectedEvent` | Protocol fee collection | `protocol_fees` |

### Contract Information

- **Contract Address**: `0x66c4ec614f237de2470e107a17329e17d2e9d04bd6f609bdb7f7b52ae24c957c`
- **Network**: Aptos Testnet
- **Protocol Fee**: 5% (500 basis points)
- **Explorer**: [View on Aptos Explorer](https://explorer.aptoslabs.com/account/0x66c4ec614f237de2470e107a17329e17d2e9d04bd6f609bdb7f7b52ae24c957c?network=testnet)

## Prerequisites

- **Rust**: 1.78 or higher
- **PostgreSQL**: 12 or higher
- **Aptos CLI** (optional, for contract interaction)
- **Diesel CLI** (for database migrations)

### Installing Diesel CLI

```bash path=null start=null
cargo install diesel_cli --no-default-features --features postgres
```

## Installation

### 1. Clone the Repository

```bash path=null start=null
git clone <repository-url>
cd kizo/aptos/indexer
```

### 2. Database Setup

Create a PostgreSQL database:

```bash path=null start=null
creatdb kizo_indexer
```

### 3. Configure Database Connection

Update `config.yaml` with your database credentials:

```yaml path=null start=null
postgres_config:
  connection_string: postgresql://username@localhost/kizo_indexer
```

### 4. Run Migrations

The indexer will automatically run migrations on startup, or you can run them manually:

```bash path=null start=null
diesel migration run --config-file diesel.toml
```

### 5. Build the Project

```bash path=null start=null
cargo build --release
```

## Configuration

The indexer is configured via `config.yaml`. Key configuration options:

```yaml path=null start=null
health_check_port: 8085
server_config:
  transaction_stream_config:
    indexer_grpc_data_service_address: "https://grpc.testnet.aptoslabs.com:443"
    auth_token: "<your-auth-token>"
    request_name_header: "kizo-prediction-market-indexer"
    starting_version: 6890217349  # Starting block version
  postgres_config:
    connection_string: postgresql://user@localhost/kizo_indexer
```

### Configuration Parameters

- **health_check_port**: Port for health check endpoint
- **indexer_grpc_data_service_address**: Aptos gRPC endpoint URL
- **auth_token**: Authentication token for Aptos indexer service
- **starting_version**: Blockchain version to start indexing from
- **connection_string**: PostgreSQL connection string

## Usage

### Running the Indexer

```bash path=null start=null
cargo run --release
```

The indexer will:
1. Connect to the PostgreSQL database
2. Run any pending migrations
3. Begin streaming transactions from the configured starting version
4. Process events and insert data into the database

### Development Mode

```bash path=null start=null
cargo run
```

### Health Check

Verify the indexer is running:

```bash path=null start=null
curl http://localhost:8085/health
```

## Database Schema

### Markets Table

Stores prediction market metadata:

- `market_id` (PK): Unique market identifier
- `title`: Market question/title
- `description`: Detailed market description
- `category`: Market category
- `end_time`: Market closing timestamp
- `yes_shares`, `no_shares`: Total shares for each outcome
- `total_liquidity`: Total liquidity in the market
- Additional metadata and transaction tracking fields

### Bets Table

Records all betting activity:

- `bet_id` (PK): Unique bet identifier
- `market_id` (FK): Associated market
- `user_address`: Better's wallet address
- `bet_type`: YES or NO position
- `amount`: Bet amount
- `shares_received`: Shares allocated
- Transaction metadata

### Market Resolutions Table

Tracks market outcomes:

- `market_id` (PK): Resolved market identifier
- `outcome`: Final outcome (YES/NO)
- `total_yield_earned`: Total yield generated
- Resolution transaction details

### Additional Tables

- **winnings_claims**: User winnings claim records
- **yield_deposits**: Yield generation history
- **protocol_fees**: Protocol fee collection tracking

## Development

### Project Structure

```
kizo-indexer/
├── src/
│   ├── main.rs              # Main indexer logic
│   ├── models.rs            # Database models & event parsers
│   └── db/
│       └── schema.rs        # Diesel schema definitions
├── migrations/              # Database migrations
├── config.yaml             # Indexer configuration
├── Cargo.toml              # Rust dependencies
└── workspace modules/
    ├── sdk/                # Indexer SDK
    ├── transaction-stream/ # Transaction streaming
    └── instrumented-channel/ # Monitoring utilities
```

### Testing

```bash path=null start=null
cargo test
```

### Code Formatting

```bash path=null start=null
cargo fmt --all
```

### Linting

```bash path=null start=null
cargo clippy --all-targets --all-features
```

## Monitoring & Observability

The indexer includes structured logging via the `tracing` crate:

- **Info**: Successful event processing
- **Warn**: Missing transaction data
- **Error**: Event parsing failures

Logs include contextual information such as transaction versions and event types.

## Troubleshooting

### Connection Issues

**Problem**: Cannot connect to PostgreSQL

**Solution**: Verify database is running and connection string is correct:
```bash path=null start=null
psql -h localhost -U username -d kizo_indexer
```

### Missing Events

**Problem**: Events not being indexed

**Solution**: 
1. Check the `starting_version` in config.yaml
2. Verify contract address matches deployed contract
3. Review logs for parsing errors

### Performance Issues

**Problem**: Slow indexing speed

**Solution**:
1. Increase PostgreSQL connection pool size
2. Add database indexes on frequently queried columns
3. Monitor system resources (CPU, memory, disk I/O)

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Standards

- Follow Rust idioms and best practices
- Maintain comprehensive error handling
- Add tests for new functionality
- Update documentation as needed

## License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details.

## Support

For questions, issues, or contributions:

- **Issues**: Open an issue on GitHub
- **Documentation**: [Kizo Protocol Docs](https://kizoprotocol.gitbook.io/kizoprotocol-docs)
- **Community**: [Kizo Protocol X](https://x.com/kizoprotocol)

## Acknowledgments

- Built on [Aptos Indexer Processor SDK](https://github.com/aptos-labs/aptos-indexer-processor-sdk)
- Powered by [Aptos Labs](https://aptoslabs.com) infrastructure

---

**Last Updated**: October 1, 2025  
**Version**: 0.1.0  
**Status**: Active Development
# CI/CD Fix Applied - Thu Oct  2 10:23:34 WIB 2025
[INFO] : Testing professional CI/CD pipeline - Thu Oct  2 10:54:32 WIB 2025
