

diesel::table! {
    items (id) {
        id -> Integer,
        item_name -> Text,
        item_type_id -> Integer,
        acquired_time -> Timestamp,
    }
}

diesel::table! {
    item_types (id) {
        id -> Integer,
        name -> Text,
    }
}



diesel::joinable!(items -> item_types (item_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    item_types,
    items,
);