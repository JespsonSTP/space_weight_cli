use std::io;

fn main() {
    println!("How much do you weight");
    let mut user_weight = String::new();
    io::stdin().read_line(&mut user_weight);
    println!("Wich Planet you want to pick");
    let mut user_planet = String::new();
    io::stdin().read_line(&mut user_planet);
    println!("Your weight on mars {}kg", calculate_weight_in_space(100.0));
}

fn calculate_weight_in_space(weight: f32) -> f32 {
   let marsweight =  weight_mars(weight);
   return marsweight;
}

fn weight_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}