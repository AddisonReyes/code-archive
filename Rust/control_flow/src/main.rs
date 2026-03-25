fn main() {
    /*
        IF STATEMENT
        If a number is divisible by 5 and 3 print 'TriQunit'
        If a number is divisible by 6 and 4 print 'HexaQuad'
        Otherwise print 'Just another number'
    */

    let number = 67;

    if number % 5 == 0 && number % 3 == 0 {
        println!("{} is a TriQunit", number);
    } else if number % 6 == 0 && number % 4 == 0 {
        println!("{} is a HexaQuad", number);
    } else {
        println!("Just another number");
    }

    /*
    IF LET STATEMENT
    If is_weekend is true, assign "Go hiking" to the variable 'activity'
    Otherwise, assing 'Go to work'
    */

    let is_weekend: bool = true;
    let activity: &str;

    if is_weekend {
        activity = "Go Hiking";
    } else {
        activity = "Go to work";
    };

    println!("{} is today's activity", activity);

    /*
        FOR LOOP
        Iterate over collection like arrays
    */

    let arr = [10, 20, 30, 40, 50, 60];
    for elem in arr {
        if elem == arr[arr.len() - 1] {
            println!("{}", elem);
        } else {
            print!("{}, ", elem);
        }
    }

    /*
        WHILE STATMENT
        Print a count down and when counter reaches zero, print "LIFT OFF!"
    */

    let mut counter = 10;
    while counter > 0 {
        println!("Countdown: {}", counter);
        counter -= 1;
    }

    println!("LIFT OFF!");

    /* LOOP STATEMENT */
    let mut index = 1;
    loop {
        index += 1;
        println!("Index {}", index);

        if index == 100 {
            println!("Max index reached");
            break;
        }
    }
}
