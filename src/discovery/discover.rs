use super::nodes::Node;

#[derive(Debug)]
pub struct Registry<'a> {
    // peers is of the Option type for init convenience here.
    pub peers: Vec<Option<&'a Node>>,
    pub pending_cvorum: bool
}

impl<'a> Registry<'a> {
    pub fn register(&mut self, node: &'a Node) {
        let find = self.peers.iter()
            .find(|&n| {
                match Some(n) {
                    Some(e) => *e == Some(node),
                    None => false
                }
            });

        match find {
            Some(_) => info!("node already present"),
            None => {
                debug!("Adding node: {:?}", node);
                self.peers.push(Some(node))
            }
        }
    }

    async fn diregister(&mut self, index: usize) {
        self.peers.remove(index);
    }

    pub async fn pool(&mut self) -> usize {
        self.pending_cvorum = true;
        8
    }
    async fn check_cvorum(&self) -> bool {
        let size = self.peers.len();
        if size >= 3 {
            return true
        }
        false
    }
}

impl Default for Registry<'_> {
    fn default() -> Self {
        let peers = vec![None];
        Self {
            peers: peers,
            pending_cvorum: true
        }
    }
}
