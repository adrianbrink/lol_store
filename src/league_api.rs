extern crate serde_json;

use hyper::Client;
use dotenv::dotenv;
use std::env;
use super::models::{FeaturedGames, Participant, Summoner};
use std::io::Read;
use std::collections::BTreeMap;

pub struct APIClient {
    api_key: String,
    hyper_client: Client, // base_url: String,
}

// Move the exposed methods into a trait and then implement the trait. This way one can pass
// a testable API client that just implements the interface.
impl APIClient {
    pub fn new() -> Result<APIClient, env::VarError> {
        dotenv().ok();
        let api_key = env::var("RIOT_API_KEY")?;
        Ok(APIClient {
            api_key: api_key,
            hyper_client: Client::new(),
        })
    }

    // TODO - make this cache the result and discard the cache after the client_refresh_interval
    pub fn get_summoner_seed(&self) -> Vec<i64> {
        // This returns a vector of summoner ids from the featured games.
        let request_data = self.request_get_featured_games();
        let featured_games: FeaturedGames = serde_json::from_str(&request_data).unwrap();
        let summoner_names = featured_games.game_list
            .into_iter()
            .flat_map(|featured_game_info| featured_game_info.participants)
            .map(|participant| participant.summoner_name)
            .collect::<Vec<String>>();
        self.get_summoner_ids(summoner_names)
    }

    pub fn get_matchlist(&self, summoner_id: i64) -> Vec<i64> {
        // This returns a vector of match ids for the given summoner.
        vec![2995897557]

    }

    fn request_get_featured_games(&self) -> String {
        let request_url = format!("https://euw.api.pvp.net/observer-mode/rest/featured?api_key={}",
                                  self.api_key);
        let mut req = self.hyper_client.get(&request_url).send().expect("API call failed.");
        let mut res = String::new();
        let _ = req.read_to_string(&mut res);
        res
    }

    fn get_summoner_ids(&self, summoner_names: Vec<String>) -> Vec<i64> {
        let summoner_names_by_40: Vec<Vec<String>> =
            summoner_names.chunks(40).map(|chunk| chunk.to_owned()).collect();
        let request_urls = summoner_names_by_40.into_iter()
            .map(|chunk| self.create_request_url_for_get_summoner_ids(chunk))
            .collect::<Vec<String>>();
        request_urls.iter()
            .map(|url| self.request_get_summoner_ids(url))
            .flat_map(|x| x)
            .collect::<Vec<i64>>()
    }

    fn request_get_summoner_ids(&self, request_url: &String) -> Vec<i64> {
        let mut req = self.hyper_client.get(request_url).send().expect("API call failed.");
        let mut res = String::new();
        let _ = req.read_to_string(&mut res);
        let summoners: BTreeMap<String, Summoner> = serde_json::from_str(&res).unwrap();
        summoners.into_iter().map(|(_, summoner)| summoner.id).collect::<Vec<i64>>()
    }

    fn create_request_url_for_get_summoner_ids(&self, summoner_names: Vec<String>) -> String {
        let names = summoner_names.join(",");
        format!("https://euw.api.pvp.net/api/lol/euw/v1.4/summoner/by-name/{}?api_key={}",
                names,
                self.api_key)
    }

    // fn get_summoner_ids(&self, summoner_names: Vec<String>) -> Vec<i64> {
    //     let data = self.request_get_summoner_ids(summoner_names);
    //     let deserialized_summoner_ids: BTreeMap<String, Summoner> = serde_json::from_str(&data)
    //         .unwrap();
    //     deserialized_summoner_ids.into_iter()
    //         .map(|(_, summoner)| summoner.id)
    //         .collect::<Vec<i64>>()
    // }

    // pub fn request_get_matchlist(&self, summoner_id: String) -> String {
    //     let request_url = format!("https://euw.api.pvp.net/api/lol/euw/v2.\
    //                                2/matchlist/by-summoner/{}?rankedQueues=RANKED_FLEX_SR,\
    //                                RANKED_SOLO_5x5,RANKED_TEAM_3x3,RANKED_TEAM_5x5,\
    //                                TEAM_BUILDER_DRAFT_RANKED_5x5,\
    //                                TEAM_BUILDER_RANKED_SOLO&seasons=SEASON2016&api_key={}",
    //                               summoner_id,
    //                               self.api_key);
    //     let mut req = self.hyper_client.get(&request_url).send().expect("API call failed.");
    //     let mut res = String::new();
    //     let _ = req.read_to_string(&mut res);
    //     res
    // }



    // pub fn request_get_shards(&self) -> String {
    //     let mut req = self.hyper_client
    //         .get("http://status.leagueoflegends.com/shards")
    //         .send()
    //         .expect("API call failed.");
    //     let mut res = String::new();
    //     let _ = req.read_to_string(&mut res);
    //     res
    // }

    // pub fn get_match_ids(&self, summoner_id: String) -> Vec<i64> {
    //     let data = self.request_get_matchlist(summoner_id);
    //     let deserialized_summoner_ids: MatchList = serde_json::from_str(&data).unwrap();
    //     deserialized_summoner_ids.matches
    //         .into_iter()
    //         .map(|match_reference| match_reference.match_id)
    //         .collect::<Vec<i64>>()
    // }

    // pub fn get_summoner_names(&self, participants: Vec<Participant>) -> Vec<String> {
    //     participants.into_iter()
    //         .map(|participant| participant.summoner_name)
    //         .collect()
    // }



    // pub fn get_featured_games(&self) -> FeaturedGames {
    //     let data = self.request_get_featured_games();
    //     let deserialized_featured_games: FeaturedGames = serde_json::from_str(&data).unwrap();
    //     deserialized_featured_games
    // }

    // pub fn get_shards(&self) -> Vec<Shard> {
    //     let data = self.request_get_shards();
    //     let deserialized_shards: Vec<Shard> = serde_json::from_str(&data).unwrap();
    //     deserialized_shards
    // }
}