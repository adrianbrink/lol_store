// extern crate lol_store;
// extern crate diesel;

// use self::lol_store::*;
// use self::lol_store::models::*;
// use self::diesel::prelude::*;

// fn main() {
//     use lol_store::schema::shards::dsl::*;

//     let connection = establish_connection();
//     let results = shards.limit(5)
//         .load::<LoadedShard>(&connection)
//         .expect("Error loading shards");

//     println!("Displaying {} shards", results.len());
//     for shard in results {
//         println!("{}", shard.hostname);
//         println!("----------\n");
//         println!("{}", shard.slug);
//     }
// }