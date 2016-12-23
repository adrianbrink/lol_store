#![feature(proc_macro)]

#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate diesel_codegen;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Game, NewGame};

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, name: &'a str) -> Game {
    use schema::games;

    let new_game = NewGame {
        name: name,
    };

    diesel::insert(&new_game).into(games::table)
        .get_result(conn)
        .expect("Error saving new game.")
}
