#[cfg(test)]
mod tests {
    use crate::neat::{net::ann::ANN, simple_ann::SimpleANN};

    #[test]
    fn test_forward_no_connections() {
        let ann: SimpleANN = ANN::new()
            .with_inputs(2)
            .and_outputs(1).into();

        assert_eq!(ann.forward(&[1f32,0.5]).unwrap().len(), 1);
        assert_eq!(ann.forward(&[1f32,0.5]).unwrap()[0], 0f32);
    }
}