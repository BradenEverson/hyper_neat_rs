#[cfg(test)]
mod tests {
    use crate::neat::net::{activation::Activation, ann::ANN, node::Node};

    #[test]
    fn test_forward_no_connections() {
        let ann = ANN::new()
            .with_inputs(2)
            .and_outputs(1);

        assert_eq!(ann.forward(&[1f32,0.5]).unwrap().len(), 1);
        assert_eq!(ann.forward(&[1f32,0.5]).unwrap()[0], 0f32);
    }

    #[test]
    fn test_mismatched_inputs_err() {
        let ann = ANN::new()
            .with_inputs(3)
            .and_outputs(4);
        let forward = ann.forward(&[1i8]);

        assert!(forward.is_err())
    }

    #[test]
    fn test_simple_forward() {
        let mut ann = ANN::new()
            .with_inputs(2)
            .and_outputs(1);

        let inputs = ann.inputs.clone();
        let outputs = ann.outputs.clone();

        for input in inputs.iter() {
            for output in outputs.iter() {
                ann.connect(*input, *output).unwrap();
            }
        }
        let fwrd = ann.forward(&[1u8, 2u8]).unwrap();

        assert_eq!(fwrd[0], 3f32)
    }

    #[test]
    fn test_forward_with_hidden_layer() {
        let mut ann = ANN::new()
            .with_inputs(2)
            .and_outputs(1);

        let middle = Node::inner();

        let middle_node = ann.insert(middle);

        let input1 = ann.inputs[0].clone();
        let input2 = ann.inputs[1].clone();

        let output = ann.outputs[0].clone();

        ann.connect(input1, middle_node).unwrap();
        ann.connect(input2, middle_node).unwrap();

        ann.connect(middle_node, output).unwrap();
        ann.connect(input2, output).unwrap();

        //ANN Structure -> (1) (2)
        //                  \  / |
        //                  (h)  |
        //                    \ /
        //                    (O) essentially (1+2) + 2 because weights are 1 for now
        let fwrd = ann.forward(&[4u8,5u8]).unwrap();
        assert_eq!(fwrd[0], 14f32);


    }

    #[test]
    fn ensure_add() {
        let ann = ANN::new()
            .with_inputs(3)
            .and_outputs(2);

        assert_eq!(ann.nodes.len(), 5)
    }

    #[test]
    fn ensure_species() {
        let ann = ANN::new()
            .with_species(10);

        assert_eq!(ann.species_num(), 10)
    }

    #[test]
    fn ensure_recursive_connect_err() {
        let rec_node = Node::input();

        let mut ann = ANN::new();
        let rec_id = ann.insert(rec_node);

        let result = ann.connect(rec_id, rec_id);

        assert!(result.is_err())
    }

    #[test]
    fn ensure_forward_facing_connections() {
        let inp = Node::input();
        let out = Node::output();

        let mut ann = ANN::new();

        let in_id = ann.insert(inp);
        let out_id = ann.insert(out);

        let result = ann.connect(out_id, in_id);

        assert!(result.is_err())
    }

    #[test]
    fn test_relu() {
        let in1 = -1f32;
        let in2 = 4f32;

        let activation = Activation::RELU;

        assert_eq!(activation.apply(in1), 0f32);
        assert_eq!(activation.apply(in2), 4f32);
    }
}
