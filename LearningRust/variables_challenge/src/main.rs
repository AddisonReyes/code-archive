fn main() {
    //  Variables Challenge!
    
    /* 1. Store a person's name */
    let name: &str = "Addison Reyes";
    
    /* 2. Use a variable to store the person's current age */
    let mut age: i8 = 24;
    
    /* 3. Use a constant for number of years */
    const YEARS: i8 = 10;
    
    /* 4. Calculate the person's age in the future~ */
    age += YEARS;
    
    /* 5. Print the name and the calculated future age */
    println!("Name: {}", name);
    println!("Age: {}", age);
}
