use crate::{EdgeIndex, Graph, LocalEdgeIndex, State, VertexIndex};

#[derive(Debug)]
pub struct Children<'graph, ND, ED>
where
    ND: State,
    ED: State,
{
    pub graph: &'graph Graph<ND, ED>,
    pub parent: VertexIndex,
    pub index: LocalEdgeIndex,
}

impl<'graph, ND, ED> Iterator for Children<'graph, ND, ED>
where
    ND: State,
    ED: State,
{
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
                    graph_edges[source_edges[*self.index - 1]].vertex,
                ))
            }
        }
    }
}

impl<ND, ED> Graph<ND, ED>
where
    ND: State,
    ED: State,
{
    pub fn successors(&self, source_vertex: VertexIndex) -> Children<ND, ED> {
        Children {
            graph: self,
            parent: source_vertex,
            index: LocalEdgeIndex(0),
        }
    }
}
