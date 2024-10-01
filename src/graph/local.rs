use std::ops::{Deref, DerefMut, Index, IndexMut};

use super::{Edge, Vertex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocalVertexIndex(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocalEdgeIndex(pub usize);

impl Deref for LocalVertexIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LocalVertexIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for LocalEdgeIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LocalEdgeIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<VD> Index<LocalVertexIndex> for Vec<Vertex<VD>> {
    type Output = Vertex<VD>;

    fn index(&self, index: LocalVertexIndex) -> &Self::Output {
        &self[*index]
    }
}

impl<VD> IndexMut<LocalVertexIndex> for Vec<Vertex<VD>> {
    fn index_mut(&mut self, index: LocalVertexIndex) -> &mut Self::Output {
        &mut self[*index]
    }
}

impl<ED> Index<LocalEdgeIndex> for Vec<Edge<ED>> {
    type Output = Edge<ED>;

    fn index(&self, index: LocalEdgeIndex) -> &Self::Output {
        &self[*index]
    }
}

impl<ED> IndexMut<LocalEdgeIndex> for Vec<Edge<ED>> {
    fn index_mut(&mut self, index: LocalEdgeIndex) -> &mut Self::Output {
        &mut self[*index]
    }
}
