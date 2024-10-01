use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::State;

use super::{Edge, Vertex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexIndex(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeIndex(pub usize);

impl Deref for VertexIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VertexIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for EdgeIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EdgeIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<VD> Index<VertexIndex> for Vec<Vertex<VD>>
where
    VD: State,
{
    type Output = Vertex<VD>;

    fn index(&self, index: VertexIndex) -> &Self::Output {
        &self[*index]
    }
}

impl<VD> IndexMut<VertexIndex> for Vec<Vertex<VD>>
where
    VD: State,
{
    fn index_mut(&mut self, index: VertexIndex) -> &mut Self::Output {
        &mut self[*index]
    }
}

impl<ED> Index<EdgeIndex> for Vec<Edge<ED>>
where
    ED: State,
{
    type Output = Edge<ED>;

    fn index(&self, index: EdgeIndex) -> &Self::Output {
        &self[*index]
    }
}

impl<ED> IndexMut<EdgeIndex> for Vec<Edge<ED>>
where
    ED: State,
{
    fn index_mut(&mut self, index: EdgeIndex) -> &mut Self::Output {
        &mut self[*index]
    }
}
