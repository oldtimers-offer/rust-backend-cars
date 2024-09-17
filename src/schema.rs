// @generated automatically by Diesel CLI.

diesel::table! {
    classic_cars (id) {
        id -> Int4,
        make -> Varchar,
        model -> Varchar,
        year -> Int4,
        mileage -> Int4,
        created_at -> Timestamp,
    }
}
