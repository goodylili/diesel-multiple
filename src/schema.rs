extern crate diesel;


// @generated automatically by Diesel CLI.
#[macro_use]

diesel::table! {
    item_types (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    items (id) {
        id -> Nullable<Integer>,
        item_name -> Text,
        item_type_id -> Integer,
        acquired_time -> Timestamp,
    }
}

diesel::joinable!(items -> item_types (item_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    item_types,
    items,
);
