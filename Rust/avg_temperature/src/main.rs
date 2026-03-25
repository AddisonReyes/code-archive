fn main() {
    /*
       SCENARIO 2:

       1. Create an array with the temperatures for each day of the week
       2. Use a for loop to calculate sum of temperatures
       3. Find and print the average
    */

    let temperatures = [70.1, 75.2, 72.6, 68.1, 74.0, 78.5, 73.4];
    let mut total_sum = 0.0;

    for temp in temperatures {
        total_sum += temp;
    }

    let avg_temperature = total_sum / temperatures.len() as f32;
    println!("The average temperature is {}", avg_temperature);
}
