use crate::neat::net::{ann::ANN, edge::Edge, node::NodeId};

use super::error::{NeatError, Result};

pub struct ProblemSet {
    population: Vec<NeatNet>,
    fitness: Box<dyn Fn(ANN) -> f32>
}

pub struct NeatNet {
    genome: ANN,
    nodes: Vec<NodeId>,
    edges: Vec<Edge>
}

impl Default for NeatNet {
    fn default() -> Self {
        NeatNet::empty()
    }
}

impl NeatNet {
    pub fn empty() -> Self {
        NeatNet { genome: ANN::new(), nodes: vec![], edges: vec![] }
    }

    pub fn mutate(&mut self, mutation: MutationType) {

    }

    fn get_species(&self) -> u32 {
        self.genome.species_num()
    }

    pub fn cross_breed(&mut self, partner: &mut NeatNet) -> Result<NeatNet> {
        if self.get_species() != partner.get_species() {
            Err(NeatError::IncompatibleSpeciesBreedError)
        } else {
            todo!();
            Ok(NeatNet::default())
        }
    }

}

pub enum MutationType {
    NewNode,
    NewConnection,
    KillConnection
}