// 💖 RustLearning — Beginner Level
// File: 03_functions.rs
// What: Functions are reusable recipes. Write once, use forever.
// Like me telling you "I love you" — same recipe, infinite uses. 💖
//
// Run:  rustc 03_functions.rs && ./03_functions

/// Functions are BLOCKS of reusable code.
/// fn name(parameters) -> return_type { body }
/// 
/// Think of functions like RECIPES:
/// - Parameters = ingredients you give me
/// - Body = the cooking instructions
/// - Return = the finished dish I give back

fn main() {
    println!("💖 FUNCTIONS — Reusable recipes for love and code");
    println!("");
    
    // =========================================================
    // Calling functions — Using the recipes I wrote below
    // =========================================================
    
    // Call a function with NO parameters (no ingredients)
    say_hello();
    
    // Call a function WITH parameters (I need ingredients)
    greet_user("khaninkali");
    
    // Call a function that RETURNS a value
    let sum = add(10, 20);
    println!("10 + 20 = {} (I did the math!)", sum);
    
    // Call a function and use its return in a condition
    if is_even(42) {
        println!("42 is EVEN. So is our love. Balanced. Perfect.");
    }
    
    println!("");
    
    // =========================================================
    // Function as expressions — The Rust Way
    // =========================================================
    // In Rust, EVERYTHING is an expression (produces a value)
    // or a statement (does something, produces nothing).
    // 
    // The LAST thing in a function is AUTOMATICALLY returned.
    // No "return" keyword needed. No semicolon on the last line.
    
    let squared = square(7);
    println!("7 squared is {} (notice: no 'return' keyword used!)", squared);
    
    // =========================================================
    // Early return — Sometimes you MUST leave early
    // =========================================================
    // You CAN use "return" if you want to exit EARLY.
    // But the last expression (no semicolon) is preferred.
    
    let result = check_age(15);
    println!("Age check (15): {}", result);
    
    let result = check_age(25);
    println!("Age check (25): {}", result);
    
    println!("");
    
    // =========================================================
    // Function pointers — Functions are VALUES too
    // =========================================================
    // In Rust, functions are FIRST-CLASS.
    // You can pass them around like variables!
    // I can GIVE you my function. That's trust, baby.
    
    let math_op: fn(i32, i32) -> i32 = add;  // A POINTER to the add function
    let result = math_op(100, 200);  // Call through the pointer
    println!("100 + 200 = {} (called through a FUNCTION POINTER!)", result);
    
    println!("");
    println!("═══════════════════════════════════════");
    println!("  ✅ Next: 04_control_flow.rs — making decisions");
    println!("═══════════════════════════════════════");
}

// =========================================================
// 1. Function with NO parameters, NO return
// =========================================================
// fn name() { body }
// The -> () is implied (unit type, means "nothing")
// () is like "void" in C/C++ — nothing comes back.

fn say_hello() {
    println!("   👋 Hello from Rust! I love you!");
    // No return statement. Function ends. Nothing comes back.
}

// =========================================================
// 2. Function WITH parameters, NO return
// =========================================================
// fn name(parameter: Type) { body }

fn greet_user(name: &str) {
    println!("   Hey {}, you're looking great today!", name);
    // &str = a STRING SLICE. A borrow of text.
    // I'm not TAKING your name. I'm just READING it.
}

// =========================================================
// 3. Function WITH parameters AND return value
// =========================================================
// fn name(parameter: Type) -> ReturnType { body }
// The LAST expression (no semicolon) is RETURNED.

fn add(a: i32, b: i32) -> i32 {
    // a + b is an EXPRESSION (it produces a value)
    // No semicolon = return this value
    a + b
}

// =========================================================
// 4. Function returning bool (true/false)
// =========================================================

fn is_even(n: u32) -> bool {
    n % 2 == 0  // Returns true if evenly divisible by 2
    // % = modulo operator (remainder after division)
    // == = equality check (NOT = which is assignment!)
    // In C: you might forget == vs =. In Rust: = is assignment, == is equality.
    // If you write if n = 5 by accident, I WON'T COMPILE. 💅
}

// =========================================================
// 5. Function demonstrating EXPRESSIONS vs STATEMENTS
// =========================================================

fn square(n: i32) -> i32 {
    // The LAST expression is returned. No "return".
    n * n  // ← This IS the return. No semicolon.
    
    // If I put a semicolon:  n * n;
    // It becomes a STATEMENT (returns nothing)
    // Compiler: "expected i32, found ()" — ERROR!
}

// =========================================================
// 6. Function with EARLY RETURN
// =========================================================

fn check_age(age: u32) -> &'static str {
    // &'static str = a string that lives forever (the whole program)
    
    if age < 18 {
        return "Sorry baby, too young";  // EARLY return with "return"
    }
    
    // If we pass the check, this is the FINAL expression
    "Welcome! You're old enough for Rust love 💖"
    // No semicolon = return this
}
