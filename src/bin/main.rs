extern crate lol_store;
extern crate serde_json;

use self::lol_store::*;

// TODO
// - write all the commented out code into a test for deserialization of Shards
// - write manual deserialization for FeaturedGames
// - move the 2nd point into a test
fn main() {
    println!("Hello, world!");

    let shards = get_shards();
    let printable_shards = serde_json::to_string_pretty(&shards).unwrap();
    println!("deserialized shards = {}", printable_shards);

    let featured_games = get_featured_games();
    let printable_featured_games = serde_json::to_string_pretty(&featured_games).unwrap();
    println!("deserialized featured_games = {}", printable_featured_games);

    let connection = establish_connection();

    let loaded_shards = create_shards(&connection, &shards);
    println!("{:?}", loaded_shards);

    let loaded_games = create_games(&connection, &featured_games.game_list);
    println!("{:?}", loaded_games);
}
