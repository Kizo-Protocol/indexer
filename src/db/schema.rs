// @generated automatically by Diesel CLI.

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
    protocol_fees (fee_id) {
        fee_id -> Int8,
        market_id -> Int8,
        fee_amount -> Int8,
        transaction_version -> Int8,
        transaction_block_height -> Int8,
        inserted_at -> Timestamp,
    }
}

diesel::joinable!(bets -> markets (market_id));
diesel::joinable!(market_resolutions -> markets (market_id));
diesel::joinable!(winnings_claims -> bets (bet_id));
diesel::joinable!(yield_deposits -> markets (market_id));
diesel::joinable!(protocol_fees -> markets (market_id));

diesel::allow_tables_to_appear_in_same_query!(
    markets,
    bets,
    market_resolutions,
    winnings_claims,
    yield_deposits,
    protocol_fees,
);
