#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::neat::{net::{ann::ANN, initializer::Initializer}, simple_ann::SimpleANN};

    #[test]
    fn test_forward_no_connections() {
        let ann: SimpleANN = ANN::new()
            .with_inputs(2)
            .and_outputs(1).into();

        assert_eq!(ann.forward(&[1f32,0.5]).unwrap().len(), 1);
        assert_eq!(ann.forward(&[1f32,0.5]).unwrap()[0], 0f32);
    }

    #[test]
    fn test_speedier_than_norm() {

        let mut ann = ANN::new()
            .with_inputs(100)
            .and_outputs(10);

        //TODO: Adding connections doesn't happen in the right order for SimpleANN, nodenotfound error returned
        //let inputs = ann.inputs.clone();
        //let outputs = ann.outputs.clone();

        /*for input in inputs.iter() {
            for output in outputs.iter() {
                ann.connect(*input, *output).unwrap();
            }
        }*/

        ann.init(&Initializer::Uniform);

        let ann_simp: SimpleANN = ann.clone().into();

        let input = [1u8; 100];

        let start = Instant::now();
        let _ = ann.forward(&input).unwrap();
        let time_norm = Instant::now().duration_since(start);

        let start_simple = Instant::now();
        let _ = ann_simp.forward(&input).unwrap();
        let end_simple = Instant::now().duration_since(start_simple);

        assert!(time_norm.cmp(&end_simple).is_gt());
    }
}