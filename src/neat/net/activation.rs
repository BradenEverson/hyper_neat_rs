use std::f32::consts::E;


#[derive(Debug, Clone)]
pub enum Activation {
    SIGMOID,
    TANH,
    RELU,
    SIN,
    COS
}

impl Activation {
    pub fn apply(&self, curr_val: f32) -> f32 {
        match self {
            Self::COS => f32::cos(curr_val),
            Self::SIN => f32::sin(curr_val),
            Self::RELU => relu(curr_val),
            Self::TANH => f32::tanh(curr_val),
            Self::SIGMOID => sigmoid(curr_val)
        }
    }
}

fn relu(inp: f32) -> f32 {
    (!(inp < 0f32) as i8 as f32) * inp
}

fn sigmoid(inp: f32) -> f32 {
    1f32 / (1f32 + E.powf(-1f32 * inp))
}