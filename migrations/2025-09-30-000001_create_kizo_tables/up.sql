-- Kizo Prediction Market Tables

-- Markets table
CREATE TABLE markets (
    market_id BIGINT PRIMARY KEY,
    question TEXT NOT NULL,
    end_time BIGINT NOT NULL,
    yield_protocol_addr VARCHAR(66) NOT NULL,
    transaction_version BIGINT NOT NULL,
    transaction_block_height BIGINT NOT NULL,
    inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
    -- Market status fields (updated when resolved)
    resolved BOOLEAN DEFAULT FALSE,
    outcome BOOLEAN DEFAULT NULL,
    total_yield_earned BIGINT DEFAULT 0,
    resolution_transaction_version BIGINT DEFAULT NULL
);

-- Bets table
CREATE TABLE bets (
    bet_id BIGINT PRIMARY KEY,
    market_id BIGINT NOT NULL REFERENCES markets(market_id),
    user_addr VARCHAR(66) NOT NULL,
    position BOOLEAN NOT NULL, -- true for YES, false for NO
    amount BIGINT NOT NULL,
    transaction_version BIGINT NOT NULL,
    transaction_block_height BIGINT NOT NULL,
    inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
    -- Claim status (updated when winnings claimed)
    claimed BOOLEAN DEFAULT FALSE,
    winning_amount BIGINT DEFAULT 0,
    yield_share BIGINT DEFAULT 0,
    claim_transaction_version BIGINT DEFAULT NULL
);

-- Market resolutions table (separate for denormalization)
CREATE TABLE market_resolutions (
    market_id BIGINT PRIMARY KEY REFERENCES markets(market_id),
    outcome BOOLEAN NOT NULL,
    total_yield_earned BIGINT NOT NULL,
    transaction_version BIGINT NOT NULL,
    transaction_block_height BIGINT NOT NULL,
    inserted_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Winnings claims table
CREATE TABLE winnings_claims (
    claim_id BIGSERIAL PRIMARY KEY,
    bet_id BIGINT NOT NULL REFERENCES bets(bet_id),
    user_addr VARCHAR(66) NOT NULL,
    winning_amount BIGINT NOT NULL,
    yield_share BIGINT NOT NULL,
    transaction_version BIGINT NOT NULL,
    transaction_block_height BIGINT NOT NULL,
    inserted_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Yield deposits table
CREATE TABLE yield_deposits (
    deposit_id BIGSERIAL PRIMARY KEY,
    market_id BIGINT NOT NULL REFERENCES markets(market_id),
    amount BIGINT NOT NULL,
    protocol_addr VARCHAR(66) NOT NULL,
    transaction_version BIGINT NOT NULL,
    transaction_block_height BIGINT NOT NULL,
    inserted_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Protocol fees table
CREATE TABLE protocol_fees (
    fee_id BIGSERIAL PRIMARY KEY,
    market_id BIGINT NOT NULL REFERENCES markets(market_id),
    fee_amount BIGINT NOT NULL,
    transaction_version BIGINT NOT NULL,
    transaction_block_height BIGINT NOT NULL,
    inserted_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX idx_markets_resolved ON markets(resolved);
CREATE INDEX idx_markets_end_time ON markets(end_time);
CREATE INDEX idx_markets_transaction_version ON markets(transaction_version);

CREATE INDEX idx_bets_market_id ON bets(market_id);
CREATE INDEX idx_bets_user_addr ON bets(user_addr);
CREATE INDEX idx_bets_claimed ON bets(claimed);
CREATE INDEX idx_bets_transaction_version ON bets(transaction_version);

CREATE INDEX idx_market_resolutions_transaction_version ON market_resolutions(transaction_version);

CREATE INDEX idx_winnings_claims_bet_id ON winnings_claims(bet_id);
CREATE INDEX idx_winnings_claims_user_addr ON winnings_claims(user_addr);
CREATE INDEX idx_winnings_claims_transaction_version ON winnings_claims(transaction_version);

CREATE INDEX idx_yield_deposits_market_id ON yield_deposits(market_id);
CREATE INDEX idx_yield_deposits_transaction_version ON yield_deposits(transaction_version);

CREATE INDEX idx_protocol_fees_market_id ON protocol_fees(market_id);
CREATE INDEX idx_protocol_fees_transaction_version ON protocol_fees(transaction_version);