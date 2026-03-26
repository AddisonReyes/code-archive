use num::integer::gcd;
use num::integer::lcm;

mod interest;
mod tax;

fn main() {
    let income = 45000.0;
    let tax = tax::calculate_income_tax(income);
    println!("Annual Income Tax: ${:.2}", tax);

    let principal = 1000.0;
    let rate = 0.05;
    let time = 10.0;
    let n = 4.0;
    let compound_amount = interest::calculate_compound_interest(principal, rate, time, n);
    println!("Compound Interest Amount: ${:.2}", compound_amount);

    let simple_amount = interest::calculate_simple_interest(principal, rate, time);
    println!("Simple Interest Amount: ${:.2}", simple_amount);

    let tax_rate = tax::get_tax_rate(income);
    println!("Tax Rate: {:.2}%", tax_rate * 100.0);

    // Scenario 2: Using the num crate for GCD and LCM
    let x = 1234;
    let y = 9876;

    let result_lsm = lcm(x, y);
    let result_gcd = gcd(x, y);

    println!("\nGCD => {}", result_gcd);
    println!("LSM => {}", result_lsm);
}
