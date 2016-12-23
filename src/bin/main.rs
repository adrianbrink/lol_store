extern crate lol_store;
extern crate diesel;

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
}
