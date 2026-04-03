fn main() {
    let mut value = 9;
    value = 10;

    println!("The value is {}", value);

    let x = 66;
    let x = x + 1; // Shadowing
    let x = "Rust programming";
    println!("The value is {}", x);

    let small_value: i8 = 100;
    let simple_float: f32 = -700.25;
    println!("The small_value is {}", small_value);
    println!("The simple_float is {}", simple_float);

    // Arrays
    let numbers = [1, 2, 3, 4, 5];
    println!("Element at index 0: {}", numbers[0]);

    // Tuples
    let person = ("Alice", 30, 5.4);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Height: {}", person.2);

    // Constants
    const PI: f32 = 3.14;
    println!("PI = {}", PI);

    // Constants vs Immutable variable
    // You have to specify the data type
    // Don't suports mutable
    // Cannot be shadowed

    // const PI: f32 = 6.28; <-- Error

    // Strings
    let mut name = String::from("Zenva");
    // let mut name = "Zenva"; <-- This is a String Slices (&str) no a String
    name.push_str(" Academy");
    println!("{}", name);

    // Use &str when you have a string literal or when you do not need to own the data.
    // Use String when you require a string that you can modify or if you need ownership of the data.
}
