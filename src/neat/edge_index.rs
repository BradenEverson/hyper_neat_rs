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
        self.edges.iter().any(|edge| edge.innovation == innov_num)
    }

    pub fn split_edge(&mut self, target_innov: usize)  {
        let middle = self.add_node();
        let target = self[target_innov];
        let from = target.from;
        let to = target.to;
        let weight = target.weight;

        let idx = match self.edges.iter().enumerate().find(|edge| edge.1.innovation == target_innov) {
            Some((id, _)) => id,
            None => 0
        };

        self.edges.remove(idx);

        self.insert((from, middle, weight));
        self.insert((middle, to, 1f32));
    }
}