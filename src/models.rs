extern crate serde_json;

use super::schema::matchdetails;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result;
use diesel;

#[derive(Debug, Deserialize, Insertable, Serialize, Queryable)]
#[table_name="matchdetails"]
pub struct MatchDetail {
    #[serde(rename="matchId")]
    pub match_id: i64,
    #[serde(rename="matchDuration")]
    pub match_duration: i64,
}

impl MatchDetail {
    pub fn save(self, conn: &PgConnection) -> Result<MatchDetail, result::Error> {
        diesel::insert(&self).into(matchdetails::table).get_result(conn)
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct FeaturedGames {
    #[serde(rename="clientRefreshInterval")]
    pub client_refresh_interval: i64,
    #[serde(rename="gameList")]
    pub game_list: Vec<FeaturedGameInfo>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct FeaturedGameInfo {
    pub participants: Vec<Participant>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Participant {
    #[serde(rename="summonerName")]
    pub summoner_name: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Summoner {
    pub id: i64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct MatchList {
    pub matches: Vec<MatchReference>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct MatchReference {
    #[serde(rename="matchId")]
    pub match_id: i64,
}


// This is old - deprecated

// use super::schema::shards;
// use super::schema::games;
// use super::*;

// #[table_name="games"]
// #[derive(Serialize, Deserialize, Debug, Insertable)]
// pub struct Game {
//     //#[serde(rename="bannedChampions")]
//     //#pub banned_champions: Vec<BannedChampion>,
//     #[serde(rename="gameId")]
//     pub game_id: i64,
//     #[serde(rename="gameLength")]
//     pub game_length: i64,
//     #[serde(rename="gameMode")]
//     pub game_mode: String,
//     #[serde(rename="gameQueueConfigId")]
//     pub game_queue_config_id: i64,
//     #[serde(rename="gameStartTime")]
//     pub game_start_time: i64,
//     #[serde(rename="gameType")]
//     pub game_type: String,
//     #[serde(rename="mapId")]
//     pub map_id: i64,
//     //#[serde(rename="observers")]
//     //#pub observers: Observer,
//     //#[serde(rename="participants")]
//     //#pub participants: Vec<Participant>,
//     #[serde(rename="platformId")]
//     pub platform_id: String,
// }

// #[derive(Queryable, Debug)]
// pub struct LoadedGame {
//     pub id: i32,
//     //#[serde(rename="bannedChampions")]
//     //#pub banned_champions: Vec<BannedChampion>,
//     pub game_id: i64,
//     pub game_length: i64,
//     pub game_mode: String,
//     pub game_queue_config_id: i64,
//     pub game_start_time: i64,
//     pub game_type: String,
//     pub map_id: i64,
//     //#[serde(rename="observers")]
//     //#pub observers: Observer,
//     //#[serde(rename="participants")]
//     //#pub participants: Vec<Participant>,
//     pub platform_id: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct BannedChampion {
//     #[serde(rename="championId")]
//     pub champion_id: i64,
//     #[serde(rename="pickTurn")]
//     pub pick_turn: i64,
//     #[serde(rename="teamId")]
//     pub team_id: i64,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Observer {
//     #[serde(rename="encryptionKey")]
//     pub encryption_key: String,
// }

// // This struct is used to deserialize to and then save to DB.
// #[table_name="shards"]
// #[derive(Serialize, Deserialize, Debug, Insertable)]
// pub struct Shard {
//     pub hostname: String,
//     // pub locales: Vec<String>,
//     pub name: String,
//     pub region_tag: String,
//     pub slug: String,
// }

// impl Shard {
//     pub fn save(&self) {
//         let conn = establish_connection();
//         let _ = diesel::insert(self).into(shards::table).execute(&conn);
//     }
// }

// // This struct is used to load from DB.
// #[derive(Queryable, Debug)]
// pub struct LoadedShard {
//     pub id: i32,
//     pub hostname: String,
//     // pub locales: Vec<String>,
//     pub name: String,
//     pub region_tag: String,
//     pub slug: String,
// }