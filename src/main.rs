use neater::neat::simple_ann::SimpleANN;

fn main() {
    let ann = SimpleANN::new(
        &[3,2,1], 
        &[0,1,2,3,4,5], 
        &[(0, 0,3,0.5), (1, 1,3,-0.3),(2, 2,3,0.1),(3, 3,4,0.8)]); 

    println!("{}", ann);
}