
#[derive(Clone, Copy)]
pub(super) struct SimpleEdge {
    from: usize,
    to: usize,
    weight: f32,
    innovation: usize
}


impl From<(usize, usize, f32)> for SimpleEdge {
    fn from(value: (usize, usize, f32)) -> Self {
        SimpleEdge::new(value.0, value.1, value.2)
    }
}

impl Into<(usize, usize, usize, f32)> for SimpleEdge {
    fn into(self) -> (usize, usize, usize, f32) {
        (self.innovation, self.from, self.to, self.weight)
    }
}

impl SimpleEdge {
    pub fn new(from: usize, to: usize, weight: f32) -> Self {
        let mut innovation = SimpleEdge::get_innov(from, to);

        SimpleEdge { from, to, weight, innovation }
    }

    fn get_innov(from: usize, to: usize) -> usize {
        let rate = f32::ceil(f32::log10(to as f32));
        
        (from * f32::powf(10f32, rate) as usize) + to
    }
}