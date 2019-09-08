use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use std::fs::create_dir_all;

use actix::prelude::*;
use std::io::{Error, ErrorKind};


const LOG_PATH: &'static str = "/var/njord/log";

#[derive(Message)]
struct Payload {
    ip:  SocketAddrV4,
    log_path: &'static str
}

struct Node {
    ip: Ipv4Addr,
    log_path: String
}

impl Node {
    fn init(&mut self) -> Result<Self, Error> {
        match create_dir_all(LOG_PATH) {
            Ok(_) => Ok(
                Node {
                    ip: "127.0.0.1".parse::<Ipv4Addr>().unwrap(),
                    log_path: LOG_PATH.to_string()
                }
            ),
            Err(e) => Err(Error::new(ErrorKind::Other, e))
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
