pub fn run() {
    /* -------------- Print Tricks ------------------ */ 
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
    );

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    /* -------------- Basic Math ------------------ */
    println!("10 + 10 = {}", 10 + 10)
    
}
