use std::fmt::{Debug, Display};

use automata::{EdgeState, Graph, State, VertexState};

#[derive(Debug)]
struct VertexData {
    state: VertexState,
    name: &'static str,
}

impl VertexData {
    pub fn new(state: VertexState, name: &'static str) -> Self {
        VertexData { state, name }
    }
}

impl Display for VertexData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.name)
    }
}

impl State for VertexData {
    type S = VertexState;

    fn get_state(&self) -> Self::S {
        self.state
    }
}

#[derive(Debug)]
struct EdgeData {
    state: EdgeState,
    name: &'static str,
}

impl EdgeData {
    pub fn new(state: EdgeState, name: &'static str) -> Self {
        EdgeData { state, name }
    }
}

impl Display for EdgeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.name)
    }
}

impl State for EdgeData {
    type S = EdgeState;

    fn get_state(&self) -> Self::S {
        self.state
    }
}

fn main() -> Result<(), &'static str> {
    let mut graph: Graph<VertexData, EdgeData> = Graph {
        vertices: vec![],
        edges: vec![],
    };

    let v0 = graph.add_vertex(VertexData::new(VertexState::Common, "v0"));
    let v1 = graph.add_vertex(VertexData::new(VertexState::Common, "v1"));
    let v2 = graph.add_vertex(VertexData::new(VertexState::Common, "v2"));
    let v3 = graph.add_vertex(VertexData::new(VertexState::Final, "v3"));

    graph.add_edge(v0, v1, EdgeData::new(EdgeState::A, "e0"));
    graph.add_edge(v0, v2, EdgeData::new(EdgeState::A, "e1"));

    graph.add_edge(v1, v1, EdgeData::new(EdgeState::B, "e2"));
    graph.add_edge(v1, v2, EdgeData::new(EdgeState::A, "e3"));

    graph.add_edge(v2, v3, EdgeData::new(EdgeState::B, "e4"));

    let mut verify = graph.verify(v0, &[EdgeState::A, EdgeState::B]);

    println!("{:?}", verify.verify());

    Ok(())
}
