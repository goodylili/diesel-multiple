-- Your SQL goes here
-- Table definition for items
CREATE TABLE IF NOT EXISTS items
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    item_name     TEXT     NOT NULL,
    item_type_id  INTEGER  NOT NULL,
    acquired_time DATETIME NOT NULL,
    FOREIGN KEY (item_type_id) REFERENCES item_types (id)
);