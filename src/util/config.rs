extern crate config;

use crate::discovery::nodes::{DEFAULT_NODE_NAME, LOG_PATH};
use config::{Config, ConfigError, Environment, File};
use std::net::{Ipv4Addr, SocketAddrV4};

#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    pub bind_address: Ipv4Addr,
    pub peers: Vec<SocketAddrV4>,
    pub replicas: u8,
    pub partitions: u8,
    pub log_path: String,
    pub node_name: String,
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
