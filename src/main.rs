use ndarray::*;   

fn main() {
    let a = arr2(&[[0.0]]);

    let a_negative = negative(a);

    println!("{}", a_negative);
}

// Logic

fn negative(value: Array2<f32>) -> Array2<f32> {
    arr2(&[[-1.0]]).dot(&value) + arr2(&[[1.0]])
}

fn and(value1: Array2<f32>, value2: Array2<f32>) -> Array2<f32> {
    value1.dot(&value2)
}

fn or(value1: Array2<f32>, value2: Array2<f32>) -> Array2<f32> {
    &value1 + &value2 - value1.dot(&value2)
}