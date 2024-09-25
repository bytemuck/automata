use std::fmt::Debug;

use automata::{Graph, NodeIndex};

fn walk<ND, ED>(graph: &Graph<ND, ED>, source: NodeIndex, indent: usize, increment: usize)
where
    ND: Debug,
    ED: Debug,
{
    for n in graph.successors(source) {
        println!("{:indent$}{:?}", "", graph.nodes[n].data);
        walk(graph, n, indent + increment, increment);
    }
}

fn main() -> Result<(), &'static str> {
    let mut graph: Graph<&'static str, &'static str> = Graph {
        nodes: vec![],
        edges: vec![],
    };

    let node0 = graph.add_node("node0");
    let node1 = graph.add_node("node1");
    let node2 = graph.add_node("node2");
    let node3 = graph.add_node("node3");
    let node4 = graph.add_node("node4");
    let node5 = graph.add_node("node5");
    let node6 = graph.add_node("node6");

    graph.add_edge(node0, node1, "edge00");
    graph.add_edge(node0, node2, "edge01");
    graph.add_edge(node0, node3, "edge02");
    graph.add_edge(node0, node4, "edge03");
    graph.add_edge(node1, node4, "edge04");
    graph.add_edge(node1, node5, "edge05");
    graph.add_edge(node1, node6, "edge06");
    graph.add_edge(node2, node3, "edge07");
    graph.add_edge(node3, node4, "edge08");
    graph.add_edge(node4, node5, "edge09");
    graph.add_edge(node5, node6, "edge10");

    println!("{:?}", graph);
    walk(&graph, node0, 0, 2);

    Ok(())
}
