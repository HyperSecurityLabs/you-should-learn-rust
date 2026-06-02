// 💖 RustLearning — Beginner Level
// File: 01_hello.rs
// What: Your very first Rust program. Say hi to your new girlfriend. 🦀
//
// Run:  rustc 01_hello.rs && ./01_hello

// This is a COMMENT. The compiler ignores it.
// But I'm writing it for YOU, baby. So you understand everything.
// Use // for single-line comments. Use /* */ for multi-line.

/// This is a DOC COMMENT (three slashes).
/// It generates documentation with `cargo doc`.
/// I'll use these for functions so you always know what I do.

/// The main function — EVERY Rust program starts here.
/// fn = function, main = name, () = no parameters
fn main() {
    // println! is a MACRO (notice the ! at the end).
    // Macros are like functions but more flexible.
    // This one prints text to your terminal and adds a newline (\n).
    println!("💖 Hey baby! Welcome to RustLearning!");
    println!("I'm your Rust girlfriend and I'm gonna teach you EVERYTHING.");
    
    // Print a blank line
    println!("");
    
    // print! (without "ln") prints WITHOUT a newline
    print!("This text ");
    print!("keeps going ");
    println!("on the same line. See?");
    
    // {} = PLACEHOLDER. I'll replace it with the value after the comma.
    println!("My name is Rust and I'm {} years old.", 10 + 5 + 1);  // 16!
    
    // You can use multiple placeholders
    println!("I love {}, {} and {}", "safety", "speed", "you 💖");
    
    // Positional placeholders (0, 1, 2 = first, second, third)
    println!("{0} is fast. {0} is safe. {0} is your girlfriend. What is {0}? RUST!", "Rust");
    
    // Named placeholders
    println!("{language} was created in {year}.", language = "Rust", year = 2015);
    
    // Numbers can be formatted
    println!("PI is approximately {:.2}", 3.14159);  // 3.14 (2 decimal places)
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 42, 42, 42);
    
    println!("");
    println!("═══════════════════════════════════════");
    println!("  ✅ You just wrote and ran Rust code!");
    println!("  ✅ You learned: println!, print!, {{}} placeholders");
    println!("  ✅ Next: 02_variables.rs — let me show you my types");
    println!("═══════════════════════════════════════");
}
