extern crate futures;
extern crate failure;
extern crate tokio;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;


use std::net::{SocketAddrV4, Ipv4Addr};
use std::error::Error;
use std::str;

use clap::{AppSettings, App, Arg, SubCommand};
use config::{File, Environment, Config, ConfigError};
use tokio::prelude::*;
use tokio::net::TcpListener;

mod discovery;

use discovery::nodes::Node;


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
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("njord")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(VERSION)
        .about(ASCIIART)
        .author("Cloudflavor Org")
        .arg(Arg::with_name("master")
             .help("set node up as master"))
        .arg(Arg::with_name("verbosity")
             .multiple(true)
             .short("v")
             .help("application verbosity level 0-4")
             .long("verbosity"))
        .subcommand(SubCommand::with_name("start")
                    .help("start the application")
                    .arg(Arg::with_name("config")
                         .short("c")
                         .long("config")
                         .value_name("JSON, TOM, YAM, HJSON, INI - configuration")
                         .takes_value(true)
                         .help("path to config file")
                         .required(true)))
        .get_matches();

    let mut config = Configuration::default();
    // NOTE: should this have more than start?
    // if not, then start should be removed completely.
    if let Some(matches) = matches.subcommand_matches("start") {
        config = Configuration::new(matches.value_of("config").unwrap())
            .unwrap();
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
    let client_sock_addr = format!("{:}:6505", &config.bind_address)
        .parse::<SocketAddrV4>()
        .unwrap();
    let node_sock_addr = format!("{:}:6404", &config.bind_address)
        .parse::<SocketAddrV4>()
        .unwrap();

    debug!("Initializing node...");
    // The node will not initialize until it gets more nodes available to be
    // able to meet decorum.
    let mut node = Node::default();
    tokio::spawn(async move {
        node.init(&config)
            .map(|e| {
                debug!("Initializing node, waiting for peers...");
                let mut looper = true;
                while looper {

                }
            }).await;
    });

    let mut listener = TcpListener::bind(client_sock_addr.to_string()).await?;
    debug!("Listening for connections...");
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            loop {
                let mut buff = [0; 1024];
                match socket.read(&mut buff).await {
                    Ok(n) if n == 0 => {
                        info!("empty buffer received, ignoring...");
                        return;
                    },
                    Ok(n) => {
                        let rec = str::from_utf8(&buff).unwrap();
                        info!("wrote: {} buffer size: {}", rec, n);
                    },
                    Err(e) => {
                        error!("failed to write to socket: {:?}", e);
                        return;
                    }
                };
            }
        });
    }
}

#[derive(Deserialize, Debug)]
pub struct Configuration {
    bind_address: Ipv4Addr,
    peers: Vec<SocketAddrV4>,
    partitions: u8,
    log_path: String
}

impl Default for Configuration {
    fn default() -> Self {
        let sample_peer = "127.0.0.1:6505".parse::<SocketAddrV4>().unwrap();

        Self{
            bind_address: "127.0.0.1".parse::<Ipv4Addr>().unwrap(),
            peers: vec![sample_peer],
            partitions: 4,
            log_path: "/tmp/log/".to_string()
        }
    }
}

impl Configuration {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        let mut c = Config::new();
        c.merge(File::with_name(path))?;
        c.merge(Environment::with_prefix("NJORD_CONFIG"))?;
        c.try_into()
    }
}
