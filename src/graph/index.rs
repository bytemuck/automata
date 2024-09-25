use std::ops::{Deref, Index, IndexMut};

use super::{EdgeData, NodeData};

#[derive(Debug, Clone, Copy)]
pub struct NodeIndex(pub usize);

#[derive(Debug, Clone, Copy)]
pub struct EdgeIndex(pub usize);

impl Deref for NodeIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for EdgeIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<ND> Index<NodeIndex> for Vec<NodeData<ND>> {
    type Output = NodeData<ND>;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self[*index]
    }
}

impl<ND> IndexMut<NodeIndex> for Vec<NodeData<ND>> {
    fn index_mut(&mut self, index: NodeIndex) -> &mut Self::Output {
        &mut self[*index]
    }
}

impl<ED> Index<EdgeIndex> for Vec<EdgeData<ED>> {
    type Output = EdgeData<ED>;

    fn index(&self, index: EdgeIndex) -> &Self::Output {
        &self[*index]
    }
}

impl<ND> IndexMut<EdgeIndex> for Vec<EdgeData<ND>> {
    fn index_mut(&mut self, index: EdgeIndex) -> &mut Self::Output {
        &mut self[*index]
    }
}
