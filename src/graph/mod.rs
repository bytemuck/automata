pub mod index;
pub use index::*;

pub mod local;
pub use local::*;

#[derive(Debug)]
pub struct Graph<VD, ED> {
    pub vertices: Vec<Vertex<VD>>,
    pub edges: Vec<Edge<ED>>,
}

#[derive(Debug)]
pub struct Vertex<VD> {
    pub edges: Vec<EdgeIndex>,
    pub data: VD,
}

#[derive(Debug)]
pub struct Edge<ED> {
    pub next: VertexIndex,
    pub previous: VertexIndex,
    pub data: ED,
}

impl<VD, ED> Graph<VD, ED> {
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
            next: target,
            previous: source,
            data,
        });

        let length = self.edges.len() - 1;
        let index = EdgeIndex(length);

        self.vertices[source].edges.push(index);

        index
    }
}
