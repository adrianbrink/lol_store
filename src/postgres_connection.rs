use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub struct PostgresConnector {
    connection: PgConnection,
}

impl PostgresConnector {
    pub fn new() -> Result<PostgresConnector, PostgresConnectorError> {
        dotenv().ok();
        // let url = env::var("postgres://postgres@postgres/lol_store")?;
        let conn = PgConnection::establish("postgres://postgres@postgres/lol_store")?;
        Ok(PostgresConnector { connection: conn })
    }

    pub fn get_connection(&self) -> &PgConnection {
        &self.connection
    }
}

#[derive(Debug)]
pub enum PostgresConnectorError {
    Var(env::VarError),
    Diesel(ConnectionError),
}

impl From<env::VarError> for PostgresConnectorError {
    fn from(err: env::VarError) -> PostgresConnectorError {
        PostgresConnectorError::Var(err)
    }
}

impl From<ConnectionError> for PostgresConnectorError {
    fn from(err: ConnectionError) -> PostgresConnectorError {
        PostgresConnectorError::Diesel(err)
    }
}


// pub fn create_shard(&self, shard: &Shard) -> LoadedShard {
//     use schema::shards;
//     diesel::insert(shard)
//         .into(shards::table)
//         .get_result(&self.connection)
//         .expect("Error saving new shard.")
// }

// pub fn create_shards(&self, shards: &Vec<Shard>) -> Vec<LoadedShard> {
//     use schema::shards;
//     diesel::insert(shards)
//         .into(shards::table)
//         .get_results(&self.connection)
//         .expect("Error saving new shards.")
// }

// pub fn create_game(&self, game: &Game) -> LoadedGame {
//     use schema::games;
//     diesel::insert(game)
//         .into(games::table)
//         .get_result(&self.connection)
//         .expect("Error saving new game.")
// }

// pub fn create_games(&self, games: &Vec<Game>) -> Vec<LoadedGame> {
//     use schema::games;
//     diesel::insert(games)
//         .into(games::table)
//         .get_results(&self.connection)
//         .expect("Error saving new games.")
// }