extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Shard {
    pub region_tag: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeaturedGames {
    //#[serde(rename = "clientRefreshInterval")]
    pub clientRefreshInterval: u64,
}


// This is old.
use super::schema::games;

#[derive(Queryable)]
pub struct Game {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Insertable)]
#[table_name="games"]
pub struct NewGame {
    pub name: String,
    pub id: i32,
}