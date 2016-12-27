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

    pub fn get_featured_games() -> FeaturedGames {
        let data = request_get_featured_games();
        // println!("{}", data);
        let deserialized_featured_games: FeaturedGames = serde_json::from_str(&data).unwrap();
        deserialized_featured_games
    }

    pub fn get_shards() -> Vec<Shard> {
        let data = request_get_shards();
        let deserialized_shards: Vec<Shard> = serde_json::from_str(&data).unwrap();
        deserialized_shards
    }
}