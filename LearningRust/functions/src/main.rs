fn main() {
    let name = String::from("Addison");
    greet_user(name);

    let a = 5;
    let b = 10;
    let sum = calculate_sum(a, b);
    println!("{} + {} = {}", a, b, sum)
}

fn calculate_sum(a: i32, b: i32) -> i32 {
    let sum = a + b; // Statments
    sum // Return -> expressions
}

fn greet_user(name: String) {
    println!("Hello {}, welcome to Rust Programming!", name);
}
