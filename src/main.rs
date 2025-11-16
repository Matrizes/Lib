use ndarray::*;   

fn main() {
    let a = arr1(&[30.0]);

    let a_negative = negative(a);

    println!("{}", a_negative);
}

// Logic

fn negative(value: Array1<f32>) -> f32 {

    arr1(&[-1.0]).dot(&value)
}



