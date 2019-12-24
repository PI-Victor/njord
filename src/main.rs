extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate futures;
extern crate protobuf;
extern crate tokio;

use futures::prelude::*;

use std::net::SocketAddrV4;
use std::str;

use clap::{App, AppSettings, Arg, SubCommand};
use tokio::net::TcpListener;
use tokio::task;

mod discovery;
mod protos;
mod util;

use discovery::discover::Registry;

use protos::node;
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
    let client_sock_addr = format!("{:}:8718", &config.bind_address)
        .parse::<SocketAddrV4>()
        .unwrap();
    let node_sock_addr = format!("{:}:8717", &config.bind_address)
        .parse::<SocketAddrV4>()
        .unwrap();
    info!("Initializing node, waiting for peers...");
    let mut init_node = node::Node::new();

    init_node.set_address(config.bind_address.to_string().clone());
    init_node.set_id(config.node_name.clone());
    init_node.set_leader(true);
    init_node.set_state(node::State::Running);
    Registry::default().start(config).await;

    let mut client_listener = TcpListener::bind(&client_sock_addr.to_string())
        .await
        .unwrap();

    info!("Listening for data on: {:?}", &client_sock_addr.to_string());

    client_listener
        .incoming()
        .try_for_each_concurrent(None, |_client| {
            async move { task::spawn(async move { Ok::<(), std::io::Error>(()) }).await? }
        })
        .await?;
    Ok(())
}
