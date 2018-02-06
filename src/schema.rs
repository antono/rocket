table! {
    users (user_id) {
        user_id -> Int4,
        email -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
