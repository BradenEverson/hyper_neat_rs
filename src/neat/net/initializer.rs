use rand::{Rng, RngCore};
use rand_seeder::Seeder;
use rand_pcg::Pcg64;
use rand_distr::{Distribution, Normal};
pub enum Initializer {
    Normal,
    Uniform
}

impl Initializer {
    pub fn get_rng(seed: &Option<String>) -> Box<dyn RngCore> {
        match seed {
            Some(inner) => Box::new(Seeder::from(inner).make_rng::<Pcg64>()),
            None => Box::new(rand::thread_rng())
        }
    }
    pub fn gen_range(&self, seed: &Option<String>, to: usize) -> Vec<f32> {
        let mut res = vec![];
        let mut rng = Initializer::get_rng(seed);

        for _ in 0..to {
            res.push(self.sample(&mut rng));
        }

        res
    }
    pub fn sample(&self, rng: &mut Box<dyn RngCore>) -> f32 {
        match self {
            Initializer::Normal => Normal::new(0f32, 1f32).unwrap().sample(rng),
            Initializer::Uniform => rng.gen_range(-3f32..3f32)
        }
    }
}