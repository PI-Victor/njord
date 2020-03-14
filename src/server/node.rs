use std::io::{Error, ErrorKind};

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

impl Node {
    fn default() -> Self {
        Node {
            state: State::Pending,
        }
    }
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
