table! {
    accounts (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    blocks (id) {
        id -> Int4,
        creation_time -> Nullable<Timestamp>,
        chain_id -> Nullable<Numeric>,
    }
}

table! {
    transactions (id) {
        id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    blocks,
    transactions,
);
