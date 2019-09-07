extern crate futures;
extern crate failure;
extern crate exitfailure;
extern crate tokio;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;


use std::net::SocketAddrV4;

use clap::{AppSettings, App, Arg, SubCommand};
use config::{File, Environment, Config, ConfigError};
use exitfailure::ExitFailure;


mod taxonomy;


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
async fn main() -> Result<(), Box<ExitFailure>> {
    let matches = App::new("njord")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(VERSION)
        .about(ASCIIART)
        .author("Cloudflavor Org")
        .arg(Arg::with_name("verbosity")
             .multiple(true)
             .short("v")
             .help("application verbosity level 0-4")
             .long("verbosity"))
        .subcommand(SubCommand::with_name("start")
                    .help("start the application")
                    .arg(Arg::with_name("config")
                         .short("c")
                         .long("configuration")
                         .value_name("JSON, TOML, YAML, HJSON, INI - configuration")
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
    
    Ok(())
}

#[derive(Deserialize, Debug)]
struct Configuration {
    bind_address:  SocketAddrV4
}

impl Default for Configuration {
    fn default() -> Self {
        Self{
            bind_address: "127.0.0.1:6504".parse::<SocketAddrV4>().unwrap()
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
