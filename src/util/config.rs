//  Copyright 2020 Palade Ionut-Victor
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//  http://www.apache.org/licenses/LICENSE-2.0

extern crate config;

use config::{Config, ConfigError, Environment, File};
use std::net::{Ipv4Addr, SocketAddrV4};

#[derive(Debug, Deserialize)]
pub struct Configuration<'a> {
    pub bind_address: Ipv4Addr,
    pub peers: Vec<SocketAddrV4>,
    pub node_name: String,
    pub storage: &'a str,
}

impl<'a> Default for Configuration<'a> {
    fn default() -> Self {
        let sample_peer = "127.0.0.1:8717".parse::<SocketAddrV4>().unwrap();

        Self {
            bind_address: "127.0.0.1".parse::<Ipv4Addr>().unwrap(),
            peers: vec![sample_peer],
            node_name: "default".to_string(),
            storage: "memory",
        }
    }
}

impl<'a> Configuration<'a> {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        let mut c = Config::new();
        c.merge(File::with_name(path))?;
        c.merge(Environment::with_prefix("NJORD_CONFIG"))?;
        c.try_into()
    }
}
