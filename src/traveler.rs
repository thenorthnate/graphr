// Methods / structures associated with traversing the graph

use std::collections::HashMap;
use std::iter::Iterator;

use crate::Graph;

pub struct Trace<T> {
    from: Option<T>,
    // over: Option<&Edge>,
    to: T,
}

pub struct Dfs<T> {
    at: T,
    visited: HashMap<T, bool>,
    graph: &Graph,
}

impl Iterator<T> for Dfs<T> {
    type Item = Trace<T>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Dfs<T> {
    pub fn new(g: &impl Graph, start: T) -> Dfs {
        let size = g.size();
        Dfs{
            at: start,
            visited: HashMap::new(),
            graph: g,
        }
    }
}
