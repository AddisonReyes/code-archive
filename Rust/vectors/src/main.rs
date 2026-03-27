fn main() {
    // Vec<T>
    // Heap-allocated

    let mut numbers: Vec<i32> = Vec::new();

    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    println!("{:?}", numbers);

    // Creating a vector using vector macro
    let mut fruits = vec!["apple", "banana", "orange"];

    fruits.push("grape");
    println!("\n{:?}", fruits);

    let removed_fruit = fruits.pop();
    println!("{:?}, Removed: {:?}", fruits, removed_fruit);

    // Accessing and modifying elements
    let second = fruits[1];
    println!("The second element is: {}", second);

    match fruits.get(6) {
        Some(fruit) => println!("The sixth element is: {}", fruit),
        None => println!("Not value at index 6"),
    }

    // iteranting over a vector
    for fruit in fruits {
        println!("{}", fruit)
    }

    // Preallocating a vector
    let mut vec = Vec::with_capacity(10);

    for i in 0..11 {
        vec.push(i);
    }

    println!("\nVector: {:?}, Capacity: {}", vec, vec.capacity());
}
