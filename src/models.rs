#![feature(proc_macro)]

extern crate serde_json;

use super::schema::games;

#[derive(Queryable)]
pub struct Game {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Insertable)]
#[table_name="games"]
pub struct NewGame {
    pub name: String,
}
