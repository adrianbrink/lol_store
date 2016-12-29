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
}

pub struct UniqueQueue {
    connection: Connection,
    key_name: String,
}

impl UniqueQueue {
    pub fn new(connection: Connection, name: String) -> UniqueQueue {
        UniqueQueue {
            connection: connection,
            key_name: name,
        }
    }

    pub fn push(&self, value: String) {
        let _ = self.connection.sadd::<_, _, i32>(&self.key_name, value);
    }

    pub fn pop(&self) -> String {
        unimplemented!();
    }
}