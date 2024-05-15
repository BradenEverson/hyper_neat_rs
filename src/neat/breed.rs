use std::{cmp::Ordering, ops::IndexMut};

use rand::Rng;

use super::simple_ann::SimpleANN;

impl SimpleANN {
    /// Crossbreed two SimpleANNs by their genes, crossing over edge weights
    /// Topology of 'self' is preserved, for our NEAT impl this makes sure that the
    /// fittest topology is kept
    /// 
    /// All intersected innovation numbers are then iterated over and the new weight in the child
    /// is decided randomly between the two parent's
    pub fn cross(&self, partner: &SimpleANN) -> SimpleANN {
        let mut child = self.clone();
        let mut basic_rng = rand::thread_rng();

        let union = self.edges.iter().filter(|e| partner.contains(e.innovation));

        for edge in union.map(|e| e.innovation) {
            child.index_mut(edge).update_weight(match basic_rng.gen_range(-1..1).cmp(&0) {
                Ordering::Equal | Ordering::Greater => self[edge].weight,
                _ => partner[edge].weight
            });
        }
        child
    }
}