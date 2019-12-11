extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate protobuf;
extern crate tokio;

use futures::future::*;
use futures::prelude::*;
use futures::stream::*;

use std::net::{Ipv4Addr, SocketAddrV4};
use std::str;
use std::string::String;
use std::sync::mpsc;

use clap::{App, AppSettings, Arg, SubCommand};
use config::{Config, ConfigError, Environment, File};
use protobuf::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::task;

mod discovery;
mod protos;

use discovery::discover::Registry;
use discovery::nodes::{DEFAULT_NODE_NAME, LOG_PATH};
use protos::node;

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
    let mut init_node = node::Node::new();

    init_node.set_address(config.bind_address.to_string().clone());
    init_node.set_id(config.node_name.clone());
    init_node.set_leader(true);
    init_node.set_state(node::Node_State::Running);
    let mut registry = Registry::default();

    task::spawn(async move {
        info!("{:?}", registry);
    });

    task::spawn(async move {
        TcpListener::bind(&node_sock_addr.to_string())
            .then(|socket| {
                async move {
                    match socket {
                        Ok(mut stream) => {
                            let res = stream.incoming().next().await.unwrap();
                            match res {
                                Ok(mut msg) => {
                                    info!("{:?}", msg);
                                    let mut buf = vec![];
                                    msg.read_to_end(&mut buf).await.unwrap();
                                    let node = parse_from_bytes(&buf).unwrap();
                                    registry.register(node).await;
                                }
                                Err(err) => error!("{:}", err),
                            }
                        }
                        Err(err) => error!("failed to read stream: {:}", err),
                    }
                }
            })
            .await;
    });

    task::spawn(async move {
        // TODO: spawn a task for each node connection attempt;
        for node_addr in config.peers.iter() {
            debug!("Trying to contact: {:?}", &node_addr.to_string());

            //            thread::sleep(time::Duration::from_secs(3));
            let client_socket = TcpStream::connect(&node_addr.to_string()).await;
            match client_socket {
                Ok(mut socket) => {
                    let msg = init_node.write_to_bytes();
                    match msg {
                        Ok(res) => {
                            debug!("Writing message: {:?}", &res);
                            let res = socket.write(&res).await;
                            match res {
                                Ok(_) => debug!("Wrote to client at: {:?}", socket),
                                Err(f) => debug!("we got an error sending the message: {:?}", f),
                            }
                        }
                        Err(err) => info!("failed to byte serialize: {:}", err),
                    }
                }
                Err(e) => debug!("Failed to connect to client: {:?}", &e),
            }
        }
    })
    .await?;

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
