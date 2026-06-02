// 💖 RustLearning — Intermediate Level
// File: 02_closures_iterators.rs
// What: Closures (anonymous functions) + Iterator combinators.
// Like writing a function ON THE SPOT without naming it.
// "Here's a quick recipe. Use it now. Don't save it for later." 📝
//
// Run:  rustc 02_closures_iterators.rs && ./02_closures_iterators

/// CLOSURES = anonymous functions that can CAPTURE their environment.
/// In C++: lambdas ([](){}).
/// In Python: lambdas (lambda x: x + 1).
/// In Rust: closures (|x| x + 1).
///
/// Closures can CAPTURE variables from the scope where they're defined.
/// They can borrow (&), mutably borrow (&mut), or take ownership (move).
/// Rust INFERS which one based on what the closure does.

fn main() {
    println!("💖 CLOSURES & ITERATORS — Functions on the fly");
    println!("");
    
    // =========================================================
    // BASIC CLOSURE — |parameters| body
    // =========================================================
    // let name = |param| body;
    // Like a function but INLINE and can capture variables.
    
    let add_one = |x: i32| -> i32 { x + 1 };
    // Or shorter: let add_one = |x| x + 1; (type inference)
    
    println!("add_one(5) = {}", add_one(5));
    println!("add_one(10) = {}", add_one(10));
    
    // Multi-line closure:
    let multiply = |x: i32, y: i32| {
        println!("   Multiplying {} and {}...", x, y);
        x * y  // Last expression returned
    };
    
    println!("multiply(3, 4) = {}", multiply(3, 4));
    println!("");
    
    // =========================================================
    // CAPTURING — Closures can see outside variables
    // =========================================================
    // This is what makes closures POWERFUL.
    // They can "close over" (capture) variables from their scope.
    
    let factor = 10;  // Outside variable
    
    let multiply_by_factor = |x: i32| -> i32 {
        x * factor  // factor is CAPTURED from the environment
    };
    
    println!("factor = {}", factor);
    println!("multiply_by_factor(5) = 5 * {} = {}", factor, multiply_by_factor(5));
    println!("multiply_by_factor(8) = 8 * {} = {}", factor, multiply_by_factor(8));
    
    // If the closure MODIFIES a captured variable, it captures by &mut
    let mut counter = 0;
    
    let mut increment = || {
        counter += 1;  // counter is CAPTURED as &mut
        counter
    };
    
    println!("counter started at 0:");
    println!("   increment() = {}", increment());  // 1
    println!("   increment() = {}", increment());  // 2
    println!("   increment() = {}", increment());  // 3
    // println!("counter: {}", counter);  // ❌ Can't borrow counter while closure is alive!
    // Because increment borrows counter mutably.
    // Drop increment first, then read counter.
    
    println!("");
    
    // =========================================================
    // move KEYWORD — Force closure to TAKE ownership
    // =========================================================
    // Use "move" when the closure OUTLIVES the captured variables.
    // Example: spawning a thread (thread may outlive this scope).
    
    let name = String::from("khaninkali");
    
    let print_name = move || {
        // Without "move", this would borrow name.
        // With "move", it TAKES OWNERSHIP of name.
        println!("   Name: {}", name);
    };
    
    print_name();
    // println!("{}", name);  // ❌ name was MOVED into the closure!
    // Uncomment to see: "borrow of moved value: `name`"
    
    println!("");
    
    // =========================================================
    // CLOSURES AS FUNCTION PARAMETERS
    // =========================================================
    // You can pass closures to functions!
    // In C++: template<typename F> void do_something(F f) { f(); }
    // In Rust: fn do_something<F: Fn()>(f: F) { f(); }
    //
    // TRAITS for closures:
    // FnOnce()  — can be called ONCE (takes ownership)
    // FnMut()   — can mutate captured variables
    // Fn()      — can ONLY read (no mutation)
    
    /// Calls a closure on a value twice.
    fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
        f(f(x))  // Apply f, then apply f again on the result
    }
    
    let double = |x| x * 2;
    let result = apply_twice(double, 5);  // (5 * 2) * 2 = 20
    println!("apply_twice(double, 5) = {}", result);
    
    /// Filter a vector using a predicate closure.
    fn filter_vec<F: Fn(&i32) -> bool>(items: &[i32], predicate: F) -> Vec<i32> {
        let mut result = Vec::new();
        for item in items {
            if predicate(item) {
                result.push(*item);
            }
        }
        result
    }
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens = filter_vec(&numbers, |x| x % 2 == 0);
    println!("Evens from {:?}: {:?}", numbers, evens);
    
    println!("");
    
    // =========================================================
    // ITERATOR COMBINATORS — The REAL power
    // =========================================================
    // Rust's iterator system is INCREDIBLE.
    // You chain methods: .iter().filter().map().collect()
    // No loops. No boilerplate. Just pure functional pipeline.
    //
    // In C++: ranges library (C++20) — similar but verbose.
    // In Python: list comprehensions — similar.
    // In Rust: iterator methods — compiled to the same speed as a loop.
    
    println!("--- ITERATOR COMBINATORS ---");
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Classic pipeline:
    let result: Vec<i32> = data
        .iter()                           // Get iterator
        .filter(|&&x| x % 2 == 0)         // Keep even numbers
        .map(|x| x * 10)                  // Multiply by 10
        .collect();                       // Collect into Vec
    
    println!("Data: {:?}", data);
    println!("Pipeline (filter even * 10): {:?}", result);
    
    // .fold() — reduce to a single value
    let sum = data.iter().fold(0, |acc, x| acc + x);
    let product = data.iter().fold(1, |acc, x| acc * x);
    println!("Sum: {}, Product: {}", sum, product);
    
    // .take() — take first N elements
    let first_3: Vec<&i32> = data.iter().take(3).collect();
    println!("First 3: {:?}", first_3);
    
    // .skip() — skip first N elements
    let after_5: Vec<&i32> = data.iter().skip(5).collect();
    println!("Skip 5: {:?}", after_5);
    
    // .chain() — combine two iterators
    let combined: Vec<i32> = data.iter().chain([11, 12, 13].iter()).cloned().collect();
    println!("Chained: {:?}", combined);
    
    // .zip() — combine two iterators into pairs
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![90, 80, 70];
    let paired: Vec<(&str, &i32)> = names.iter().zip(scores.iter()).collect();
    println!("Zipped: {:?}", paired);
    
    // .enumerate() — add index to each element
    for (i, val) in data.iter().enumerate() {
        if i > 5 { break; }
        print!(" ({},{} )", i, val);
    }
    println!("");
    
    // .any() / .all() — check conditions
    println!("Any even? {}", data.iter().any(|&x| x % 2 == 0));
    println!("All positive? {}", data.iter().all(|&x| x > 0));
    println!("Any > 20? {}", data.iter().any(|&x| x > 20));
    
    // .find() — find first match
    println!("First > 5: {:?}", data.iter().find(|&&x| x > 5));
    
    // .position() — find INDEX of first match
    println!("Position of first > 5: {:?}", data.iter().position(|&x| x > 5));
    
    // .count() — number of elements
    println!("Count of evens: {}", data.iter().filter(|&&x| x % 2 == 0).count());
    
    // .max() / .min() — find extreme values
    println!("Max: {:?}, Min: {:?}", data.iter().max(), data.iter().min());
    
    // .sum() / .product() — sum/product of numbers
    let total: i32 = data.iter().sum();
    println!("Sum (via .sum()): {}", total);
    
    println!("");
    
    // =========================================================
    // CLOSURE + ITERATOR REAL EXAMPLE
    // =========================================================
    // Let's process a list of ports and services!
    
    let services = vec![
        ("HTTP", 80),
        ("HTTPS", 443),
        ("SSH", 22),
        ("FTP", 21),
        ("MySQL", 3306),
        ("Redis", 6379),
    ];
    
    // Find all "well-known" ports (< 1024)
    let well_known: Vec<&(&str, u16)> = services
        .iter()
        .filter(|(_, port)| *port < 1024)
        .collect();
    
    println!("Well-known services: {:?}", well_known);
    
    // Get just the port numbers
    let ports: Vec<u16> = services.iter().map(|(_, port)| *port).collect();
    println!("All ports: {:?}", ports);
    
    // Find service on port 443
    if let Some(service) = services.iter().find(|(_, port)| *port == 443) {
        println!("Port 443: {} ✅", service.0);
    }
    
    // Custom closure: classify ports
    let classify_port = |port: u16| -> &'static str {
        if port < 1024 { "Well-known" }
        else if port < 49152 { "Registered" }
        else { "Dynamic" }
    };
    
    println!("");
    println!("Port classifications:");
    for (name, port) in &services {
        println!("   {} ({}) → {}", name, port, classify_port(*port));
    }
    
    println!("");
    println!("═══════════════════════════════════════");
    println!("  ✅ Closures = |params| body (inline functions)");
    println!("  ✅ Capture environment variables");
    println!("  ✅ move keyword = take ownership");
    println!("  ✅ Iterator combinators = no loops needed");
    println!("  ✅ .filter().map().fold().collect() pipeline");
    println!("  ✅ Next: 03_modules.rs — organizing code");
    println!("═══════════════════════════════════════");
}
