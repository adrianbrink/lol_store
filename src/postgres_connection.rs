extern crate dotenv;
extern crate diesel;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use super::models::{Shard, LoadedShard, Game, LoadedGame};

pub struct PostgresConnector {
    // keep this around, damn you github
    database_url: String,
    pub connection: PgConnection,
}

impl PostgresConnector {
    pub fn new() -> Option<PostgresConnector> {
        dotenv().ok();
        match env::var("POSTGRES_DATABASE_URL") {
            Ok(val) => {
                match PgConnection::establish(&val) {
                    Ok(conn) => {
                        Some(PostgresConnector {
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

    pub fn create_shard(&self, shard: &Shard) -> LoadedShard {
        use schema::shards;
        diesel::insert(shard)
            .into(shards::table)
            .get_result(&self.connection)
            .expect("Error saving new shard.")
    }

    pub fn create_shards(&self, shards: &Vec<Shard>) -> Vec<LoadedShard> {
        use schema::shards;
        diesel::insert(shards)
            .into(shards::table)
            .get_results(&self.connection)
            .expect("Error saving new shards.")
    }

    pub fn create_game(&self, game: &Game) -> LoadedGame {
        use schema::games;
        diesel::insert(game)
            .into(games::table)
            .get_result(&self.connection)
            .expect("Error saving new game.")
    }

    pub fn create_games(&self, games: &Vec<Game>) -> Vec<LoadedGame> {
        use schema::games;
        diesel::insert(games)
            .into(games::table)
            .get_results(&self.connection)
            .expect("Error saving new games.")
    }
}
