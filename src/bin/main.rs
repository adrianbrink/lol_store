extern crate lol_store;
extern crate serde_json;

use self::lol_store::*;
use self::lol_store::models::*;

fn main() {
    println!("Hello, world!");

    let shards = get_shards();
    let printable_shards = serde_json::to_string_pretty(&shards).unwrap();
    println!("deserialized shards = {}", printable_shards);

    let featured_games = get_featured_games();
    let printable_featured_games = serde_json::to_string_pretty(&featured_games).unwrap();
    println!("deserialized featured_games = {}", printable_featured_games);

    let connection = establish_connection();

    let shard = Shard {
        hostname: "ssss".to_string(),
        name: "ssss".to_string(),
        region_tag: "ssss".to_string(),
        slug: "ssss".to_string(),
    };

    shard.save();

    // let loaded_shards = create_shards(&connection, &shards);
    // println!("{:?}", loaded_shards);

    // let loaded_games = create_games(&connection, &featured_games.game_list);
    // println!("{:?}", loaded_games);
}

// The goal:
// A small server utility that downloads data from the league api and stores it in a postgres database for future analysis.

// The process:
// The program contacts the league api up to the rate limit and can also work with multiple api keys to further increase the limit.
// It starts with the featured games, and then stores all the data in postgres while storing the summoner id's in a queue using
// redis. Then it takes the first summoner id from the queue and pulls all game data for that id. From those games it extracts the
// summoner ids and adds them to the unique queue. Then it runs indefinitely.

// The design:
// RateLimiter {
// - an object that schedules the execution of tasks and limits the throughput
// - takes a job (probably a closure) and executes it at a later point
// - maintains an internal counter and timer
// - only works in memory and nothing is written to disk
// }

// APIClient {
// - contacts the league api and returns a deserialized object
// }

// DBDriver {
// - takes a deserialized object and saves it to the database
// - makes sure that a game isn't inserted more than once into the database
// }

// Scheduler {
// - asks the APIClient to make a request and extracts the summoner ids from the response
// - maintains an internal in-memory queue of summoner ids
// - persist internal queue to redis using the RedisDriver in refills the queue when it runs low
// - takes the next summoner id and makes a request with it using the APIClient
// }

// RedisDriver {
// - saves a summoner id to redis
// - loads a summoner id from redis
// }
//