extern crate lol_store;

use self::lol_store::postgres_connection::PostgresConnector;
use self::lol_store::redis_connection::RedisConnector;
use self::lol_store::unique_redis_queue::UniqueQueue;
use self::lol_store::models::MatchDetail;

fn main() {
    println!("Hello, world!");

    // TODO - this should be handled with .map_err(), but for that PostgresConnector needs to implement std::fmt::Display.
    let postgres_connector = PostgresConnector::new().expect("Postgres connection failed.");
    let postgres_connection = postgres_connector.get_connection();

    // TODO - this should be handled with .map_err(), but for that RedisConnector needs to implement std::fmt::Display.
    let redis_connector = RedisConnector::new().expect("Redis connection failed.");
    let redis_connection = redis_connector.get_connection();

    let summoner_queue = UniqueQueue::new(&redis_connection, "summoner".to_string());

    loop {
        if summoner_queue.is_empty() {
            println!("There are no summoners and hence we will fetch the featured games to get a \
                      seed of summoner ids.");

        }

    }
}


// extern crate serde_json;

// use self::lol_store::*;
// use self::lol_store::league_api::*;
// use lol_store::redis_connection::{RedisConnector, UniqueQueue};

// let api_client = APIClient::new().unwrap();

// let match_ids = api_client.get_match_ids("19861577".to_string());
// println!("{:?}", match_ids);

// let featured_games = api_client.get_featured_games();
// let mut summoners_1 = featured_games.game_list
//     .into_iter()
//     .flat_map(|featured_game| featured_game.participants)
//     .collect::<Vec<Participant>>();
// let length = summoners_1.len();
// let summoners_2 = summoners_1.split_off(length / 2);

// let mut summoner_names_1 = api_client.get_summoner_names(summoners_1);
// let mut summoner_names_2 = api_client.get_summoner_names(summoners_2);

// let mut summoner_ids_1 = api_client.get_summoner_ids(summoner_names_1);
// let mut summoner_ids_2 = api_client.get_summoner_ids(summoner_names_2);
// summoner_ids_1.append(&mut summoner_ids_2);

// let redis_connector = RedisConnector::new().unwrap();
// let unique_queue = UniqueQueue::new(redis_connector.connection, "summoners".to_string());

// let results = summoner_ids_1.into_iter().map(|id| unique_queue.push(id)).collect::<Vec<i32>>();

// let names = vec!["n3wk1d".to_string(), "awacatization".to_string()];
// let result = api_client.get_summoner_ids(names);

// // let featured_games = api_client.request_get_summoner_ids(names);
// println!("{:?}", results);

// let redis = RedisConnector::new().unwrap();
// let queue = UniqueQueue::new(redis.connection, "summoner_id".to_string());
// let x = queue.push("2".to_string());
// println!("{}", x);

// save_game_id(33333);
// let game_id = retrieve_game_id();
// println!("{}", game_id);

// let api_client = APIClient::new().unwrap();
// let shardy = api_client.get_shards();
// let printable_shardy = serde_json::to_string_pretty(&shardy).unwrap();
// println!("shardy = {}", printable_shardy);

// let shards = get_shards();
// let printable_shards = serde_json::to_string_pretty(&shards).unwrap();
// println!("deserialized shards = {}", printable_shards);

// let featured_games = get_featured_games();
// let printable_featured_games = serde_json::to_string_pretty(&featured_games).unwrap();
// println!("deserialized featured_games = {}", printable_featured_games);

// let shard = Shard {
//     hostname: "ssss".to_string(),
//     name: "ssss".to_string(),
//     region_tag: "ssss".to_string(),
//     slug: "ssss".to_string(),
// };

// shard.save();

// let loaded_shards = create_shards(&connection, &shards);
// println!("{:?}", loaded_shards);

// let loaded_games = create_games(&connection, &featured_games.game_list);
// println!("{:?}", loaded_games);
