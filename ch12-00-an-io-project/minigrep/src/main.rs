extern crate minigrep;

use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // unwrap_or_elseでerrが返って来た際、panic!以外の挙動を定義することができる
        print!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1)
    }
}
