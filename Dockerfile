# Build stage
FROM rustlang/rust:nightly as builder

WORKDIR /app

# Install build dependencies including libdw for profiling support
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    libdw-dev \
    libelf-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy entire workspace
COPY . .

# Build the application in release mode
RUN cargo build --release --bin kizo-indexer

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/kizo-indexer /usr/local/bin/kizo-indexer

# Copy the config file
COPY --from=builder /app/config.yaml ./config.yaml

# Set environment
ENV RUST_LOG=info

EXPOSE 8081

CMD ["kizo-indexer"]
