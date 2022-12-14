use diesel::prelude::*;
use diesel::table;

table! {
    messages (id) {
        id -> Int4,
        username -> Varchar,
        message -> Text,
        timestamp -> Int8,
    }
}