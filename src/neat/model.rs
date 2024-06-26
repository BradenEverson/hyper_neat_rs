use rand::Rng;

use crate::neat::net::ann::ANN;

use super::{ fitness::{Fitness, FitnessFn}, net::{error::{AnnError, Result}, initializer::Initializer}, simple_ann::SimpleANN};

pub struct Population {
    generation: Vec<SimpleANN>,
    fitness: FitnessFn,
    population_size: usize,
    survivor_percentage: f32,
    mutation_power: f32,
    node_add_rate: f32,
    connect_rate: f32,
    weight_rate: f32,
    initializer: Initializer,
    seed: Option<String>,
    inputs: usize,
    outputs: usize,
    dbg: bool
}

impl Default for Population {
    fn default() -> Self {
        Population::new()
            .with_inputs_and_outputs(2, 1)
            .with_add_rate(0.05)
            .with_connect_rate(0.05)
            .with_weight_rate(0.05)
            .top_n_percent_survive(0.1)
            .population_size(100)
            .with_mutation_power(0.1)
    }
}

impl Population {
    pub fn new() -> Self {
        Population { 
            generation: vec![],
            fitness: Fitness::placeholder(), 
            node_add_rate: 0f32,
            connect_rate: 0f32,
            weight_rate: 0f32,
            inputs: 0, 
            initializer: Initializer::Normal,
            outputs: 0,
            population_size: 0,
            survivor_percentage: 0f32,
            dbg: false,
            mutation_power: 0f32,
            seed: None
        }
    }

    pub fn get_best(&self) -> SimpleANN {
        self.generation[0].clone()
    }

    pub fn with_mutation_power(mut self, mut_frac: f32) -> Self {
        self.mutation_power = mut_frac;

        self
    }

    pub fn with_inputs_and_outputs(mut self, inputs: usize, outputs: usize) -> Self {
        self.inputs = inputs;
        self.outputs = outputs;

        self
    }

    pub fn with_seed(mut self, seed: String) -> Self {
        self.seed = Some(seed);

        self
    }

    pub fn set_fitness(&mut self, fitness: FitnessFn) {
        self.fitness = fitness;
    }
    
    pub fn with_add_rate(mut self, add_rate: f32) -> Self {
        self.node_add_rate = add_rate;
        
        self
    }
    pub fn with_weight_rate(mut self, weight_rate: f32) -> Self {
        self.weight_rate = weight_rate;

        self
    }
    pub fn with_connect_rate(mut self, con_rate: f32) -> Self {
        self.connect_rate = con_rate;
    
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

    fn survival_of_the_fittest(&mut self) {
        let lucky_few = (self.population_size as f32 * self.survivor_percentage).round() as usize;

        self.generation = self.generation[0..lucky_few].to_vec();
    }
    
    fn cross_breed(&mut self) {
        let mut new_gen = vec![];
        let mut rng = Initializer::get_rng(&self.seed);

        for _ in 0..self.population_size {
            let parent1 = &self.generation[rng.gen_range(0..self.generation.len())];
            let parent2 = &self.generation[rng.gen_range(0..self.generation.len())];

            let child = parent1.cross(parent2);
            new_gen.push(child);
        }

        self.generation = new_gen;
    }

    fn mutate_newgen(&mut self) {
        let mut rng = Initializer::get_rng(&self.seed);

        for member in self.generation.iter_mut() {
            let mut split_queue = vec![];
            for curr_edge in member.edges.iter_mut() {
                //Weight check
                if rng.gen_bool(self.weight_rate as f64) {
                    /*if self.dbg {
                        println!("\tUpdating Weight on Edge: {} to {}", curr_edge.from, curr_edge.to);
                    }*/
                    curr_edge.update_weight(curr_edge.weight + 
                        rng.gen_range(-self.mutation_power..=self.mutation_power));
                }
                //New node check
                if rng.gen_bool(self.node_add_rate as f64) {
                    split_queue.push(curr_edge.innovation);
                }
            }

            for innov in split_queue {
                /*if self.dbg {
                    println!("\t\tNew Node Split");
                }*/
                member.split_edge(innov);
            }

            //New connections check 
            for i in 0..(member.nodes.len() - member.dims[member.dims.len() - 1]) {
                if rng.gen_bool(self.connect_rate as f64) {
                    let from = member.nodes[i];
                    let to = member.nodes[rng.gen_range((i + 1)..member.nodes.len())];
                    let weight = self.initializer.sample(&mut Initializer::get_rng(&self.seed));

                    /*if self.dbg {
                        println!("\t\t\tNew Connection: {} to {}", from, to);
                    }*/

                    member.insert((from, to, weight));
                }
            }
        }
    }

    pub fn evolve(&mut self, start_conditions: &[f32]) -> Result<()> {
        self.rank(start_conditions)?;

        if self.dbg {
            //Gotta reimpl display
            //println!("Fittest Genome: {}\nFitness: {}", self.generation[0], (self.fitness)(&self.generation[0], start_conditions));
            println!("Best Fitness: {}", (self.fitness)(&self.generation[0], start_conditions));
        }

        self.survival_of_the_fittest();

        self.cross_breed();
        self.mutate_newgen();

        Ok(())
    }

}