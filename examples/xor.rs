use neater::neat::{model::Population, net::error::Result, simple_ann::SimpleANN};


fn main() -> Result<()> {
    let mut model = Population::default()
        .with_add_rate(0.2)
        .with_inputs_and_outputs(3, 1);

    let fitness = |model: &SimpleANN, _: &[f32]| {
        let inputs = [[0f32,0f32, 1f32],[0f32,1f32, 1f32],[1f32,0f32, 1f32],[1f32,1f32, 1f32]];
        let outputs = [0f32, 1f32, 1f32, 0f32];

        let mut total_error = 0f32;

        for i in 0..inputs.len() {
            let prediciton = model.forward(&inputs[i]).unwrap();
            let error = (prediciton[0] - outputs[i]).powi(2);

            total_error += error;
        }

        let mse = total_error / outputs.len() as f32;

        let fitness = 1f32 / mse + f32::EPSILON;
        fitness
    };

    model.set_fitness(Box::new(fitness));

    model.init();
    model.toggle_debug();

    for _ in 0..10 {
        model.evolve(&[0f32, 0f32, 1f32])?;
    }

    let best = model.get_best();

    println!("[0, 0] -> {}", best.forward(&[0f32,0f32,1f32])?[0]);
    println!("[1, 0] -> {}", best.forward(&[1f32,0f32,1f32])?[0]);
    println!("[0, 1] -> {}", best.forward(&[0f32,1f32,1f32])?[0]);
    println!("[1, 1] -> {}", best.forward(&[1f32,1f32,1f32])?[0]);

    Ok(())
}