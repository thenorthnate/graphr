// Graphr

use std::ops::{Index, IndexMut};
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::ops::Add;

// Modules
pub mod default;
pub mod error;

pub trait Graph<T, V, E, Output = V>: Index<T>+IndexMut<T>
where
    T: Eq+Hash+Copy+Clone+Debug+?Sized,
    E: Add<Output = E>
{
    fn size(&self) -> usize;
    fn clear(&mut self);
    fn add_node(&mut self, name: T, value: V) -> T;
    /// del_node returns a mapping of old node names to new names based on a shift when
    /// the named node was removed
    fn del_node(&mut self, name: T) -> HashMap<T, T>;
    fn add_edge(&mut self, origin: T, dest: T, value: E) -> Result<(), error::NodeError>;
    fn get_edge(&self, origin: T, dest: T) -> Option<&E>;
    fn get_edges(&self, origin: T) -> Vec<T>;
}

// pub trait Graph<T, V, E>: Index<T>+IndexMut<T> {
//     fn node(&self, id: T) -> dyn Node<V, E>;
// }

// pub trait Node<V, E> {
//     fn eit(&self) -> dyn Iterator<Item = E>;
// }

// pub trait Edge<E> {

// }



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_graph() {
        // Basic operations to build a simple graph
        let mut g = default::Gal::new();
        g.add_node("n0", 0);
        g.add_node("n1", 1);
        g.add_node("n2", 2);

        g.add_edge("n0", "n1", 0.5);
        g.add_edge("n1", "n2", 0.7);

        // Easily traverse a graph
        // let mut w = Walker::new(&g);
        // w.dfs()
        // for node in w { ... }

        g.dfs("n0");
    }
}
