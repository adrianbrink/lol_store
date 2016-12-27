extern crate dotenv;
extern crate hyper;
extern crate serde_json;

use hyper::Client;
use dotenv::dotenv;
use std::env;
use std::io::Read;
use super::models::{FeaturedGames, Shard};

pub struct APIClient {
    api_key: String,
    hyper_client: Client, // base_url: String,
}

impl APIClient {
    pub fn new() -> Option<APIClient> {
        dotenv().ok();
        let api_key = env::var("RIOT_API_KEY");
        match api_key {
            Ok(val) => {
                let api_client = APIClient {
                    api_key: val,
                    hyper_client: Client::new(),
                };
                Some(api_client)
            }
            Err(_) => None,
        }
    }

    fn request_get_featured_games(&self) -> String {
        let request_url = format!("https://euw.api.pvp.net/observer-mode/rest/featured?api_key={}",
                                  self.api_key);
        let mut req = self.hyper_client.get(&request_url).send().expect("API call failed.");
        let mut res = String::new();
        let _ = req.read_to_string(&mut res);
        res
    }

    fn request_get_shards(&self) -> String {
        let mut req = self.hyper_client
            .get("http://status.leagueoflegends.com/shards")
            .send()
            .expect("API call failed.");
        let mut res = String::new();
        let _ = req.read_to_string(&mut res);
        res
    }

    pub fn get_featured_games(&self) -> FeaturedGames {
        let data = self.request_get_featured_games();
        let deserialized_featured_games: FeaturedGames = serde_json::from_str(&data).unwrap();
        deserialized_featured_games
    }

    pub fn get_shards(&self) -> Vec<Shard> {
        let data = self.request_get_shards();
        let deserialized_shards: Vec<Shard> = serde_json::from_str(&data).unwrap();
        deserialized_shards
    }
}
