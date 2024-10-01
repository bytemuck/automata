#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VertexState {
    Common,
    Final,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EdgeState {
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Finality {
    Common,
    Final,
}

pub trait State {
    type S;

    fn get_state(&self) -> Self::S;
}

impl State for VertexState {
    type S = VertexState;

    fn get_state(&self) -> Self::S {
        *self
    }
}

impl State for EdgeState {
    type S = EdgeState;
    fn get_state(&self) -> Self::S {
        *self
    }
}

pub trait StateFinality {
    fn get_finality(&self) -> Finality;
}

impl StateFinality for VertexState {
    fn get_finality(&self) -> Finality {
        match self {
            VertexState::Common => Finality::Common,
            VertexState::Final => Finality::Final,
        }
    }
}
