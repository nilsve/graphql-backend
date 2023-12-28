// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        body -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}
