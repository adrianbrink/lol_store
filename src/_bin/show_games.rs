// extern crate lol_store;
// extern crate diesel;

// use self::lol_store::*;
// use self::lol_store::models::*;
// use self::diesel::prelude::*;

// fn main() {
//     use lol_store::schema::games::dsl::*;

//     let connection = establish_connection();
//     let results = games.limit(5)
//         .load::<LoadedGame>(&connection)
//         .expect("Error loading games");

//     println!("Displaying {} games", results.len());
//     for game in results {
//         println!("{}", game.game_id);
//         println!("----------\n");
//         println!("{}", game.game_mode);
//     }
// }