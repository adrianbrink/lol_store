extern crate lol_store;
extern crate diesel;

use self::lol_store::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What would you like the name to be?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", name, EOF);

    let game = create_post(&connection, name);
    println!("\nSaved game {} with id {}", name, game.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
