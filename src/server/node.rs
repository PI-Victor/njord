//  Copyright 2020 Palade Ionut-Victor
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//  http://www.apache.org/licenses/LICENSE-2.0

use raft::raw_node::RawNode;
use raft::storage::MemStorage;
use raft::Config;

use std::io::Error;

#[derive(Debug)]
pub struct Node {}

impl Default for Node {
    fn default() -> Self {
        Node {}
    }
}

impl Node {
    pub fn init(self) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Node;

    #[test]
    fn test_node_init() {
        let node = Node::default();
        node.init().unwrap();
    }
}
