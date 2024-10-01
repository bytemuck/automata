use crate::{EdgeIndex, Graph, LocalEdgeIndex, VertexIndex};

pub struct Children<'graph, ND, ED> {
    graph: &'graph Graph<ND, ED>,
    parent: VertexIndex,
    index: LocalEdgeIndex,
}

impl<'graph, ND, ED> Iterator for Children<'graph, ND, ED> {
    type Item = (EdgeIndex, VertexIndex);

    fn next(&mut self) -> Option<Self::Item> {
        let source_edges = &self.graph.vertices[self.parent].edges;
        let graph_edges = &self.graph.edges;

        match *self.index >= source_edges.len() {
            true => None,
            false => {
                *self.index += 1;
                Some((
                    source_edges[*self.index - 1],
                    graph_edges[source_edges[*self.index - 1]].next,
                ))
            }
        }
    }
}

impl<ND, ED> Graph<ND, ED> {
    pub fn successors(&self, source_vertex: VertexIndex) -> Children<ND, ED> {
        Children {
            graph: self,
            parent: source_vertex,
            index: LocalEdgeIndex(0),
        }
    }
}
