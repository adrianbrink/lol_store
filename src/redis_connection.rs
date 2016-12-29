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
    set_name: String,
    list_name: String,
}

impl UniqueQueue {
    pub fn new(connection: Connection, name: String) -> UniqueQueue {
        let set = format!("{}_set", &name);
        let list = format!("{}_list", &name);
        UniqueQueue {
            connection: connection,
            key_name: name,
            set_name: set,
            list_name: list,
        }
    }

    // This should panic when the Result from adding to the set is Err,
    // because it implies that the connection isn't set up properly.
    // TODO - make this an atomic operation
    pub fn push(&self, value: i64) -> i32 {
        let x = self.connection.sadd::<_, _, i32>(&self.set_name, value).unwrap();
        if x == 1 {
            self.connection.rpush::<_, _, i32>(&self.list_name, value).unwrap();
        }
        x
    }

    // TODO - make this an atomic operation
    pub fn pop(&self) -> Option<i32> {
        let x = self.connection.lpop::<_, i32>(&self.list_name);
        match x {
            Ok(val) => {
                let _ = self.connection.srem::<_, _, i32>(&self.set_name, val).unwrap();
                Some(val)
            }
            Err(_) => None,
        }
    }
}