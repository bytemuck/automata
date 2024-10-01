use automata::{EdgeState, Graph, VertexState};

fn main() -> Result<(), &'static str> {
    let mut graph: Graph<VertexState, EdgeState> = Graph {
        vertices: vec![],
        edges: vec![],
    };

    let v0 = graph.add_vertex(VertexState::Common);
    let v1 = graph.add_vertex(VertexState::Common);
    let v2 = graph.add_vertex(VertexState::Common);
    let v3 = graph.add_vertex(VertexState::Final);

    graph.add_edge(v0, v1, EdgeState::A);
    graph.add_edge(v1, v2, EdgeState::A);
    graph.add_edge(v2, v3, EdgeState::B);

    let mut verify = graph.verify(v0, &[EdgeState::A, EdgeState::A, EdgeState::B]);

    println!("{:?}", verify.verify());

    Ok(())
}
