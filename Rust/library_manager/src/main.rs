/* Step 1: Define a Book struct */
struct Book {
    title: String,
    author: String,
    pages: u32,
}

/* Step 2: Implement the get_summary method on the Book struct */
impl Book {
    fn get_summary(&self) -> String {
        return format!("{} by {}, {} pages", self.title, self.author, self.pages);
    }
}

fn main() {
    /* Step 3: Create an array of books */
    let books = [
        Book {
            title: String::from("The Rust programming language"),
            author: String::from("Steve Klabnik and Carol Nichols"),
            pages: 527,
        },
        Book {
            title: String::from("Hands-on Rust: Effective Learning through 2D Game Development"),
            author: String::from("Herbert Wolverson"),
            pages: 344,
        },
        Book {
            title: String::from("Advanced Hands-on Rust: Level Up Your Coding Skills"),
            author: String::from("Herbert Wolverson"),
            pages: 350,
        },
        Book {
            title: String::from(
                "Rust for Rustaceans: Idiomatic Programming for Experienced Developers",
            ),
            author: String::from("Jon Gjengset"),
            pages: 350,
        },
    ];

    /*
        Step 4: Use a for loop to iterate through the array
        and print the summary of each book
    */
    for book in books {
        println!("{}", book.get_summary());
    }
}
