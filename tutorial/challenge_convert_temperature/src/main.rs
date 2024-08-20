fn main() {
    println!("The temperature is {}", celsius_to_farhenheit(0.0));
    println!("The temperature is {}", celsius_to_farhenheit(22.0));
}

fn celsius_to_farhenheit( temp_c: f64) -> f64 {
    1.8 * temp_c + 32.0
}