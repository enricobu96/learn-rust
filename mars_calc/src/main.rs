use std::io;

fn main() {
    let mut inp: String = String::new();
    io::stdin().read_line(&mut inp);

    let mut mars_weight: f32 = calculate_weight(inp);
    mars_weight = mars_weight * 1000.0;
    println!("weight on mars = {} g", mars_weight);
}


fn calculate_weight(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}