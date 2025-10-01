// @generated automatically by Diesel CLI.

diesel::table! {
    bets (bet_id) {
        bet_id -> Int8,
        market_id -> Int8,
        #[max_length = 66]
        user_addr -> Varchar,
        position -> Bool,
        amount -> Int8,
        transaction_version -> Int8,
        transaction_block_height -> Int8,
        inserted_at -> Timestamp,
        claimed -> Nullable<Bool>,
        winning_amount -> Nullable<Int8>,
        yield_share -> Nullable<Int8>,
        claim_transaction_version -> Nullable<Int8>,
    }
}

diesel::table! {
    bets_extended (id) {
        id -> Text,
        blockchainBetId -> Int8,
        userId -> Text,
        marketId -> Nullable<Text>,
        position -> Nullable<Bool>,
        amount -> Nullable<Numeric>,
        odds -> Numeric,
        status -> Text,
        payout -> Nullable<Numeric>,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    blockchain_events (id) {
        id -> Text,
        eventType -> Text,
        blockchainId -> Text,
        blockNumber -> Int8,
        blockTimestamp -> Int8,
        transactionHash -> Text,
        processed -> Bool,
        data -> Nullable<Text>,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    event_processing_log (id) {
        id -> Int4,
        #[max_length = 100]
        event_type -> Varchar,
        event_data -> Jsonb,
        transaction_version -> Nullable<Int8>,
        processed_at -> Nullable<Timestamp>,
        #[max_length = 50]
        processing_status -> Nullable<Varchar>,
        error_message -> Nullable<Text>,
        processing_duration_ms -> Nullable<Int4>,
    }
}

diesel::table! {
    fee_records (id) {
        id -> Text,
        marketId -> Nullable<Text>,
        feeType -> Text,
        amount -> Numeric,
        source -> Text,
        createdAt -> Timestamp,
    }
}

diesel::table! {
    indexer_state (indexer_name) {
        #[max_length = 255]
        indexer_name -> Varchar,
        last_processed_version -> Int8,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    market_resolutions (market_id) {
        market_id -> Int8,
        outcome -> Bool,
        total_yield_earned -> Int8,
        transaction_version -> Int8,
        transaction_block_height -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    markets (market_id) {
        market_id -> Int8,
        question -> Text,
        end_time -> Int8,
        #[max_length = 66]
        yield_protocol_addr -> Varchar,
        transaction_version -> Int8,
        transaction_block_height -> Int8,
        inserted_at -> Timestamp,
        resolved -> Nullable<Bool>,
        outcome -> Nullable<Bool>,
        total_yield_earned -> Nullable<Int8>,
        resolution_transaction_version -> Nullable<Int8>,
    }
}

diesel::table! {
    markets_extended (id) {
        id -> Text,
        blockchainMarketId -> Nullable<Int8>,
        marketId -> Nullable<Text>,
        adjTicker -> Nullable<Text>,
        platform -> Text,
        question -> Nullable<Text>,
        description -> Nullable<Text>,
        rules -> Nullable<Text>,
        status -> Text,
        probability -> Int4,
        volume -> Numeric,
        openInterest -> Numeric,
        endDate -> Timestamp,
        resolutionDate -> Nullable<Timestamp>,
        result -> Nullable<Bool>,
        link -> Nullable<Text>,
        imageUrl -> Nullable<Text>,
        totalPoolSize -> Numeric,
        yesPoolSize -> Numeric,
        noPoolSize -> Numeric,
        countYes -> Int4,
        countNo -> Int4,
        currentYield -> Numeric,
        totalYieldEarned -> Numeric,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        adj_market_id -> Nullable<Text>,
    }
}

diesel::table! {
    protocol_fees (fee_id) {
        fee_id -> Int8,
        market_id -> Int8,
        fee_amount -> Int8,
        transaction_version -> Int8,
        transaction_block_height -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    protocols (id) {
        id -> Text,
        name -> Text,
        displayName -> Text,
        baseApy -> Numeric,
        isActive -> Bool,
        description -> Nullable<Text>,
        iconUrl -> Nullable<Text>,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    sync_status (id) {
        id -> Text,
        eventType -> Text,
        lastSyncBlock -> Int8,
        lastSyncTime -> Timestamp,
        isActive -> Bool,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        address -> Text,
        email -> Nullable<Text>,
        username -> Nullable<Text>,
        avatarUrl -> Nullable<Text>,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    winnings_claims (claim_id) {
        claim_id -> Int8,
        bet_id -> Int8,
        #[max_length = 66]
        user_addr -> Varchar,
        winning_amount -> Int8,
        yield_share -> Int8,
        transaction_version -> Int8,
        transaction_block_height -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    yield_deposits (deposit_id) {
        deposit_id -> Int8,
        market_id -> Int8,
        amount -> Int8,
        #[max_length = 66]
        protocol_addr -> Varchar,
        transaction_version -> Int8,
        transaction_block_height -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    yield_records (id) {
        id -> Text,
        marketId -> Text,
        protocolId -> Text,
        amount -> Numeric,
        apy -> Numeric,
        #[sql_name = "yield"]
        yield_ -> Numeric,
        period -> Timestamp,
        createdAt -> Timestamp,
    }
}

diesel::joinable!(bets -> markets (market_id));
diesel::joinable!(bets_extended -> users (userId));
diesel::joinable!(market_resolutions -> markets (market_id));
diesel::joinable!(protocol_fees -> markets (market_id));
diesel::joinable!(winnings_claims -> bets (bet_id));
diesel::joinable!(yield_deposits -> markets (market_id));
diesel::joinable!(yield_records -> protocols (protocolId));

diesel::allow_tables_to_appear_in_same_query!(
    bets,
    bets_extended,
    blockchain_events,
    event_processing_log,
    fee_records,
    indexer_state,
    market_resolutions,
    markets,
    markets_extended,
    protocol_fees,
    protocols,
    sync_status,
    users,
    winnings_claims,
    yield_deposits,
    yield_records,
);
