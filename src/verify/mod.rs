use std::fmt::{Debug, Display};

use crate::{EdgeIndex, Graph, VertexIndex};

pub mod state;
pub use state::*;

pub struct Verify<'graph, VD, ED>
where
    VD: State,
    ED: State,
{
    graph: &'graph Graph<VD, ED>,
    source: VertexIndex,

    target: &'graph [EdgeState],
    current: usize,

    backtracked: Vec<bool>,
    stack: Vec<EdgeIndex>,
}

impl<VD, ED> Graph<VD, ED>
where
    VD: State,
    ED: State,
{
    pub fn verify<'graph>(
        &'graph self,
        source: VertexIndex,
        target: &'graph [EdgeState],
    ) -> Verify<VD, ED> {
        Verify {
            graph: self,
            source,
            target,
            current: 0,

            backtracked: vec![false; self.edges.len()],
            stack: Vec::new(),
        }
    }
}

impl<'graph, VD, ED> Verify<'graph, VD, ED>
where
    VD: State + Display,
    ED: State + Display,

    VD::S: StateFinality,
    ED::S: PartialEq<state::EdgeState>,
{
    fn backtrack(&mut self) -> Finality {
        if let Some(e) = self.stack.pop() {
            self.backtracked[*e] = true;

            self.current -= 1;
            self.source = self.graph.edges[*e].source;

            println!(
                "backtracked to {} passing by {}",
                self.graph.vertices[self.source].data, self.graph.edges[*e].data
            );

            return self.verify();
        }

        println!("could not backtrack.");
        Finality::Common
    }

    fn advance(&mut self, e: EdgeIndex, v: VertexIndex) -> Finality {
        self.stack.push(e);
        self.source = self.graph.edges[*e].vertex;

        self.current += 1;

        println!(
            "advanced to {} passing by {}",
            self.graph.vertices[v].data, self.graph.edges[e].data
        );

        return self.verify();
    }

    pub fn verify(&mut self) -> Finality {
        if self.current == self.target.len() {
            println!("no more input");

            let finality = self.graph.vertices[self.source]
                .data
                .get_state()
                .get_finality();

            return match finality {
                Finality::Common => self.backtrack(),
                Finality::Final => Finality::Final,
            };
        }

        for (e, v) in self.graph.successors(self.source) {
            let edge = &self.graph.edges[*e];

            if edge.data.get_state() == self.target[self.current] && !self.backtracked[*e] {
                return self.advance(e, v);
            }
        }

        self.backtrack()
    }
}
