#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::dsl::now;
use diesel::sql_types::Timestamp;

mod database;
mod schema;

use crate::database::establish_connection;

// Import the table and schema macros.
use crate::schema::{item_types, items};

#[derive(Insertable)]
#[table_name = "item_types"]
struct NewItemType<'a> {
    name: &'a str,
}

#[derive(Insertable)]
#[table_name = "items"]
struct NewItem<'a> {
    item_name: &'a str,
    item_type_id: i32,
    acquired_time: NaiveDateTime,
}

fn main() {
    let mut connection = establish_connection();
    let new_item = NewItem {
        item_name: "Book",
        item_type_id: 1,
        acquired_time: Utc::now().naive_utc(),
    };

    diesel::insert_into(items::table)
        .values(&new_item)
        .execute(&mut connection)
        .expect("Error saving new item");

    let new_item_type = NewItemType { name: "Education" };
    diesel::insert_into(item_types::table)
        .values(&new_item_type)
        .execute(&mut connection)
        .expect("Error saving new item type");
}
