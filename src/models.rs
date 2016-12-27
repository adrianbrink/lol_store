extern crate serde_json;

use super::schema::shards;
use super::schema::games;

// This struct is used to deserialize to and then save to DB.
#[derive(Insertable)]
#[table_name="shards"]
#[derive(Serialize, Deserialize, Debug)]
pub struct Shard {
    pub hostname: String,
    // pub locales: Vec<String>,
    pub name: String,
    pub region_tag: String,
    pub slug: String,
}

// This struct is used to load from DB.
#[derive(Queryable, Debug)]
pub struct LoadedShard {
    pub id: i32,
    pub hostname: String,
    // pub locales: Vec<String>,
    pub name: String,
    pub region_tag: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeaturedGames {
    #[serde(rename="clientRefreshInterval")]
    pub client_refresh_interval: i64,
    #[serde(rename="gameList")]
    pub game_list: Vec<Game>,
}

#[derive(Insertable)]
#[table_name="games"]
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    //#[serde(rename="bannedChampions")]
    //#pub banned_champions: Vec<BannedChampion>,
    #[serde(rename="gameId")]
    pub game_id: i64,
    #[serde(rename="gameLength")]
    pub game_length: i64,
    #[serde(rename="gameMode")]
    pub game_mode: String,
    #[serde(rename="gameQueueConfigId")]
    pub game_queue_config_id: i64,
    #[serde(rename="gameStartTime")]
    pub game_start_time: i64,
    #[serde(rename="gameType")]
    pub game_type: String,
    #[serde(rename="mapId")]
    pub map_id: i64,
    //#[serde(rename="observers")]
    //#pub observers: Observer,
    //#[serde(rename="participants")]
    //#pub participants: Vec<Participant>,
    #[serde(rename="platformId")]
    pub platform_id: String,
}

#[derive(Queryable)]
pub struct LoadedGame {
    pub id: i32,
    //#[serde(rename="bannedChampions")]
    //#pub banned_champions: Vec<BannedChampion>,
    pub game_id: i64,
    pub game_length: i64,
    pub game_mode: String,
    pub game_queue_config_id: i64,
    pub game_start_time: i64,
    pub game_type: String,
    pub map_id: i64,
    //#[serde(rename="observers")]
    //#pub observers: Observer,
    //#[serde(rename="participants")]
    //#pub participants: Vec<Participant>,
    pub platform_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BannedChampion {
    #[serde(rename="championId")]
    pub champion_id: i64,
    #[serde(rename="pickTurn")]
    pub pick_turn: i64,
    #[serde(rename="teamId")]
    pub team_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Observer {
    #[serde(rename="encryptionKey")]
    pub encryption_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Participant {
    pub bot: bool,
    #[serde(rename="championId")]
    pub champion_id: i64,
    #[serde(rename="profileIconId")]
    pub profile_icon_id: i64,
    #[serde(rename="spell1Id")]
    pub spell_1_id: i64,
    #[serde(rename="spell2Id")]
    pub spell_2_id: i64,
    #[serde(rename="summonerName")]
    pub summoner_name: String,
    #[serde(rename="teamId")]
    pub team_id: i64,
}