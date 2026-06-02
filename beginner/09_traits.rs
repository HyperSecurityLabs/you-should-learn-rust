// 💖 RustLearning — Beginner Level
// File: 09_traits.rs
// What: Traits = shared behavior between types.
// Like saying: "Anything that can QUACK is a Duck."
// This is Rust's version of INTERFACES (Java) or CONCEPTS (C++20).
//
// Run:  rustc 09_traits.rs && ./09_traits

/// TRAITS define SHARED BEHAVIOR.
/// A trait is a COLLECTION of method signatures.
/// Any type that IMPLEMENTS a trait promises to have those methods.
///
/// Think of it like a SUPERPOWER:
/// - The "Fly" trait: anything that can fly has a .fly() method.
/// - Birds implement Fly. Airplanes implement Fly. I don't. I'm Rust. 💅
///
/// In C++: this is like abstract base classes or concepts.
/// In Java: this is like interfaces.
/// In Rust: this is TRAITS. More powerful than both.

use std::fmt;

// =========================================================
// DEFINING A TRAIT — What behavior looks like
// =========================================================

/// Anything that can make a sound has this trait.
trait Speak {
    /// Return the sound as a string.
    fn speak(&self) -> String;
    
    /// DEFAULT IMPLEMENTATION. Types CAN override this.
    /// If they don't, this version is used.
    fn name(&self) -> String {
        String::from("an unknown creature")
    }
    
    /// ANOTHER default method. Uses speak() internally.
    fn describe(&self) -> String {
        format!("{} says '{}'", self.name(), self.speak())
    }
}

// =========================================================
// IMPLEMENTING A TRAIT — Giving types superpowers
// =========================================================

struct Dog {
    name: String,
    breed: String,
}

impl Speak for Dog {
    fn speak(&self) -> String {
        String::from("Woof! Woof!")
    }
    
    fn name(&self) -> String {
        format!("{} the {} dog", self.name, self.breed)
    }
    // describe() uses the DEFAULT implementation (from the trait)
}

struct Cat {
    name: String,
    color: String,
}

impl Speak for Cat {
    fn speak(&self) -> String {
        String::from("Meow!")
    }
    
    fn name(&self) -> String {
        format!("{} the {} cat", self.name, self.color)
    }
    
    // OVERRIDE describe() with a custom version
    fn describe(&self) -> String {
        format!("{} hisses '{}' and ignores you", self.name, self.speak())
    }
}

// A type that uses the DEFAULT name() method
struct Robot {
    id: u32,
}

impl Speak for Robot {
    fn speak(&self) -> String {
        format!("BEEP BOOP. I AM UNIT {}.", self.id)
    }
    // name() uses DEFAULT: "an unknown creature"
    // describe() uses DEFAULT: "an unknown creature says 'BEEP BOOP...'"
}

// =========================================================
// TRAIT BOUNDS — Functions that work on ANY type with a trait
// =========================================================
// This is RUST'S GENERICS. Like C++ templates but SAFE.
//
// fn function<T: TraitName>(param: T) { ... }
// Means: "This function works with ANY type T that implements TraitName."
//
// In C++: template<typename T> void function(T param) { ... }
// In C++: if T doesn't have the method, you get a 200-line error template.
// In Rust: T must IMPLEMENT the trait. The compiler checks EARLY.
//          Clean error messages. "Type T doesn't implement trait Speak."

// =========================================================
// SYNTAX 1: Generic with trait bound
// =========================================================

/// Make ANY Speak-able thing speak, then describe itself.
fn make_it_speak<T: Speak>(thing: &T) {
    // Because T: Speak, we KNOW thing has .speak(), .name(), .describe()
    // The compiler GUARANTEES it. No runtime check needed. No crashes.
    println!("   {}", thing.describe());
}

// =========================================================
// SYNTAX 2: Multiple trait bounds with +
// =========================================================

/// Debug + Clone + Speak — any type that implements ALL THREE.
fn clone_and_describe<T: Speak + Clone + fmt::Debug>(thing: &T) {
    let cloned = thing.clone();
    println!("   Original: {:?}", thing);
    println!("   Cloned speaks: {}", cloned.speak());
}

// =========================================================
// SYNTAX 3: where clause (cleaner for many bounds)
// =========================================================

/// Print the description of two Speak-able things.
fn compare_speeches<T, U>(thing1: &T, thing2: &U)
where
    T: Speak + fmt::Debug,
    U: Speak + fmt::Debug,
{
    println!("   Thing 1: {:?} → '{}'", thing1, thing1.speak());
    println!("   Thing 2: {:?} → '{}'", thing2, thing2.speak());
}

// =========================================================
// Built-in Traits You Should Know
// =========================================================

// Debug (fmt::Debug) — for {:?} printing
// Clone — for .clone() (deep copy)
// Copy — for implicit copy (integers, bools, etc.)
// PartialEq — for == and !=
// Eq — for strict equality (for HashSet, HashMap keys)
// PartialOrd — for <, >, <=, >=
// Ord — for sorting
// Hash — for hashing (for HashMap keys)
// Default — for ::default()
// Iterator — for .iter(), .map(), .filter(), etc.

// Let me show you how to DERIVE these traits AUTOMATICALLY:
// Just add #[derive(Debug, Clone, PartialEq)] above your struct!

#[derive(Debug, Clone, PartialEq, Default)]
struct HackerTool {
    name: String,
    version: u32,
    stealth_level: u8,
}

impl Speak for HackerTool {
    fn speak(&self) -> String {
        format!("I am {}, v{}. Stealth: {}. I HACK.", self.name, self.version, self.stealth_level)
    }
}

// =========================================================
// TRAIT as RETURN TYPE — impl Trait syntax
// =========================================================
// "impl Speak" means "returns SOME type that implements Speak."
// The caller doesn't know the exact type. Just that it can Speak.

fn create_default_tool() -> impl Speak {
    HackerTool {
        name: String::from("HyperGuard"),
        version: 1,
        stealth_level: 255,  // MAX STEALTH
    }
}

// =========================================================
// TRAIT OBJECTS — dynamic dispatch (like virtual functions)
// =========================================================
// dyn Trait = POINTER to any type that implements Trait.
// Size is UNKNOWN at compile time. Must be behind a pointer (& or Box).
// Like virtual functions in C++. Dynamic dispatch at runtime.

/// Collect Speak-able things and make them all speak.
fn make_all_speak(things: &[&dyn Speak]) {
    for thing in things {
        println!("   → {}", thing.describe());
    }
}

fn main() {
    println!("💖 TRAITS — Shared behavior between types");
    println!("");
    
    // Create some instances
    let dog = Dog {
        name: String::from("Buddy"),
        breed: String::from("Golden Retriever"),
    };
    
    let cat = Cat {
        name: String::from("Whiskers"),
        color: String::from("black"),
    };
    
    let robot = Robot { id: 42 };
    
    // Demonstrate methods
    println!("--- Individual speak() calls ---");
    println!("Dog: {}", dog.speak());
    println!("Cat: {}", cat.speak());
    println!("Robot: {}", robot.speak());
    println!("");
    
    println!("--- describe() — Cat overrides, Dog uses default ---");
    println!("Dog: {}", dog.describe());
    println!("Cat: {}", cat.describe());
    println!("Robot: {}", robot.describe());
    println!("");
    
    // =========================================================
    // Static Dispatch (monomorphization)
    // =========================================================
    // When you use generics, Rust creates a SEPARATE copy of the
    // function for each type. Like C++ templates.
    // This is FASTER (no runtime overhead) but creates more code.
    
    println!("--- Static Dispatch (generic functions) ---");
    make_it_speak(&dog);
    make_it_speak(&cat);
    make_it_speak(&robot);
    println!("");
    
    // =========================================================
    // Dynamic Dispatch (trait objects)
    // =========================================================
    // When you use &dyn Trait, Rust uses a VTABLE (virtual table).
    // Like C++ virtual functions. Slower call, but one function works.
    // Good when you have MANY types at runtime (mix of dogs, cats, robots).
    
    println!("--- Dynamic Dispatch (trait objects: &dyn Speak) ---");
    let things: Vec<&dyn Speak> = vec![&dog, &cat, &robot];
    make_all_speak(&things);
    println!("");
    
    // =========================================================
    // Derive Macros
    // =========================================================
    
    println!("--- Derived Traits (#[derive(...)]) ---");
    
    let tool1 = HackerTool {
        name: String::from("USBGuard"),
        version: 1,
        stealth_level: 200,
    };
    
    let tool2 = HackerTool {
        name: String::from("HDDGuard"),
        version: 1,
        stealth_level: 255,
    };
    
    // Debug — thanks to #[derive(Debug)]
    println!("Debug: {:?}", tool1);
    println!("Debug: {:?}", tool2);
    
    // Clone — thanks to #[derive(Clone)]
    let tool1_clone = tool1.clone();  // Deep copy
    println!("Cloned: {:?}", tool1_clone);
    
    // PartialEq — thanks to #[derive(PartialEq)]
    println!("tool1 == tool1_clone: {}", tool1 == tool1_clone);
    println!("tool1 == tool2: {}", tool1 == tool2);
    
    // Default — thanks to #[derive(Default)]
    let default_tool: HackerTool = Default::default();
    println!("Default: {:?}", default_tool);  // Empty strings, 0 versions
    
    // impl Speak in return type
    let default_tool = create_default_tool();
    println!("Default tool speaks: {}", default_tool.speak());
    
    println!("");
    
    // =========================================================
    // ITERATOR TRAIT — The most important trait in Rust
    // =========================================================
    
    println!("--- Iterator Trait (.map, .filter, .collect) ---");
    
    // .iter() — borrows each element
    // .filter() — keep elements matching a condition
    // .map() — transform each element
    // .collect() — collect into a collection (Vec, etc.)
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Chain: filter even numbers, then double them, then collect
    let result: Vec<i32> = numbers
        .iter()                     // Get iterator over &i32
        .filter(|&x| x % 2 == 0)   // Keep only even numbers
        .map(|x| x * 2)            // Double each
        .collect();                 // Collect into Vec<i32>
    
    println!("Numbers: {:?}", numbers);
    println!("Filtered (even) and doubled: {:?}", result);
    
    // .fold() — reduce to a single value
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum of all numbers: {} (using fold)", sum);
    
    // .any() / .all() — check conditions
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("Has even? {} | All positive? {}", has_even, all_positive);
    
    // .find() — find first match
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    println!("First even number: {:?}", first_even);
    
    println!("");
    println!("═══════════════════════════════════════");
    println!("  ✅ Traits = shared behavior (like interfaces)");
    println!("  ✅ impl Trait for Type — give types methods");
    println!("  ✅ Trait bounds — generics with guarantees");
    println!("  ✅ derive — auto-implement common traits");
    println!("  ✅ dyn Trait — dynamic dispatch (virtual)");
    println!("  ✅ Iterator — .map(), .filter(), .fold()");
    println!("  ✅ Next: 10_collections.rs — advanced data structures");
    println!("═══════════════════════════════════════");
}
