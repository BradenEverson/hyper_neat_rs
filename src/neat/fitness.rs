use super::{net::ann::ANN, simple_ann::SimpleANN};

pub struct Fitness;

impl Fitness {
    pub fn default<K: Into<SimpleANN>>() -> Box<dyn Fn(K, &[f32]) -> f32> {
        Box::new(|net, inputs| net.into().forward(inputs).unwrap().iter().sum())
    }
}