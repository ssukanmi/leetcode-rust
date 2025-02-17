pub mod solutions;

use solutions as sol;

fn main() {
    println!("Hello, world!");
    let input = "hello";
    let score  = sol::score_a_string::score_of_string(input.to_string());
    println!("Score of '{}' is {}", input, score);
}
