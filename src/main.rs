// TODO: move this to a separate file once it becomes too large
struct Neuron {
    bias: f64,
    weights: Vec<f64>,
    inputs: Vec<f64>,
    output: f64
}

fn main() {
    println!("Hello, world! karat is a Rust library for machine learning.");

    let singleNeuron = Neuron{
        bias: 1.0,
        weights: vec![],
        inputs: vec![],
        output: 0.0
    };
}
