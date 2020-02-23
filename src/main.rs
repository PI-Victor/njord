extern crate futures;
extern crate serde;
extern crate tokio;
#[macro_use]
extern crate serde_derive;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::net::{Ipv4Addr, SocketAddrV4};
use std::str;

use clap::{App, AppSettings, Arg, SubCommand};
use config::{Config, ConfigError, Environment, File};

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

    let mut config = Configuration::default();
    // NOTE: should this have more than start?
    // if not, then start should be removed completely.
    if let Some(matches) = matches.subcommand_matches("start") {
        config = Configuration::new(matches.value_of("config").unwrap()).unwrap();
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

    Ok::<(), std::io::Error>(())
}

#[derive(Deserialize, Debug)]
pub struct Configuration {
    bind_address: Ipv4Addr,
    peers: Vec<SocketAddrV4>,
    replicas: u8,
    partitions: u8,
    log_path: String,
    node_name: String,
}

impl Default for Configuration {
    fn default() -> Self {
        let sample_peer = "127.0.0.1:8717".parse::<SocketAddrV4>().unwrap();

        Self {
            bind_address: "127.0.0.1".parse::<Ipv4Addr>().unwrap(),
            peers: vec![sample_peer],
            partitions: 4,
            replicas: 5,
            log_path: LOG_PATH.to_string(),
            node_name: DEFAULT_NODE_NAME.to_string(),
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
