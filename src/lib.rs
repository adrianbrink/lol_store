#![feature(proc_macro)]

pub mod client {
    extern crate dotenv;
    extern crate hyper;
    extern crate serde_json;

    use dotenv::dotenv;
    use std::env;
    use hyper::Client;
    use std::io::Read;
    use models::{Shard, FeaturedGames};

    // TODO
    // - refactor to group this into a struct
    // - struct should contain a list of api_keys
    // - struct should contain an object to do rate limiting, should be injected
    // - hyper client should be injected into struct
    // - then write tests for this struct since I can mock things
    // - return Option/Result instead of force unwrapping
    fn request_get_featured_games() -> String {
        dotenv().ok();
        let api_key = env::var("RIOT_API_KEY").expect("RIOT api key should be set.");
        let request_url = format!("https://euw.api.pvp.net/observer-mode/rest/featured?api_key={}",
                                  api_key);
        let client = Client::new();
        let mut req = client.get(&request_url).send().expect("API call failed.");
        let mut res = String::new();
        let _ = req.read_to_string(&mut res);
        res
    }

    fn request_get_shards() -> String {
        let client = Client::new();
        let mut req = client.get("http://status.leagueoflegends.com/shards")
            .send()
            .expect("API call failed.");
        let mut res = String::new();
        let _ = req.read_to_string(&mut res);
        res
    }

    pub fn get_featured_games() -> Vec<FeaturedGames> {
        let data = request_get_featured_games();
        println!("{}", data);
        let deserialized_featured_games: Vec<FeaturedGames> = serde_json::from_str(&data).unwrap();
        deserialized_featured_games
    }

    pub fn get_shards() -> Vec<Shard> {
        let data = request_get_shards();
        let deserialized_shards: Vec<Shard> = serde_json::from_str(&data).unwrap();
        deserialized_shards
    }
}


// This is old.
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
extern crate hyper;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Game, NewGame};

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, name: String, id: i32) -> Game {
    use schema::games;

    let new_game = NewGame {
        name: name,
        id: id,
    };

    diesel::insert(&new_game)
        .into(games::table)
        .get_result(conn)
        .expect("Error saving new game.")
}

// pub fn request_data() -> String {
//     dotenv().ok();
//     let api_key = env::var("RIOT_API_KEY").expect("RIOT api key should be set.");

//     let client = Client::new();
//     // let mut res = client.get("https://euw.api.pvp.net/observer-mode/rest/featured?api_key=RGAPI-11577715-A924-4825-A831-FF7038985625").send().unwrap();
//     let mut res = client.get("http://status.leagueoflegends.com/shards").send().unwrap();
//     let mut response = String::new();
//     res.read_to_string(&mut response);
//     println!("{:?}", response);
//     assert_eq!(res.status, hyper::Ok);
//     response
// }
