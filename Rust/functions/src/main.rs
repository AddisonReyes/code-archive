use std::io;

fn greet_user(name: String) {
    println!("Hello {}, welcome to Rust Programming!", name);
}

fn calculate_sum(a: i32, b: i32) -> i32 {
    let sum = a + b; // Statments
    sum // Return -> expressions
}

fn get_data(label: &str, mut data: String) -> Option<usize> {
    println!("{label}");
    io::stdin()
        .read_line(&mut data)
        .expect("Failed to read line.");

    let data: Option<usize> = match data.trim().parse() {
        Ok(len) => Some(len),
        Err(_) => None,
    };

    data
}

fn vec_gen(value: u8, length: usize) -> Vec<u8> {
    let arr: Vec<u8> = vec![value; length];
    arr
}

fn main() {
    let name = String::from("Addison");
    greet_user(name);

    let a = 5;
    let b = 10;
    let sum = calculate_sum(a, b);
    println!("{} + {} = {}\n", a, b, sum);

    let y = {
        let x = 66;
        x + 1
    };

    println!("The value of \'y\' is: {y}");

    loop {
        let length = String::new();
        let length = match get_data("\nPlease enter an array length: ", length) {
            Some(length) => length,
            None => continue,
        };

        let value = String::new();
        let value = match get_data("\nPlease enter a default value: ", value) {
            Some(value) => value,
            None => continue,
        };

        let arr = vec_gen(value as u8, length);
        println!("\n({length}) Array: {:?}", arr);
        break;
    }
}
