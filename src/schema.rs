// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        hashed_password -> Varchar,
        #[max_length = 50]
        role -> Varchar,
        created_at -> Timestamp,
    }
}
