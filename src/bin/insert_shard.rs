extern crate lol_store;
extern crate diesel;

use self::lol_store::*;
use self::lol_store::models::*;

fn main() {
    let connection = establish_connection();

    let shard = Shard {
        hostname: "test".to_string(),
        name: "test".to_string(),
        region_tag: "test".to_string(),
        slug: "test".to_string(),
    };

    let loaded_shard = create_shard(&connection, &shard);

    println!("{:?}", loaded_shard);
}