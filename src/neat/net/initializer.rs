pub enum Initializer {
    Normal,
    Uniform
}

impl Initializer {
    pub fn gen(&self, seed: &Option<String>) -> f32 {
        if let Some(seed_str) = seed {
            println!("{}", seed_str);
        } else {
            println!("No seed");
        }
        3f32
    }
}