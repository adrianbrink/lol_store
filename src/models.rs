extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Shard {
    pub hostname: String,
    pub locales: Vec<String>,
    pub name: String,
    pub region_tag: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeaturedGames {
    #[serde(skip_serializing)]
    #[serde(rename = "gameList")]
    pub game_list: Vec<FeaturedGameInfo>,
    #[serde(rename = "clientRefreshInterval")]
    pub client_refresh_interval: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeaturedGameInfo {
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