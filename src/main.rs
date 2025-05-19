mod lifetimes;
use lifetimes::{create_document, get_combined_str, get_document_extract, wrap_data};

mod threads;
use std::io::{self, Write};
use threads::{enter_arc, enter_mutex, enter_mutex_vec, simple_thread};

fn learn_lifetimes() {
    wait_for_enter("Press Enter to run next function...");
    // lifetime elison
    get_combined_str();

    wait_for_enter("Press Enter to run next function...");
    // structs and lifetimes
    create_document();

    wait_for_enter("Press Enter to run next function...");
    // traits and lifetimes
    get_document_extract();

    wait_for_enter("Press Enter to run next function...");
    // structs and lifetimes - complex
    wrap_data();
    println!("All done!");
}

fn learn_threads() {
    wait_for_enter("Press Enter to run next function...");
    simple_thread();

    wait_for_enter("Press Enter to run next function...");
    // thread safety
    enter_arc();

    wait_for_enter("Press Enter to run next function...");
    // thread safety - mutex
    enter_mutex();

    wait_for_enter("Press Enter to run next function...");
    // thread safety - mutex - complex
    enter_mutex_vec();

    println!("All done!");
    println!("Reference for further reading: ");
    println!(
        "1. https://www.ruststepbystep.com/rust-concurrency-made-easy-a-guide-to-arc-and-mutex/"
    );
    println!("2. https://rust-guide.com/en/documentation/concurrency/Arc");
    println!("3. https://rust-guide.com/en/documentation/concurrency/Mutex");
    println!();
    println!("If you would like to challenge yourself, try to implement the following:");
    println!("1. Modify the code so that each thread updates a different index in the vector.");
    println!("Reference for challenge: https://www.ruststepbystep.com/rust-advanced-concurrency-using-arc-mutex-and-rwlock-safely/");

    println!("All done! Thank you for attending the workshop!");
}

fn wait_for_enter(message: &str) {
    println!(); // print a newline for formatting
    print!("{}", message);
    println!(); // print a newline for formatting

    io::stdout().flush().unwrap();
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
}

fn main() {
    println!("Would you like to learn about lifetimes or threads?");
    print!("Type 'lifetimes' or 'threads' and press Enter: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim().to_lowercase();

    if choice == "lifetimes" {
        learn_lifetimes();
    } else if choice == "threads" {
        learn_threads();
    } else {
        println!("Invalid choice. Defaulting to lifetimes.");
        learn_lifetimes();
    }
}
