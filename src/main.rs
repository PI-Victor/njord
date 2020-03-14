//  Copyright 2020 Palade Ionut-Victor
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//  http://www.apache.org/licenses/LICENSE-2.0

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate futures;
extern crate raft_rs;
extern crate tarpc;
extern crate tokio;

use std::net::SocketAddrV4;
use std::str;

use clap::{App, AppSettings, Arg, SubCommand};

mod server;
mod storage;
mod util;

use util::config as conf;

const VERSION: &str = "v0.1.0-alpha";
const ASCIIART: &str = r#"
 _       _________ _______  _______  ______
( (    /|\__    _/(  ___  )(  ____ )(  __  \
|  \  ( |   )  (  | (   ) || (    )|| (  \  )
|   \ | |   |  |  | |   | || (____)|| |   ) |
| (\ \) |   |  |  | |   | ||     __)| |   | |
| | \   |   |  |  | |   | || (\ (   | |   ) |
| )  \  ||\_)  )  | (___) || ) \ \__| (__/  )
|/    )_)(____/   (_______)|/   \__/(______/
"#;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let matches = App::new("njord")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(VERSION)
        .about(ASCIIART)
        .author("Cloudflavor Org")
        .arg(Arg::with_name("master").help("set node up as master"))
        .arg(
            Arg::with_name("verbosity")
                .multiple(true)
                .short("v")
                .help("application verbosity level 0-4")
                .long("verbosity"),
        )
        .subcommand(
            SubCommand::with_name("start")
                .help("start the application")
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("JSON, TOM, YAM, HJSON, INI - configuration")
                        .takes_value(true)
                        .help("path to config file")
                        .required(true),
                ),
        )
        .get_matches();

    let mut config = conf::Configuration::default();
    // NOTE: should this have more than start?
    // if not, then start should be removed completely.
    if let Some(matches) = matches.subcommand_matches("start") {
        config = conf::Configuration::new(matches.value_of("config").unwrap()).unwrap();
    }
    let log_level = match matches.occurrences_of("verbosity") {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    env_logger::Builder::from_default_env()
        .filter(Some(module_path!()), log_level)
        .init();
    debug!("Loaded configuration: {:?}", config);
    let _client_sock_addr = format!("{:}:8718", &config.bind_address)
        .parse::<SocketAddrV4>()
        .unwrap();
    let _node_sock_addr = format!("{:}:8717", &config.bind_address)
        .parse::<SocketAddrV4>()
        .unwrap();
    info!("Initializing node, waiting for peers...");

    Ok::<(), std::io::Error>(())
}
