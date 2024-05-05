use super::net::ann::ANN;

pub struct Fitness;

impl Fitness {
    pub fn default() -> Box<dyn Fn(ANN, &[f32]) -> f32> {
        Box::new(|net, inputs| net.forward(inputs).unwrap().iter().sum())
    }
}