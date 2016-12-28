extern crate redis;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use redis::{Client, Commands, Connection};

pub struct RedisConnector {
    database_url: String,
    pub connection: Connection,
}

impl RedisConnector {
    pub fn new() -> Option<RedisConnector> {
        dotenv().ok();
        match env::var("REDIS_DATABASE_URL") {
            Ok(val) => {
                match Client::open(val.as_str()) {
                    Ok(client) => {
                        match client.get_connection() {
                            Ok(conn) => {
                                Some(RedisConnector {
                                    database_url: val,
                                    connection: conn,
                                })
                            }
                            Err(_) => None,
                        }
                    }
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    pub fn save_game_id(&self, game_id: i64) {
        let _: () = self.connection.set("game_id", game_id).unwrap();
    }

    pub fn retrieve_game_id(&self) -> i64 {
        self.connection.get("game_id").unwrap()
    }
}
