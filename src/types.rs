/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.
// means we don't need to define the type for each variable we create
pub fn run() {
    // Default i32
    let x = 1;

    // Default f64
    let y = 2.5;

    // explicit types
    let x2: i64 = 5555;

    // printing max

    println!("Max i32: {}", std::i32::MAX);
    println!("Max u32: {}", std::u32::MAX);

    println!("Max f32: {}", std::i32::MAX);

    // Boolean
    let is_active: bool = true;
    let is_active2: bool = 1 == 2;

    if is_active && is_active2 {
        println!("It's active");
    }

    // Character
    let char: char = 'a';
    let face: char = '\u{1F600}';

    println!("Single char : {} {}", char, face);
}
