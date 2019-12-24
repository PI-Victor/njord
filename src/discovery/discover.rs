use futures::prelude::*;

use protobuf::parse_from_bytes;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::task;

use crate::protos::node::{Node, State};
use crate::util::config;

/// The `registry` contains all the functional nodes to which the leader will
/// broadcast messages to.
///
/// It will ensure that the nodes are all valid by reporting `Status::Running`.
/// Nodes that do not fulfill the `Running` state will be remove from the
/// registry.
///
/// In order for data to be validated properly, the registry seeks to fulfill a
/// semi-quorum or a minimum of 3 nodes. This type of quorum is made for testing
/// purposes and does not ensure proper data validation.
///
/// The recommended quorum is formed out of at least 5 nodes (4 nodes and a
/// leader).

#[derive(Debug)]
pub struct Registry {
    // peers is of the Option type for init convenience here.
    pub peers: Vec<Node>,
    pub quorum: bool,
}

impl Registry {
    /// Register handles the registration of a node.
    /// The nodes is checked against the current list of nodes that the registry
    /// has.
    /// The node needs to be in a `Running` state to be added successfully.
    pub async fn register(&mut self, node: Node) {
        match node.state {
            State::Running => {
                self.peers.push(node);
            }
            _ => {}
        }
    }
    /// Unregister will remove a node from the registry.
    async fn unregister(&mut self, index: usize) {
        self.peers.remove(index);
        self.check_quorum().await;
    }
    /// This function will do a simple check against the node registry to see if
    /// there are enough nodes to meet quorum.
    ///
    /// #### Raft
    /// Not fully implemented at this time so the check is quite simple:
    ///
    /// A minimum of three nodes is required for a validation, this is mostly
    /// for development purposes.
    /// For more serious use cases a minimum of 5 nodes should be considered.
    async fn check_quorum(&mut self) {
        if self.peers.len() >= 3 {
            self.quorum = true;
            return;
        }
        self.quorum = false;
        debug!("quorum not met!");
    }

    pub async fn start(mut self, conf: config::Configuration) {
        let c = conf.clone();
        self.start_listener(conf).await;
        self.start_coms(c).await;
    }

    async fn start_listener(&mut self, conf: config::Configuration) {
        task::spawn(async move {
            info!("{:?}", &conf.bind_address);
            TcpListener::bind(&conf.bind_address.to_string())
                .and_then(|mut socket| {
                    async move {
                        socket
                            .incoming()
                            .next()
                            .then(|stream| {
                                async move {
                                    info!("received {:?}", stream);
                                }
                            })
                            .await;
                        // match socket {
                        //     Ok(mut socket) => {

                        //     }
                        //     Err(err) => error!("failed to read stream: {:?}", err),
                        // }
                        Ok::<(), std::io::Error>(())
                    }
                })
                .await
                .unwrap();
        });
    }

    async fn start_coms(&mut self, conf: config::Configuration) {
        task::spawn(async move {
            for node_addr in conf.peers.iter() {
                debug!("Trying to contact: {:?}", &node_addr.to_string());
                TcpStream::connect(&node_addr.to_string())
                    .and_then(|socket| async move { Ok::<(), std::io::Error>(()) })
                    .await
                    .unwrap()

                // match client_socket {
                //     Ok(mut socket) => {
                //         let msg = init_node.write_to_bytes().unwrap();
                //         debug!("Writing message: {:?}", &res);
                //         let res = socket.write(&res).await;
                //         match res {
                //             Ok(_) => debug!("Wrote to client at: {:?}", socket),
                //             Err(f) => {
                //                 debug!("we got an error sending the message: {:?}", f)
                //             }
                //         }
                //     }
                //     Err(e) => debug!("Failed to connect to client: {:?}", &e),
                // }
            }
        });
    }
}

impl Default for Registry {
    fn default() -> Registry {
        let peers = vec![];
        Registry {
            peers: peers,
            quorum: false,
        }
    }
}
