
#[derive(Clone, Copy)]
pub struct SimpleEdge {
    pub from: usize,
    pub to: usize,
    pub weight: f32,
    pub innovation: usize
}


impl From<(usize, usize, f32)> for SimpleEdge {
    fn from(value: (usize, usize, f32)) -> Self {
        SimpleEdge::new(value.0, value.1, value.2)
    }
}

impl From<SimpleEdge> for (usize, usize, usize, f32) {
    fn from(val: SimpleEdge) -> Self {
        (val.innovation, val.from, val.to, val.weight)
    }
}

impl SimpleEdge {
    pub fn new(from: usize, to: usize, weight: f32) -> Self {
        let innovation = SimpleEdge::get_innov(from, to);

        SimpleEdge { from, to, weight, innovation }
    }

    fn get_innov(from: usize, to: usize) -> usize {
        let rate = f32::ceil(f32::log10(to as f32));
        
        (from * f32::powf(10f32, rate) as usize) + to
    }
    pub fn update_weight(&mut self, new_weight: f32) {
        self.weight = new_weight
    }
}