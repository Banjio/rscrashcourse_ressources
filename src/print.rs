pub fn run() {
    //Print to console
    println!("Hello from the print.rs file");

    // Basic string interpliation
    println!("My Number is: {}", 42);

    // Works also with multiple placeholders
    println!("{} is from {}", "Eminem", "The Hood");

    // Positional Args
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Eminem", "The Hood", "Rap"
    );

    //Named Args
    println!(
        "{name} is {name_slang}",
        name = "Slim Shady",
        name_slang = "Mega Shady"
    )
}
