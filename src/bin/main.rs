extern crate lol_store;
extern crate diesel;
extern crate serde_json;

use self::lol_store::client::*;

// TODO
// - write all the commented out code into a test for deserialization of Shards
// - write manual deserialization for FeaturedGames
// - move the 2nd point into a test
fn main() {
    println!("Hello, world!");

    let shards = get_shards();
    // println!("deserialized shards = {:?}", shards);

    let featured_games = get_featured_games();
    println!("deserialized featured_games = {:?}", featured_games);
}
