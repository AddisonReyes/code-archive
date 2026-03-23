fn print_greetings(name: String) -> String {
    println!("Welcome {name}");
    return name;
}

fn printr_greetings(name: &String) {
    println!("Welcome {name}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    // Example #1
    let s1 = String::from("Hello"); // s1 is the owner of Hello
    let s2 = s1; // s2 has become the owner of Hello
    let s3 = s2.clone(); // s3 is a clone of s2
    println!("{s2} world!"); // If you use s1 this gives an error
    println!("{s3} world!");

    // Example #2
    let name = String::from("Rob");
    let name = print_greetings(name);
    println!("{name}");

    // Example #3:
    // Ownership transfer only happens with complex data types
    let i = 9;
    let j = i;
    println!("Balbaro {j}{i}");

    // References
    let my_name = String::from("Addison");
    printr_greetings(&my_name);
    println!("{my_name}");

    // Slice Type
    let message = String::from("Hello world");
    let hello = &message[0..5];
    println!("{hello}");

    let array = [1, 2, 3, 4, 5, 6];
    let slice = &array[1..3];

    for num in slice {
        println!("{}", num);
    }

    // Quiz code
    let str1 = String::from("hello");
    let len = calculate_length(&str1);
    println!("The length of '{}' is {}", str1, len);
    println!("{:?}", slice); // [2, 3, 4]
}
