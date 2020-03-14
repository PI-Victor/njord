//  Copyright 2020 Palade Ionut-Victor
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//  http://www.apache.org/licenses/LICENSE-2.0

use std::io::Error;

#[derive(Debug)]
pub struct Node {
    pub state: State,
}
#[derive(Debug)]
pub enum State {
    Running,
    Pending,
    Failed,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            state: State::Pending,
        }
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
    use super::State;

    #[test]
    fn test_node_default() {
        let node = Node::default();
        let state = match node.state {
            State::Pending => true,
            _ => false,
        };
        assert_eq!(state, true)
    }

    #[test]
    fn test_node_init() {
        let node = Node::default();
        node.init().unwrap();
    }
}
