use std::result;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(i32),
}

fn calculate_area(shape: Shape) {
    match shape {
        Shape::Circle(radius) => println!("Area of circle is {}", 3.14 * radius * radius),
        Shape::Rectangle(width, height) => {
            let area = width * height;
            println!("Area of rectangle is {}", area);
        }
        Shape::Square(side) => println!("Area of square is {}", side),
    }
}

fn safe_divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let light = TrafficLight::Red;

    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Caution!"),
        TrafficLight::Green => println!("Go!"),
        _ => println!("Invalid traffic light state!"),
    }

    let rectangle = Shape::Rectangle(30.7, 16.7);
    let circle = Shape::Circle(20.6);
    let square = Shape::Square(25);

    calculate_area(rectangle);
    calculate_area(circle);
    calculate_area(square);

    // Option Enum: eg.: Safe division
    let result = safe_divide(1000.0, 10.0);
    match result {
        Some(value) => println!("Succesful division: {}", value),
        None => println!("Cannot divide by zero"),
    }

    // If LET Statement
    if let TrafficLight::Red = light {
        println!("You must no pass");
    } else if let TrafficLight::Yellow = light {
        println!("You must no pass");
    }
}
