// Default implementations for the graph

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::ops::Add;
// use std::ops::{Index, IndexMut};

/// Graph is the primary component of the library. It contains the internal structures
/// necessary to manipulate and build complex graphs.
pub struct Gal<T, V, E> {
    data: Vec<Node<V, E>>,
    map: HashMap<T, usize>,
    attr: Gattr,
}

struct Gattr {
    acyclic: Option<bool>,
}

pub struct Node<V, E> {
    val: V,
    edges: Vec<(usize, E)>,
}

impl<T, V, E> Gal<T, V, E>
where
    T: Eq+Hash+Copy+Clone+Debug+?Sized,
    E: Add<Output = E>,
    // E: Add,
{
    /// new returns a new Graph initialized, but empty
    pub fn new() -> Gal<T, V, E> {
        Gal {
            data: Vec::new(),
            map: HashMap::new(),
            attr: Gattr{
                acyclic: None,
            }
        }
    }

    /// add_node inserts a new node into the graph
    pub fn add_node(&mut self, id: T, node: V) -> usize {
        let newnode = Node{
            val: node,
            edges: Vec::new(),
        };
        let idx = self.data.len();
        self.data.push(newnode);
        self.map.insert(id, idx);
        idx
    }

    /// add_edge creates an edge between the origin and destination
    pub fn add_edge(&mut self, origin: T, dest: T, edge: E) {
        if let Some(o) = self.map.get(&origin) {
            if let Some(d) = self.map.get(&dest) {
                self.data[*o].edges.push((*d, edge));
            }
        }
    }

    pub fn nlu(&self, id: T) -> Option<usize> {
        if let Some(n) = self.map.get(&id) {
            return Some(*n);
        }
        None
    }

    pub fn n(&self, id: usize) -> Option<&Node<V, E>> {
        if id < self.data.len() {
            return Some(&self.data[id]);
        }
        None
    }

    /// dfs runs a depth first search through the graph starting at the node
    /// with the given ID
    pub fn dfs(&self, start: T) {
        if let Some(o) = self.map.get(&start) {
            let mut visited = vec![false; self.data.len()];
            self.dfs_util(*o, &mut visited);
        }

    }

    fn dfs_util(&self, start: usize, visited: &mut Vec<bool>) {
        if start >= visited.len() {
            return;
        }
        println!("Visiting: {}", start);
        visited[start] = true;
        for e in &self.data[start].edges {
            if !visited[e.0] {
                self.dfs_util(e.0, visited);
            }
        }

    }
}
