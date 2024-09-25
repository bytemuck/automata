pub mod index;
pub use index::*;

#[derive(Debug)]
pub struct Graph<ND, ED> {
    pub nodes: Vec<NodeData<ND>>,
    pub edges: Vec<EdgeData<ED>>,
}

#[derive(Debug)]
pub struct NodeData<ND> {
    pub targets: Vec<EdgeIndex>,
    pub data: ND,
}

#[derive(Debug)]
pub struct EdgeData<ED> {
    pub target: NodeIndex,
    pub data: ED,
}

impl<ND, ED> Graph<ND, ED> {
    pub fn add_node(&mut self, data: ND) -> NodeIndex {
        let index = NodeIndex(self.nodes.len());

        self.nodes.push(NodeData {
            targets: vec![],
            data,
        });

        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, data: ED) -> EdgeIndex {
        let length = self.edges.len();
        let index = EdgeIndex(length);
        let node = &mut self.nodes[source];

        self.edges.push(EdgeData { target, data });
        node.targets.push(index);

        index
    }
}
