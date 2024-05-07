use crate::neat::net::{ann::ANN, edge::Edge, node::NodeId};

use super::{error::{NeatError, Result}, fitness::Fitness, net::initializer::Initializer, simple_ann::SimpleANN};

pub struct Population<K: Into<SimpleANN>> {
    generation: Vec<SimpleANN>,
    fitness: Box<dyn Fn(K, &[f32]) -> f32>,
    max_species: u32,
    population_size: u64,

    node_add_rate: f32,
    node_rem_rate: f32,
    connect_rate: f32,
    disconnect_rate: f32,
    initializer: Initializer,

    inputs: usize,
    outputs: usize
}

impl<K: Into<SimpleANN>> Default for Population<K> {
    fn default() -> Self {
        Population::new()
            .with_inputs_and_outputs(2, 1)
            .with_add_rate(0.05)
            .with_delete_rate(0.01)
            
            .with_connect_rate(0.05)
            .with_disconnect_rate(0.05)
            .population_size(100)
    }
}

impl<K: Into<SimpleANN>> Population<K> {
    pub fn new() -> Self {
        Population { generation: vec![],
            fitness: Fitness::default(), 
            node_add_rate: 0f32,
            node_rem_rate: 0f32,
            connect_rate: 0f32,
            disconnect_rate: 0f32,
            inputs: 0, 
            initializer: Initializer::Normal,
            outputs: 0,
            max_species: 0,
            population_size: 0
        }
    }
    pub fn with_inputs_and_outputs(mut self, inputs: usize, outputs: usize) -> Self {
        self.inputs = inputs;
        self.outputs = outputs;

        self
    }
    
    pub fn with_add_rate(mut self, add_rate: f32) -> Self {
        self.node_add_rate = add_rate;
        
        self
    }
    pub fn with_delete_rate(mut self, rem_rate: f32) -> Self {
        self.node_rem_rate = rem_rate;

        self
    }
    pub fn with_connect_rate(mut self, con_rate: f32) -> Self {
        self.connect_rate = con_rate;
    
        self
    }
    pub fn with_disconnect_rate(mut self, disc_rate: f32) -> Self {
        self.disconnect_rate = disc_rate;

        self
    }

    pub fn with_initializer(mut self, init: Initializer) -> Self {
        self.initializer = init;

        self
    }
    pub fn population_size(mut self, pop: u64) -> Self {
        self.population_size = pop;

        self
    }

    /*pub fn assess_fitness(&self, inputs: &[f32]) -> Vec<f32> {
        let mut res = vec![];
        for elem in self.generation {
            res.push((self.fitness)(elem, inputs))
        }
        res
    }*/

    pub fn init(&mut self) {
        for _ in 0..self.population_size {
            let mut temp_ann = ANN::new().with_inputs(self.inputs).and_outputs(self.outputs);
            temp_ann.init(&self.initializer);
            self.generation.push(temp_ann.into());
        }
    }

}

pub enum MutationType {
    NewNode,
    NewConnection,
    KillConnection
}