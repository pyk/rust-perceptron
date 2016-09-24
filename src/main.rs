
// Logistic-sigmoid function
fn activate_sigmoid(slope: f64, x: f64) -> f64 {
    let e: f64 = std::f64::consts::E;
    return 1.0/(1.0 + e.powf(-0.5 * slope * x));
}

fn main() {
    // Define data
    let x: Vec<[f64; 3]> = vec![
        [1.0, 1.0, 1.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [1.0, 1.0, 0.0]
    ];

    let bias: f64 = 1.0;
    let weights: [f64; 4] = [0.6, 0.2, 0.4, 0.5];

    // TODO: forward data to the perceptron
    for xi in x.iter() {
        let test: [f64; 4] = weights;
        let mut sum: f64 = 0.0;
        for (index, value) in xi.iter().enumerate() {
            let result: f64;
            if index == 0 {
                result = test[index] * bias;
            } else {
                result = test[index] * value;
            }
            sum += result;
        }
        println!("Weighted sum: {}", sum);
        // TODO: run the activation function
        let result: f64 = activate_sigmoid(1.0, sum);
        println!("Activation function: {}", result);

        // TODO: Implement the perceptron learning algorithm
    }

    // Bias
    println!("Bias: {}", bias);

    // Weights
    println!("Weights:");
    for w in weights.iter() {
        println!("{}", w);
    }
}

