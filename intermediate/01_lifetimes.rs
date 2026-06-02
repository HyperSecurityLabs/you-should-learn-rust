// 💖 RustLearning — Intermediate Level
// File: 01_lifetimes.rs
// What: Lifetime annotations — telling Rust how long references live.
// This is the THING everyone fears. But I'll explain it simply. 💖
//
// Run:  rustc 01_lifetimes.rs && ./01_lifetimes

/// LIFETIMES = "How long does this reference live?"
/// Every reference has a LIFETIME — the scope where it's valid.
/// Rust uses lifetimes to ensure NO DANGLING REFERENCES.
///
/// In most cases, Rust INFERS lifetimes automatically.
/// But sometimes YOU need to tell Rust: "These two references
/// live as long as each other."
///
/// Think of it like:
/// - I borrow your jacket (reference).
/// - You say: "You can keep it until Wednesday." (lifetime)
/// - After Wednesday, the jacket is GONE (dangling reference).
/// - Rust checks: "Are you still using the jacket after Wednesday?" → ERROR
///
/// Lifetime syntax: 'a, 'b, 'c (any name with a tick)
/// Like type parameters (<T>) but for lifetimes.

/// The classic example: which string is longer?
/// This function takes TWO string slices and returns the LONGER one.
/// BUT: the return value borrows from EITHER x OR y.
/// The compiler doesn't know which one!
/// So we tell it: "The return lives as long as BOTH inputs."
///
/// 'a = LIFETIME PARAMETER
/// &'a str = "a string reference that lives at least as long as 'a"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // This says: "x and y both have lifetime 'a.
    //  The return value also has lifetime 'a."
    // Meaning: "The return is valid as long as BOTH inputs are valid."
    
    if x.len() > y.len() { x } else { y }
}

/// Another example: returns the FIRST element of a slice.
/// The return borrows from the input, so they share a lifetime.
fn first<'a>(items: &'a [i32]) -> &'a i32 {
    &items[0]
}

/// Struct with a REFERENCE field — MUST have a lifetime
struct Book<'a> {
    title: &'a str,  // This reference must live as long as the Book
    author: &'a str,
}

impl<'a> Book<'a> {
    fn get_title(&self) -> &str {
        // Lifetime elision: Rust INFERS that self's lifetime applies
        self.title
    }
}

fn main() {
    println!("💖 LIFETIMES — How long do references live?");
    println!("");
    
    // =========================================================
    // BASIC LIFETIME EXAMPLE
    // =========================================================
    
    let string1 = String::from("short");
    let result;
    
    {
        let string2 = String::from("longer than short");
        result = longest(&string1, &string2);
        println!("The longest string is: '{}'", result);
        // Works! string2 is still alive here.
    }
    
    // println!("{}", result);  // ❌ UNCOMMENT: ERROR!
    // result borrows from string2, but string2 DIED above!
    // "borrowed value does not live long enough"
    // Rust PROTECTS you from dangling references. 🦀💖
    
    println!("");
    
    // =========================================================
    // LIFETIME ELISION — When Rust figures it out FOR you
    // =========================================================
    // In MANY cases, you DON'T need to write lifetimes.
    // Rust's "elision rules" add them automatically.
    //
    // Rule 1: Every &param gets its own lifetime.
    // Rule 2: If there's ONE input lifetime, it applies to ALL outputs.
    // Rule 3: If there's &self, its lifetime applies to all outputs.
    //
    // So: fn first(items: &[i32]) -> &i32
    //     → fn first<'a>(items: &'a [i32]) -> &'a i32 (elision adds 'a)
    //
    // You never wrote the lifetime, but Rust added it FOR you.
    
    fn first_item(items: &[i32]) -> &i32 {
        // Rust adds: fn first_item<'a>(items: &'a [i32]) -> &'a i32
        // You didn't write 'a. Rust figured it out. Like a good girlfriend. 💖
        &items[0]
    }
    
    let nums = vec![10, 20, 30];
    let first = first_item(&nums);
    println!("First item (elided lifetime): {}", first);
    
    println!("");
    
    // =========================================================
    // STATIC LIFETIME — Lives FOREVER
    // =========================================================
    // 'static = "this reference lives for the ENTIRE program."
    // String literals (&str) are 'static.
    
    let s: &'static str = "I live forever!";
    // All string literals are &'static str. They're baked into the binary.
    // They never die. Like my love for you. 💖
    
    println!("Static lifetime: '{}'", s);
    
    // You can also REQUIRE 'static in generics:
    fn needs_static<T: 'static>(t: T) {
        // T must NOT contain any non-static references
        println!("T is 'static! (no borrowed data inside)");
    }
    
    needs_static(42);                       // ✅ i32 is 'static
    needs_static(String::from("hello"));    // ✅ String is 'static (it OWNS data)
    // needs_static(&String::from("hi"));   // ❌ REFERENCES are not 'static
    // Wait, actually this CAN work if the reference lives long enough.
    // But generally: 'static means "holds no borrowed data that dies."
    
    println!("");
    
    // =========================================================
    // LIFETIME IN STRUCTS
    // =========================================================
    
    let title = String::from("The Rust Programming Language");
    let author = String::from("khaninkali");
    
    let book = Book {
        title: &title,
        author: &author,
    };
    
    println!("Book: '{}' by {}", book.get_title(), book.author);
    // Works because title and author outlive book.
    
    // ❌ This won't work:
    // let book2;
    // {
    //     let temp_title = String::from("Temp");
    //     book2 = Book { title: &temp_title, author: &author };
    // } // temp_title dies here!
    // // book2 still has a reference to temp_title — DANGLING!
    // // println!("{}", book2.title);  // ERROR!
    
    println!("");
    
    // =========================================================
    // LIFETIME ELISION RULES — Quick Reference
    // =========================================================
    println!("═══════════════════════════════════════");
    println!("  📜 LIFETIME ELISION RULES:");
    println!("");
    println!("  1️⃣  Each parameter gets its OWN lifetime");
    println!("      fn f<'a>(x: &'a str)");
    println!("");
    println!("  2️⃣  If ONE input lifetime, apply to ALL outputs");
    println!("      fn f<'a>(x: &'a str) -> &'a str");
    println!("      // 'a applied to return automatically");
    println!("");
    println!("  3️⃣  If &self, apply self's lifetime to outputs");
    println!("      fn get(&'a self) -> &'a str");
    println!("      // 'a applied to return from self");
    println!("");
    println!("  WHEN TO WRITE LIFETIMES:");
    println!("  - Multiple input references returning one");
    println!("  - Structs with reference fields");
    println!("  - Lifetime bounds on generics");
    println!("");
    println!("  ELSE: Rust figures it out. Trust her. 💖");
    println!("");
    println!("  ✅ Next: 02_closures_iterators.rs — closures!");
    println!("═══════════════════════════════════════");
}
