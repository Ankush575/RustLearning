// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        age -> Integer,
        #[max_length = 255]
        gender -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}
