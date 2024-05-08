use super::simple_ann::SimpleANN;

pub struct Fitness;

impl Fitness {
    pub fn default() -> Box<dyn Fn(&SimpleANN, &[f32]) -> f32> {
        Box::new(|net, inputs| net.forward(inputs).unwrap().iter().sum())
    }
}