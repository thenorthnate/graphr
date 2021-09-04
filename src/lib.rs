// Graphr

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::ops::Add;

pub trait Traveler<V, E> {
    fn visit(&mut self, node: V);
    fn transit(&mut self, edge: E);
}

/// Graph is the primary component of the library. It contains the internal structures
/// necessary to manipulate and build complex graphs.
pub struct Graph<T, V, E> {
    data: Vec<Node<V, E>>,
    map: HashMap<T, usize>,
}

struct Node<V, E> {
    val: V,
    edges: Vec<(usize, E)>,
}

impl<T, V, E> Graph<T, V, E>
where
    T: Eq+Hash+Copy+Clone+Debug,
    E: Add<Output = E>,
    // E: Add,
{
    /// new returns a new Graph initialized, but empty
    pub fn new() -> Graph<T, V, E> {
        Graph {
            data: Vec::new(),
            map: HashMap::new(),
        }
    }

    /// add_node inserts a new node into the graph
    pub fn add_node(&mut self, id: T, node: V) {
        let newnode = Node{
            val: node,
            edges: Vec::new(),
        };
        let idx = self.data.len();
        self.data.push(newnode);
        self.map.insert(id, idx);
    }

    /// add_edge creates an edge between the origin and destination
    pub fn add_edge(&mut self, origin: T, dest: T, edge: E) {
        if let Some(o) = self.map.get(&origin) {
            if let Some(d) = self.map.get(&dest) {
                self.data[*o].edges.push((*d, edge));
            }
        }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_graph() {
        let mut g = Graph::new();
        g.add_node("n0", 0);
        g.add_node("n1", 1);
        g.add_node("n2", 2);

        g.add_edge("n0", "n1", 0.5);
        g.add_edge("n1", "n2", 0.7);

        g.dfs("n0");
    }
}
