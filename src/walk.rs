use crate::{Graph, NodeIndex};

pub struct Walk<'graph, ND, ED> {
    graph: &'graph Graph<ND, ED>,
    node: NodeIndex,
    current: usize,
}

impl<ND, ED> Graph<ND, ED> {
    pub fn successors(&self, source: NodeIndex) -> Walk<ND, ED> {
        Walk {
            graph: self,
            node: source,
            current: 0,
        }
    }
}

impl<'graph, ND, ED> Iterator for Walk<'graph, ND, ED> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        let targets = &self.graph.nodes[self.node].targets;
        let edges = &self.graph.edges;
        match self.current >= targets.len() {
            true => None,
            false => {
                self.current += 1;
                Some(edges[targets[self.current - 1]].target)
            }
        }
    }
}
