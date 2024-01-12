// @generated automatically by Diesel CLI.

diesel::table! {
    quotes (id) {
        id -> Int4,
        uuid -> Varchar,
        quote -> Varchar,
        author -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
