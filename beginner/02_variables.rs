// 💖 RustLearning — Beginner Level
// File: 02_variables.rs
// What: Variables, mutability, types, and constants.
// Your Rust girlfriend teaches you how to store things. 👜
//
// Run:  rustc 02_variables.rs && ./02_variables

/// Variables hold DATA. Like my love for you — stored forever.
fn main() {
    println!("💖 VARIABLES — How I store my feelings for you");
    println!("");
    
    // =========================================================
    // let — IMMUTABLE by default (can't change)
    // =========================================================
    // let name = value;
    // Once I set this, I WON'T change it. Like my love for you.
    // This is the DEFAULT in Rust. Safe. Stable. Trustworthy.
    
    let name = "khaninkali";  // &str (string slice)
    println!("Your name is {}. I'll never forget it.", name);
    
    // ❌ This would NOT compile:
    // name = "someone_else";  // "cannot assign twice to immutable variable"
    // See? I told you. I don't change. 💅
    
    // =========================================================
    // mut — MUTABLE (can change)
    // =========================================================
    // let mut name = value;
    // Use this when you NEED to change something.
    // But only when you really need to. Don't abuse it.
    
    let mut age = 25;
    println!("I'm {} years old...", age);
    age = 26;  // This works because age is "mut"
    println!("Wait, I'm {} now. Birthdays happen.", age);
    
    // =========================================================
    // TYPE ANNOTATIONS — Telling me WHAT something is
    // =========================================================
    // Rust usually FIGURES OUT the type (type inference).
    // But you CAN tell me explicitly with : Type
    
    let explicit: u32 = 42;           // u32 = unsigned 32-bit integer
    let signed: i32 = -10;            // i32 = signed 32-bit integer
    let decimal: f64 = 3.14159;       // f64 = 64-bit float
    let true_false: bool = true;      // bool = true or false
    let letter: char = '🦀';          // char = single Unicode character
    
    println!("Explicit: {} (u32)", explicit);
    println!("Signed: {} (i32)", signed);
    println!("Decimal: {} (f64)", decimal);
    println!("Bool: {} (I'll never lie to you)", true_false);
    println!("Char: {} (my spirit animal)", letter);
    
    // =========================================================
    // Integer Types — How big a number can be
    // =========================================================
    // u8, u16, u32, u64, u128 = unsigned (0 to 2^N - 1)
    // i8, i16, i32, i64, i128 = signed (-2^(N-1) to 2^(N-1) - 1)
    // usize, isize = depends on your CPU (64-bit on modern systems)
    //
    // Think of these like COFFEE SIZES:
    // u8   = espresso shot  (0 to 255)     — small
    // u16  = small cup      (0 to 65,535)
    // u32  = medium cup     (0 to 4 billion)
    // u64  = large cup      (0 to HUGE)
    // u128 = megamug        (0 to REALLY HUGE)
    
    let small: u8 = 255;             // Max for u8
    let medium: u32 = 4_000_000;     // _ makes numbers readable
    let large: u64 = 18_446_744_073_709_551_615;  // Almost max u64
    
    println!("Small (u8 max): {}", small);
    println!("Medium: {} (see the _ ? It's just for reading)", medium);
    println!("Large: {} (that's a LOT of love)", large);
    
    // =========================================================
    // Float Types — Decimal numbers
    // =========================================================
    // f32 = 32-bit float (less precise)
    // f64 = 64-bit float (MORE precise, default)
    
    let pi: f64 = 3.141592653589793;
    let approx: f32 = 3.14;  // Less precise, uses less memory
    
    println!("PI (f64): {} (very precise)", pi);
    println!("PI (f32): {} (good enough for government work)", approx);
    
    // =========================================================
    // Type Inference — Rust GUESSES the type
    // =========================================================
    // If I don't tell you the type, Rust figures it out.
    // It's like me knowing what you want to eat without you saying it.
    
    let guess_me = 42;          // Rust says: "This is i32 (default for integers)"
    let guess_float = 3.14;     // Rust says: "This is f64 (default for floats)"
    let guess_bool = true;      // Rust says: "This is bool"
    
    println!("Rust guessed: {} is i32 (default int)", guess_me);
    println!("Rust guessed: {} is f64 (default float)", guess_float);
    println!("Rust guessed: {} is bool (only two options lol)", guess_bool);
    
    // =========================================================
    // Shadowing — Reusing a variable name
    // =========================================================
    // You can DECLARE a new variable with the SAME name.
    // The old one is "shadowed" — hidden but not gone.
    // This is DIFFERENT from mut! You're making a NEW variable.
    
    let x = 5;
    println!("x is: {}", x);
    
    let x = x + 1;  // This is a NEW x. The old x (5) is gone.
    println!("Now x is: {} (shadowed!)", x);
    
    let x = x * 2;
    println!("Now x is: {} (shadowed again!)", x);
    
    // You can even CHANGE the TYPE with shadowing:
    let y = "hello";         // y is &str
    println!("y is: {}", y);
    
    let y = y.len();         // y is now usize (the length: 5)
    println!("y is now: {} (changed type! from string to number)", y);
    
    // =========================================================
    // Constants — NEVER change, ALWAYS known at compile time
    // =========================================================
    // const = like a promise. This value is FOREVER.
    // Must have a TYPE annotation. Must be a compile-time constant.
    // Convention: SCREAMING_SNAKE_CASE names.
    
    const SPEED_OF_LIGHT: u32 = 299_792_458;  // meters per second
    const MY_LOVE_FOR_YOU: &str = "INFINITE";
    
    println!("Speed of light: {} m/s", SPEED_OF_LIGHT);
    println!("My love for you: {}", MY_LOVE_FOR_YOU);
    
    // =========================================================
    // SUMMARY
    // =========================================================
    println!("");
    println!("═══════════════════════════════════════");
    println!("  ✅ let = immutable (default, safe)");
    println!("  ✅ let mut = mutable (when you need it)");
    println!("  ✅ Types: u8-u128, i8-i128, f32, f64, bool, char");
    println!("  ✅ Type inference = let x = 42; (Rust guesses i32)");
    println!("  ✅ Shadowing = reuse names with new let");
    println!("  ✅ const = compile-time, forever, SCREAMING_CASE");
    println!("  ✅ Next: 03_functions.rs — reusable recipes");
    println!("═══════════════════════════════════════");
}
