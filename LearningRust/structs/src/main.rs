struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: u64,
    date_of_birth: String,
}

fn print_user_message(user: &User) {
    println!(
        "Name: {}, Email: {}, Active: {}, Sign-in Count: {}, Date of birth: {}",
        user.name, user.email, user.active, user.sign_in_count, user.date_of_birth
    );
}

struct Color(i32, i32, i32); // RGB values
struct Point(f64, f64, f64); // xyz coordinates

fn main() {
    let red = Color(255, 0, 0);
    let p1 = Point(30.9, 500.0, 67.6);

    let user1 = User {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        active: true,
        sign_in_count: 5,
        date_of_birth: String::from("1950-05-05"),
    };

    let user2 = User {
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        active: false,
        sign_in_count: 3,
        date_of_birth: String::from("1967-06-07"),
    };

    print_user_message(&user1);
    print_user_message(&user2);
}
