pub mod index;
pub use index::*;

pub mod local;
pub use local::*;

use crate::State;

#[derive(Debug)]
pub struct Graph<VD, ED>
where
    VD: State,
    ED: State,
{
    pub vertices: Vec<Vertex<VD>>,
    pub edges: Vec<Edge<ED>>,
}

#[derive(Debug)]
pub struct Vertex<VD>
where
    VD: State,
{
    pub edges: Vec<EdgeIndex>,
    pub data: VD,
}

#[derive(Debug)]
pub struct Edge<ED: State> {
    pub vertex: VertexIndex,
    pub source: VertexIndex,
    pub data: ED,
}

impl<VD, ED> Graph<VD, ED>
where
    VD: State,
    ED: State,
{
    pub fn add_vertex(&mut self, data: VD) -> VertexIndex {
        self.vertices.push(Vertex {
            edges: vec![],
            data,
        });

        let length = self.vertices.len() - 1;
        let index = VertexIndex(length);

        index
    }

    pub fn add_edge(&mut self, source: VertexIndex, target: VertexIndex, data: ED) -> EdgeIndex {
        self.edges.push(Edge {
            source: source,
            vertex: target,
            data,
        });

        let length = self.edges.len() - 1;
        let index = EdgeIndex(length);

        self.vertices[source].edges.push(index);

        index
    }
}
