// Methods / structures associated with traversing the graph

use std::iter::Iterator;

use crate::Graph;

pub struct Trace<T> {
    from: Option<T>,
    // over: Option<&Edge>,
    to: T,
}

pub struct Dfs<T> {
    at: T,
    visited: Vec<bool>,
    graph: &Graph,
}

impl Iterator<T> for Dfs<T> {
    type Item = Trace<T>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Dfs {
    pub fn new(g: &impl Graph) -> Dfs {
        let size = g.size();
        Dfs{
            visited: vec![false; size],
            graph: g,
        }
    }
}
