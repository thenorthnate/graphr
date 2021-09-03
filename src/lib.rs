// Graphr

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

pub struct Graph<T, V, E> {
    data: Vec<Node<V, E>>,
    map: HashMap<T, usize>,
}

struct Node<V, E> {
    val: V,
    edges: Vec<(usize, E)>,
}

impl<T: Eq+Hash+Copy+Clone+Debug, V, E> Graph<T, V, E> {
    pub fn new() -> Graph<T, V, E> {
        Graph {
            data: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: T, node: V) {
        let newnode = Node{
            val: node,
            edges: Vec::new(),
        };
        let idx = self.data.len();
        self.data.push(newnode);
        self.map.insert(id, idx);
    }

    pub fn add_edge(&mut self, origin: T, dest: T, edge: E) {
        if let Some(o) = self.map.get(&origin) {
            if let Some(d) = self.map.get(&dest) {
                self.data[*o].edges.push((*d, edge));
            }
        }
    }

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
