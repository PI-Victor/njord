use super::nodes::Node;
use std::io::Error;

/// The `registry` contains all the functional nodes to which the leader will
/// broadcast messages to.
///
/// It will ensure that the nodes are all valid by reporting Status::Running.
/// Nodes that do not fulfill the `Running` state will be remove from the
/// registry.
///
/// In order for data to be validated properly, the registry seeks to fulfill a
/// semi-cvorum or a minimum of 3 nodes. This type of cvorum is made for testing
/// purposes and does not ensure proper data validation.
///
/// The recommended cvorum is formed out of at least 5 nodes (4 nodes and a
/// leader).

#[derive(Debug)]
pub struct Registry {
    // peers is of the Option type for init convenience here.
    pub peers: Vec<Option<Node>>,
    pub cvorum: bool,
}

impl Registry {
    pub fn init(node: Node) -> Registry {
        let mut peers = vec![];
        peers.push(Some(node));

        Registry {
            peers: peers,
            cvorum: false,
        }
    }
    /// Register handles the registration of a node.
    /// The nodes is checked against the current list of nodes that the registry
    /// has.
    /// The node needs to be in a `Running` state to be added successfully.
    pub async fn register(&mut self, node: Option<Node>) {
        match node {
            None => return,
            Some(node) => {
                // NOTE: this is, pretty much garbage.
                let found = self.peers.iter().find(|&n| match n {
                    Some(no) => {
                        let x = &node == no;
                        debug!("i found a node: {:?}", x);
                        true
                    }
                    None => false,
                });

                match found {
                    Some(_) => debug!("Node already present. Skipping..."),
                    None => {
                        self.peers.push(Some(node));
                        debug!("{:?}", self);
                    }
                }
                self.check_cvorum().await;
                debug!("No of peers: {:?}", self.peers);
            }
        }
    }
    /// Unregister will remove a node from the registry.
    async fn unregister(&mut self, index: usize) {
        self.peers.remove(index);
        self.check_cvorum().await;
    }
    /// This function will do a simple check against the node registry to see if
    /// there are enough nodes to meet cvorum.
    ///
    /// #### Raft
    /// Not fully implemented at this time so the check is quite simple:
    ///
    /// A minimum of three nodes is required for a validation, this is mostly
    /// for development purposes.
    /// For more serious use cases a minimum of 5 nodes should be considered.
    async fn check_cvorum(&mut self) {
        if self.peers.len() >= 3 {
            self.cvorum = true;
            return;
        }
        self.cvorum = false;
        debug!("cvorum not met!");
    }
}
