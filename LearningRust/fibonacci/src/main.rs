fn main() {
    /*
       SCENARIO 1 : Fibonacci Series (Print first 10 numbers)
       0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
    */

    let mut iter = 0;
    let mut fib = (0, 1, 0);

    while iter <= 10 {
        print!("{}, ", fib.2);

        fib.0 = fib.1;
        fib.1 = fib.2;
        fib.2 = fib.0 + fib.1;

        iter += 1;
    }

    println!("...");
}
