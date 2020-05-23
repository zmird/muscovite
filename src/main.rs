#[macro_use]
extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate log;
extern crate log4rs;
extern crate chrono;

mod network;
mod constants;
mod game;
mod rules;
mod player;
mod search;
mod serialization;
mod logging;

use constants::*;
use player::Player;
use logging::config_logs;
use clap::{App, Arg};
use std::error::Error;
use chrono::Local;
use log::info;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Muscovite")
        .version("0.1")
        .about("A Tablut Engine")
        .arg(Arg::with_name("color")
            .help("Color of the player, black or white.")
            .required(true)
            .index(1))
        .arg(Arg::with_name("name")
            .short("n")
            .long("name")
            .help("Change default name")
            .takes_value(true)
            .default_value(NAME))
        .arg(Arg::with_name("address")
            .short("a")
            .long("address")
            .default_value("localhost")
            .help("Server ip address")
            .takes_value(true))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .help("Server ip port")
            .takes_value(true))
        .arg(Arg::with_name("timeout")
            .short("t")
            .long("timeout")
            .help("Timeout for move")
            .takes_value(true))
        .get_matches();

    let color: String = value_t!(matches, "color", String).unwrap().to_lowercase();
    if color.as_str() != WHITE && color.as_str() != BLACK {
        println!("Error: color can be white or black");
        std::process::exit(1);
    }

    let name: String = value_t!(matches, "name", String).unwrap();

    let address: String = value_t!(matches, "address", String).unwrap();

    let port: u32;
    let result = value_t!(matches, "port", u32);
    match result {
        Ok(p) => { port = p; },
        Err(_e) => {
            if color.as_str() == WHITE {
               port = DEFAULT_WHITE_PORT;
            } else {
               port = DEFAULT_BLACK_PORT;
            }

        }
    }

    let timeout: u64;
    let result = value_t!(matches, "timeout", u64);
    match result {
        Ok(t) => { timeout = t; },
        Err(_e) => {
            timeout = 60;
        }
    }

    let color: String = value_t!(matches, "color", String).unwrap().to_lowercase();

    config_logs(format!("{}_{}.txt", Local::now().format("%Y-%m-%d_%H:%M:%S"), color));

    info!(target: "main", "
                                             _ __
       ____ ___  __  ________________ _   __(_) /____
      / __ `__ \\/ / / / ___/ ___/ __ \\ | / / / __/ _ \\
     / / / / / / /_/ (__  ) /__/ /_/ / |/ / / /_/  __/
    /_/ /_/ /_/\\__,_/____/\\___/\\____/|___/_/\\__/\\___/

    name: {name}
    color: {color}
    address: {address}
    port: {port},
    timeout: {timeout}

    ", name=name, color=color, address=address, port=port, timeout=timeout);

    let mut player = Player::init(name, color, address, port, timeout)?;
    player.game_loop();
    Ok(())
}
