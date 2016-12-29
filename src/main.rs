extern crate lol_store;
extern crate serde_json;

// use self::lol_store::*;
// use self::lol_store::models::*;
use self::lol_store::league_api::*;
use lol_store::redis_connection::{RedisConnector, UniqueQueue};


fn main() {
    println!("Hello, world!");

    let redis = RedisConnector::new().unwrap();
    let queue = UniqueQueue::new(redis.connection, "summoner_id".to_string());
    let x = queue.push("2".to_string());
    println!("{}", x);

    // save_game_id(33333);
    // let game_id = retrieve_game_id();
    // println!("{}", game_id);

    let api_client = APIClient::new().unwrap();
    let shardy = api_client.get_shards();
    let printable_shardy = serde_json::to_string_pretty(&shardy).unwrap();
    println!("shardy = {}", printable_shardy);

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
}
