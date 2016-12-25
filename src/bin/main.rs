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

    let new_game = NewGame {
        name: "Adrian".to_string(),
        id: 23,
    };
    let new_game2 = NewGame {
        name: "Awa".to_string(),
        id: 23,
    };

    let games_to_convert = vec![new_game, new_game2];
    //println!("{:?}", new_game);

    let serialized = serde_json::to_string(&games_to_convert).unwrap();
    //let serialized = "{\"name\":\"Adrian\",\"id\":23}";
    println!("serialized = {}", serialized);

    let deserialized: Vec<NewGame> = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    let api_data = "[{\"region_tag\":\"EUW\",\"name\":\"test\"},{\"region_tag\":\"NA\",\"name\":\"test1\"}]";
    
    let deserialized_data: Vec<Shard> = serde_json::from_str(&api_data).unwrap();
    println!("deserialized_data = {:?}", deserialized_data);

    let fake_shard = "[{\"name\":\"North America\",\"slug\":\"na\",\"locales\":[\"en_US\"],\"hostname\":\"prod.na1.lol.riotgames.com\",\"region_tag\":\"na1\"}, {\"name\":\"Europe\",\"slug\":\"na\",\"locales\":[\"en_US\"],\"hostname\":\"prod.na1.lol.riotgames.com\",\"region_tag\":\"euw\"}]";
    let deserialized_fake_shard: Vec<Shard> = serde_json::from_str(&fake_shard).unwrap();
    println!("deserialized_fake_shard = {:?}", deserialized_fake_shard[0]);

    let request_data_1 = request_data();
    //println!("{}", &request_data_1);
    let deserialized_real_shard: Vec<Shard> = serde_json::from_str(&request_data_1).unwrap();
    let serialized_real_shard = serde_json::to_string_pretty(&deserialized_real_shard[0]).unwrap();
    println!("serialized = {}", serialized_real_shard);
}
