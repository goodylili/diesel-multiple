#![feature(proc_macro_hygiene, decl_macro)]
extern crate diesel;

mod schema;
mod database;

use crate::database::establish_connection;
use diesel::prelude::*;
use diesel::sql_query;


#[derive(Debug, QueryableByName)]
#[diesel(table_name = schema::item_types)]
struct ItemType {
    id: i32,
    name: String,
}

#[derive(Debug, QueryableByName)]
#[diesel(table_name = schema::items)]
struct Item {
    id: i32,
    item_name: String,
    acquired_time: chrono::NaiveDateTime,
    // Assuming you use chrono for datetime fields
    item_type_id: i32,
}


fn main() {
    let mut connection = establish_connection();

    let item_type_id_to_retrieve = 1;

    // Using raw SQL to retrieve the ItemType
    let item_type: ItemType = sql_query("SELECT * FROM item_types WHERE id = $1")
        .bind::<diesel::sql_types::Integer, _>(item_type_id_to_retrieve)
        .get_result(&mut connection)
        .expect("Failed to find item type by ID");

    // Using raw SQL to retrieve items of the specified item type
    let items_of_type: Vec<Item> = sql_query("SELECT * FROM items WHERE item_type_id = $1")
        .bind::<diesel::sql_types::Integer, _>(item_type.id)
        .load(&mut connection)
        .expect("Failed to load items of item type");

    println!("Items of type '{}':", item_type.name);
    for item in items_of_type {
        println!(
            "Item ID: {}, Name: {}, Acquired Time: {:?}",
            item.id, item.item_name, item.acquired_time
        );
    }
}
