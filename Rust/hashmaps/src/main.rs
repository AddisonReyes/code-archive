use std::collections::HashMap;

fn main() {
    // HashMap<K, V>
    let mut population = HashMap::new();

    population.insert("Tokyo", 37_400_100);
    population.insert("London", 17_400_100);
    population.insert("Dubai", 7_400_100);

    println!("population: {:?}", population);

    population.insert("Tokyo", 47_000_000);
    let city = "Tokyo";

    match population.get(city) {
        Some(&pop) => println!("Population of {city}: {}", pop),
        None => println!("City not found."),
    }

    population.remove("Tokyo");

    for (city, pop) in &population {
        println!("{}: {}", city, pop)
    }

    for value in population.values() {
        println!("Population: {}", value)
    }

    population.entry("Delhi").or_insert(32_000_000);
    population.entry("Dubai").or_insert(9_000_000);
    println!("population: {:?}", population);

    // HashMap performance
    // for better performance: String, i32 or usize as keys
    let mut scores = HashMap::with_capacity(10);

    for i in 0..14 {
        scores.insert(i, i * 10);
    }

    println!("Scores: {:?}", scores);
    println!("Capacity: {}", scores.capacity());
}
