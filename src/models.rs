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
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shard {
    pub region_tag: String,

    //#[serde(skip_serializing)]
    //pub locales: String,

    //#[serde(skip_serializing)]
    pub name: String,

    //#[serde(skip_serializing)]
    //pub hostname: String,

    //#[serde(skip_serializing)]
    //pub slug: String,
}
