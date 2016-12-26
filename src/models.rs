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
    #[serde(rename="clientRefreshInterval")]
    pub client_refresh_interval: u64,
    #[serde(rename="gameList")]
    pub game_list: Vec<FeaturedGameInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeaturedGameInfo {
    #[serde(rename="bannedChampions")]
    pub banned_champions: Vec<BannedChampion>,
    #[serde(rename="gameId")]
    pub game_id: u64,
    #[serde(rename="gameLength")]
    pub game_lenght: u64,
    #[serde(rename="gameMode")]
    pub game_mode: String,
    #[serde(rename="gameQueueConfigId")]
    pub game_queue_config_id: u64,
    #[serde(rename="gameStartTime")]
    pub game_start_time: u64,
    #[serde(rename="gameType")]
    pub game_type: String,
    #[serde(rename="mapId")]
    pub map_id: u64,
    #[serde(rename="observers")]
    pub observers: Observer,
    #[serde(rename="participants")]
    pub participants: Vec<Participant>,
    #[serde(rename="platformId")]
    pub platform_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BannedChampion {
    #[serde(rename="championId")]
    pub champion_id: u64,
    #[serde(rename="pickTurn")]
    pub pick_turn: i32,
    #[serde(rename="teamId")]
    pub team_id: u64,
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
    pub champion_id: u64,
    #[serde(rename="profileIconId")]
    pub profile_icon_id: u64,
    #[serde(rename="spell1Id")]
    pub spell_1_id: u64,
    #[serde(rename="spell2Id")]
    pub spell_2_id: u64,
    #[serde(rename="summonerName")]
    pub summoner_name: String,
    #[serde(rename="teamId")]
    pub team_id: u64,
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