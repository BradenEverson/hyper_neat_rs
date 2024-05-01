#[cfg(test)]
mod tests {
    use crate::neat::ann::ann::ANN;

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
}
