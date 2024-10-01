use crate::{Graph, VertexIndex};

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
    fn pop(&mut self) -> Option<Finality> {
        if let Some(v) = self.stack.pop() {
            self.current -= 1;
            self.source = v;

            println!("popped source: {:?}", self.source);

            return self.verify();
        }

        None
    }

    pub fn verify(&mut self) -> Option<Finality> {
        if self.current == self.target.len() {
            println!(
                "self.current == self.target.len(): {} {}",
                self.current,
                self.target.len()
            );

            let finality = self.graph.vertices[self.source]
                .data
                .get_state()
                .get_finality();

            return match finality {
                Finality::Common => self.pop(),
                Finality::Final => Some(finality),
            };
        }

        let successors = self.graph.successors(self.source);

        for (e, v) in successors {
            let edge = &self.graph.edges[*e];

            if edge.data.get_state() == self.target[self.current] && !self.visited[*e] {
                self.visited[*e] = true;

                self.current += 1;

                self.stack.push(self.source);
                self.source = v;

                println!("source: {:?}", self.source);

                return self.verify();
            }
        }

        self.pop()
    }
}
