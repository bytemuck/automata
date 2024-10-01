use std::fmt::Debug;

use automata::{Graph, VertexIndex};

fn walk<ND, ED>(
    graph: &Graph<ND, ED>,
    source: VertexIndex,
    visited: &mut Vec<bool>,
    indent: usize,
    increment: usize,
) where
    ND: Debug,
    ED: Debug,
{
    for (e, n) in graph.successors(source) {
        if visited[e.0] {
            continue;
        }

        visited[e.0] = true;

        println!("{:indent$}{:?}", "", (e, n));
        walk(graph, n, visited, indent + increment, increment);
    }
}

fn main() -> Result<(), &'static str> {
    let mut graph: Graph<&'static str, &'static str> = Graph {
        vertices: vec![],
        edges: vec![],
    };

    let v0 = graph.add_vertex("v0");
    let v1 = graph.add_vertex("v1");
    let v2 = graph.add_vertex("v2");
    let v3 = graph.add_vertex("v3");
    let v4 = graph.add_vertex("v4");
    let v5 = graph.add_vertex("v5");
    let v6 = graph.add_vertex("v6");

    graph.add_edge(v0, v1, "e00");
    graph.add_edge(v0, v2, "e01");
    graph.add_edge(v0, v3, "e02");
    graph.add_edge(v0, v4, "e03");
    graph.add_edge(v1, v4, "e04");
    graph.add_edge(v1, v5, "e05");
    graph.add_edge(v1, v6, "e06");
    graph.add_edge(v2, v3, "e07");
    graph.add_edge(v3, v4, "e08");
    graph.add_edge(v4, v5, "e09");
    graph.add_edge(v5, v6, "e10");
    graph.add_edge(v6, v0, "e11");

    println!("{:?}", graph);
    walk(&graph, v0, &mut vec![false; graph.edges.len()], 0, 2);

    Ok(())
}
