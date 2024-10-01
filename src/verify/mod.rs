use crate::{Graph, LocalEdgeIndex, VertexIndex};

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
    current: LocalEdgeIndex,
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
            current: LocalEdgeIndex(0),
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
    pub fn verify(&mut self) -> Finality {
        while *self.current < self.target.len() {
            *self.current += 1;
            println!("needs: {:?}", self.target[*self.current - 1]);

            let mut successors = self.graph.successors(self.source);
            if let Some((_, v)) = successors.find(|(e, _)| {
                self.graph.edges[*e].data.get_state() == self.target[*self.current - 1]
            }) {
                println!("found: {:?}", v);

                self.source = v;
            }
        }

        self.graph.vertices[*self.source]
            .data
            .get_state()
            .get_finality()
    }
}
