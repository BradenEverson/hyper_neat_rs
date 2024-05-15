use std::ops::{Index, IndexMut};

use super::{simple_ann::SimpleANN, simple_edge::SimpleEdge};

impl Index<usize> for SimpleANN {
    type Output = SimpleEdge;
    fn index(&self, index: usize) -> &Self::Output {
        match self.edges.iter().find(|edge| edge.innovation == index) {
            Some(edge) => edge,
            None => panic!("Innovation number not found in simple edge")
        }
    }
}

impl IndexMut<usize> for SimpleANN {
    fn index_mut(&mut self, index: usize) -> &mut SimpleEdge {
        match self.edges.iter_mut().find(|edge| edge.innovation == index) {
            Some(edge) => edge,
            None => panic!("Innovation number not found in simple edge")
        }
    }
}

impl SimpleANN {
    pub fn contains(&self, innov_num: usize) -> bool {
        self.edges.iter().find(|edge| edge.innovation == innov_num).is_some()
    }
}