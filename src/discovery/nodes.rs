use std::net::{SocketAddrV4, Ipv4Addr};
use std::fs::create_dir_all;

use actix::prelude::*;
use std::io::{Error, ErrorKind};

use crate::Configuration;

pub const LOG_PATH: &'static str = "/var/njord/log";

#[derive(Debug, Deserialize, PartialEq)]
pub enum State {
    Ok,
    Pending,
    Failed
}

#[derive(Message, Deserialize, Debug)]
pub struct Payload {
    ip:  SocketAddrV4,
    log_path: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Node {
    ip: Ipv4Addr,
    log_path: &'static str,
    state: State,
    leader: bool
}

impl Node {
    pub async fn init(&mut self, conf: &Configuration) -> Result<Self, Error> {
        match create_dir_all(&conf.log_path) {
            Ok(_) => Ok(
                Self {
                    ip: conf.bind_address.clone(),
                    log_path: LOG_PATH,
                    state: State::Pending,
                    leader: false
                }
            ),
            Err(e) => Err(Error::new(ErrorKind::Other, e))
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".parse::<Ipv4Addr>().unwrap(),
            log_path: LOG_PATH,
            state: State::Ok,
            leader: false
        }
    }
}

impl Actor for Node {
    type Context = Context<Node>;
}

impl Handler<Payload> for Node {
    type Result = ();
    fn handle(&mut self, msg: Payload, ctx: &mut Context<Self>) {}
}
