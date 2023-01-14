use std::io;
// main fn is the program entry point
// Rust uses snake case
fn main() {
    loop {
        println!("=== Mars Weight Calculator ===");

        // Read input and assign it to a mutable variable
        // using io::stdin().read_line()
        println!("Input your current weight (kg): ");
        let mut input = String::new();
        // Unwrap yield an Error or an Result
        // may panic, better to use matching for unrecoverable errors
        io::stdin().read_line(&mut input).unwrap();

        let weight: f32 = match input.trim().parse() {
            Ok(weight) => weight,
            Err(e) => {
                println!("Error: {}. Input a valid number!", e);
                continue;
            }
        };

        let weight_on_mars = calc_weight_on_mars(weight);
        // println! macro, meta programming concept
        println!("Your weight on mars would be {}kg", weight_on_mars);
    }
}

fn calc_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
