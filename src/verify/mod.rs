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

    visited: Vec<bool>,
    stack: Vec<VertexIndex>,
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

            visited: vec![false; self.edges.len()],
            stack: Vec::new(),
        }
    }
}

impl<'graph, VD, ED> Verify<'graph, VD, ED>
where
    VD: State,
    ED: State,

    VD::S: StateFinality,
    ED::S: PartialEq<state::EdgeState>,
{
    fn backtrack(&mut self) -> Finality {
        if let Some(v) = self.stack.pop() {
            println!("backtracked to {:?}", self.source);

            self.current -= 1;
            self.source = v;

            return self.verify();
        }

        println!("could not backtrack.");
        Finality::Common
    }

    fn advance(&mut self, e: EdgeIndex, v: VertexIndex) -> Finality {
        println!("advanced to {:?} passing by {:?}", v, e);

        self.visited[*e] = true;

        self.stack.push(self.source);
        self.source = v;

        self.current += 1;

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

            if edge.data.get_state() == self.target[self.current] && !self.visited[*e] {
                return self.advance(e, v);
            }
        }

        self.backtrack()
    }
}
