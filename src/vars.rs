// Variables hold primitive data or refrences to data
// Variables are immutable by default
// Rust is block-scoped language
pub fn run() {
    let name = "Hamo";
    let age = 50;
    let mut age2 = 55;

    // age = 30; // that is not allowed as this is immutable variable by default
    println!("My name is {}", name);
    println!("My age is {}", age);

    println!("My mutable age before is {}", age2);
    age2 = 22;
    println!("My mutable age after is {}", age2);

    // constants : neccessary to define the type
    const PASSKEY : i32 = 1032003;

    println!("PASSKEY : {}", PASSKEY);

    // multiple assignments

    let (my_name, my_age) = ("Muhammed ali", 22);
    println!("Hello my name is {} & my age is {}", my_name, my_age);
}
