use super::simple_ann::SimpleANN;

pub struct Fitness;

pub type FitnessFn = Box<dyn Fn(&SimpleANN, &[f32]) -> f32>;

impl Fitness {
    pub fn placeholder() -> FitnessFn {
        Box::new(|net, inputs| net.forward(inputs).unwrap().iter().sum())
    }
}