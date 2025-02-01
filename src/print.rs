pub fn run() {
    // print to console
    println!("Hello from print.rs file");

    // placeholder for numbers
    println!("Number: {}", 1);

    // placeholders
    println!("Hi {}, welcome in our rust program in {}", "Muhammed", 2025);

    // positional placeholders
    println!(
        "{0} loves football and {1} loves coding and same {0} loves fucking",
        "Muhammed", "Mo"
    );

    // named arguments
    println!(
        "{person} loves rust and it's powerful abilities",
        person = "Muhammed"
    );
    
    // traits placeholders
    println!("Binary {:b}, Hex {:x}, Octal {:o}", 10, 10, 10);

    // placeholder for debug traits
    println!("{:?}", (10, true, "Hello, world!"));

    // placeholder math
    println!("{}", 10.0/3.0);
}
