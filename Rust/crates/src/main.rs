use rand::{self, RngExt};

mod calculation;
mod messages;

fn main() {
    let mut rng = rand::rng();

    messages::greet("Addison");

    let a: i32 = rng.random_range(1..1000);
    let b: i32 = rng.random_range(1..1000);
    println!("\nRandom numbers, a: {} b: {}", a, b);

    let result = calculation::add(a, b);
    println!("\nAdds: {} + {} = {}", a, b, result);

    let result = calculation::subtract(a, b);
    println!("Subtract: {} - {} = {}\n", a, b, result);

    messages::farewell("Addison");
}
