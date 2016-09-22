
fn main() {
    // Define data
    let x: Vec<[f32; 3]> = vec![
        [1.0, 1.0, 1.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [1.0, 1.0, 0.0]
    ];

    let bias: f32 = 1.0;
    let weights: [f32; 4] = [0.6, 0.2, 0.4, 0.5];

    // TODO: forward data to the perceptron
    for xi in x.iter() {
        let mut sum: f32 = 0.0;
        for (index, value) in xi.iter().enumerate() {
            let result: f32;
            if index == 0 {
                result = weights[index] * bias;
            } else {
                result = weights[index] * value;
            }
            sum += result;
        }
        println!("Weighted sum: {}", sum);
        // TODO: run the activation function
    }

    // Weights
    println!("Weights:");
    for w in weights.iter() {
        println!("{}", w);
    }
}

