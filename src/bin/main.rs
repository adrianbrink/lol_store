extern crate lol_store;
extern crate diesel;
extern crate serde_json;

use self::lol_store::*;
use self::lol_store::models::*;
use self::diesel::prelude::*;


fn main() {
    println!("Hello, world!");

    use lol_store::schema::games::dsl::*;

    let connection = establish_connection();
    let results = games.load::<Game>(&connection)
        .expect("Error loading games");

    println!("Displaying {} games", results.len());
    for game in results {
        println!("{}", game.name);
        println!("----------\n");
    }

    let new_game = NewGame { name: "Adrian".to_string() };
    println!("{:?}", new_game);

    let serialized = serde_json::to_string(&new_game).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: NewGame = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    let request_data = request_data();
    
}
