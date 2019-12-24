use std::fs::create_dir_all;
use std::net::Ipv4Addr;

use std::io::{Error, ErrorKind};

use crate::util::config;

pub const LOG_PATH: &'static str = "opt/njord/log";
pub const DEFAULT_NODE_NAME: &'static str = "initial";
/// State represents the state of a node at any given time
///
/// `Running` - The normal operational state of a machine. In this state the
/// machine is added to the registry. Only machines that have this current state
/// are part of the registry.
///
/// `Pending` - The state of a machine before initialization. In this state the
/// machine is not part of the registry.
///
/// `Failed` - Failed is the state of the machine when the machine has problems
/// running, writing messages, or validating messages. In this state the machine
/// is removed from the registry.
#[derive(Serialize, Debug, Deserialize, PartialEq, Clone)]
pub enum State {
    Running,
    Pending,
    Failed,
}

/// Node references a node structure that captures the entire state of a
/// machine.
///
/// `Nodes` communicate with each other through a semi-raft implementation.
///
/// All the nodes that register with the leader, will be stored in the registry.
///
/// When a node first initializes, it adds itself to a local registry.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Node {
    pub ip: Ipv4Addr,
    pub log_path: String,
    pub state: State,
    pub leader: bool,
    pub name: String,
}

impl Node {
    /// Function init will initialize a local node, will attempt to create the
    /// location where the local log data is store and will flush the
    /// current machine state to the disk.
    pub async fn init(&mut self, conf: &config::Configuration) -> Result<Self, Error> {
        match create_dir_all(&conf.log_path) {
            Ok(_) => Ok(Self {
                ip: conf.bind_address,
                log_path: conf.log_path.to_string(),
                state: State::Pending,
                leader: false,
                name: conf.node_name.to_string(),
            }),
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}

impl Default for Node {
    /// Default is a convenience function that will return a new instance of a
    /// machine with some default values.
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".parse::<Ipv4Addr>().unwrap(),
            log_path: LOG_PATH.to_string(),
            state: State::Pending,
            leader: false,
            name: DEFAULT_NODE_NAME.to_string(),
        }
    }
}
