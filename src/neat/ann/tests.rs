#[cfg(test)]
mod tests {
    use crate::neat::ann::{ann::ANN, node::Node};

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
}
