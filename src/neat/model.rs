use crate::neat::net::ann::ANN;

use super::{ fitness::Fitness, net::{initializer::Initializer, error::{Result, AnnError}}, simple_ann::SimpleANN};

pub struct Population {
    generation: Vec<SimpleANN>,
    fitness: Box<dyn Fn(&SimpleANN, &[f32]) -> f32>,
    population_size: usize,
    survivor_percentage: f32,

    node_add_rate: f32,
    node_rem_rate: f32,
    connect_rate: f32,
    disconnect_rate: f32,
    initializer: Initializer,

    inputs: usize,
    outputs: usize,
    dbg: bool
}

impl Default for Population {
    fn default() -> Self {
        Population::new()
            .with_inputs_and_outputs(2, 1)
            .with_add_rate(0.05)
            .with_delete_rate(0.01)
            
            .with_connect_rate(0.05)
            .with_disconnect_rate(0.05)
            .top_n_percent_survive(0.1)
            .population_size(100)
    }
}

impl Population {
    pub fn new() -> Self {
        Population { 
            generation: vec![],
            fitness: Fitness::default(), 
            node_add_rate: 0f32,
            node_rem_rate: 0f32,
            connect_rate: 0f32,
            disconnect_rate: 0f32,
            inputs: 0, 
            initializer: Initializer::Normal,
            outputs: 0,
            population_size: 0,
            survivor_percentage: 0f32,
            dbg: false
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
    pub fn population_size(mut self, pop: usize) -> Self {
        self.population_size = pop;

        self
    }

    pub fn top_n_percent_survive(mut self, top: f32) -> Self {
        self.survivor_percentage = top;

        self
    }

    pub fn toggle_debug(&mut self) {
        self.dbg = !self.dbg
    }

    pub fn init(&mut self) {
        for _ in 0..self.population_size {
            let mut temp_ann = ANN::new().with_inputs(self.inputs).and_outputs(self.outputs);
            temp_ann.init(&self.initializer);
            self.generation.push(temp_ann.into());
        }
    }

    fn rank(&mut self, initial_inputs: &[f32]) -> Result<()> {
        if initial_inputs.len() != self.inputs {
            Err(AnnError::MismatchedInputSizeError(initial_inputs.len(), self.inputs))
        } else {
           //Sort list by fitness in descending order
            self.generation.sort_by(|a, b| 
                (self.fitness)(b, initial_inputs)
                    .partial_cmp(&(self.fitness)(a, initial_inputs)).unwrap());
            Ok(())
        }
    }

    pub fn survival_of_the_fittest(&mut self) {
        let lucky_few = (self.population_size as f32 * self.survivor_percentage).round() as usize;

        self.generation = self.generation[0..lucky_few].to_vec();
    }
    
    pub fn cross_breed(&mut self) {

    }

    pub fn evolve(&mut self, start_conditions: &[f32]) -> Result<()> {
        self.rank(start_conditions)?;

        if self.dbg {
            println!("Fittest Genome: {}\nFitness: {}", self.generation[0], (self.fitness)(&self.generation[0], start_conditions));
        }

        self.survival_of_the_fittest();

        self.cross_breed();

        Ok(())
    }

}

pub enum MutationType {
    NewNode,
    NewConnection,
    KillConnection
}