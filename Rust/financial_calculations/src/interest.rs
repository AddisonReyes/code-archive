pub fn calculate_compound_interest(principal: f64, rate: f64, time: f64, n: f64) -> f64 {
    principal * (1.0 + rate / n).powf(n * time)
}

pub fn calculate_simple_interest(principal: f64, rate: f64, time: f64) -> f64 {
    principal * rate * time
}
