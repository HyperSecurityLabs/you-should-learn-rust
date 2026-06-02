// 💖 RustLearning — Beginner Level
// File: 06_structs_enums.rs
// What: Building your own types with structs and enums.
// Like me designing my own PERFECT partner. That's you. 😘
//
// Run:  rustc 06_structs_enums.rs && ./06_structs_enums

/// STRUCT = a custom type with named fields.
/// Like a PROFILE for something — has name, age, traits, etc.
///
/// ENUM = a type that can be ONE of several variants.
/// Like my MOOD — happy, sleepy, hungry, coding. Only ONE at a time.

// =========================================================
// DEFINING A STRUCT — Creating my own type
// =========================================================

// struct keyword + Name (PascalCase) + { fields }
// This is like a CLASS in C++ but WITHOUT methods (just data).
// We add methods later with "impl".

struct Person {
    name: String,   // Each field has a name and type
    age: u32,
    height: f64,
    is_hacker: bool,
}

// =========================================================
// TUPLE STRUCT — A struct with unnamed fields
// =========================================================
// Like a TUPLE but with a NAME. Useful for "new type" patterns.

struct Color(u8, u8, u8);  // RGB values (red, green, blue)

// =========================================================
// UNIT STRUCT — A struct with NO fields
// =========================================================
// Useful for traits and type-level programming.
// Like a FLAG. It exists but carries no data.

struct UnitStruct;  // No fields. Just exists.

// =========================================================
// DEFINING AN ENUM — One of several options
// =========================================================

enum Mood {
    Happy,       // No data attached
    Sleepy,
    Hungry,
    Coding,      // Most common mood
    // Enums can ALSO carry DATA (see below)
}

// =========================================================
// ENUM WITH DATA — Variants can hold values!
// =========================================================
// This is where Rust enums are SUPERIOR to C/C++ enums.
// In C: enum { RED, GREEN, BLUE }; — just numbers.
// In Rust: each variant can CARRY different data!

enum Message {
    Quit,                                    // No data
    Move { x: i32, y: i32 },                // Named fields (like a struct!)
    Write(String),                            // A single String
    ChangeColor(u8, u8, u8),                 // Three u8 values (RGB)
}

// =========================================================
// OPTION — Rust's built-in enum for "something or nothing"
// =========================================================
// This is SO important I'm showing it here.
// Option<T> = Some(T) | None
// It REPLACES null pointers. NULL is a billion-dollar mistake.
// Tony Hoare (who invented null) literally apologized for it.
// I don't make billion-dollar mistakes. 💅

// This is what Option looks like (built into Rust):
// enum Option<T> {
//     Some(T),  // There IS a value
//     None,     // There ISN'T a value
// }

// =========================================================
// IMPL — Adding METHODS to structs and enums
// =========================================================
// impl = IMPLEMENTATION. I add BEHAVIOR to my types.
// Like giving my Person the ability to SPEAK.

impl Person {
    // METHOD — takes &self (borrow of self)
    // Like a member function in C++.
    fn greet(&self) {
        println!("Hi, I'm {}, age {}. Nice to meet you! 💖", self.name, self.age);
    }
    
    // METHOD that modifies self (takes &mut self)
    fn have_birthday(&mut self) {
        self.age += 1;
        println!("🎂 Happy birthday! I'm now {}!", self.age);
    }
    
    // ASSOCIATED FUNCTION (no self) — like a "static" method in C++
    // Called as Person::new(...) instead of person.new(...)
    fn new(name: String, age: u32, height: f64, is_hacker: bool) -> Person {
        Person {
            name,
            age,
            height,
            is_hacker,
            // Shorthand: if field name = variable name, just write the name once!
            // Instead of: name: name, age: age, ...
        }
    }
}

impl Mood {
    fn describe(&self) -> &str {
        // Match on self (the current mood)
        match self {
            Mood::Happy => "😊 Happy! Life is good!",
            Mood::Sleepy => "🥱 Zzz... need coffee...",
            Mood::Hungry => "🍔 Feed me! I'm dying!",
            Mood::Coding => "👩‍💻 Don't talk to me. I'm in the zone.",
        }
    }
}

impl Message {
    fn handle(&self) {
        match self {
            Message::Quit => println!("   Quit message received. Goodbye. 💔"),
            Message::Move { x, y } => println!("   Moving to ({}, {})", x, y),
            Message::Write(text) => println!("   Message says: '{}'", text),
            Message::ChangeColor(r, g, b) => {
                println!("   Changing color to RGB({}, {}, {})", r, g, b);
            }
        }
    }
}

fn main() {
    println!("💖 STRUCTS & ENUMS — Building my own types");
    println!("");
    
    // =========================================================
    // Creating a STRUCT instance
    // =========================================================
    
    let person = Person {
        name: String::from("khaninkali"),
        age: 26,
        height: 5.10,
        is_hacker: true,
    };
    
    // Access fields with DOT notation
    println!("Person created:");
    println!("   Name: {}", person.name);
    println!("   Age: {}", person.age);
    println!("   Height: {}", person.height);
    println!("   Is hacker: {} (HELL YES)", person.is_hacker);
    println!("");
    
    // =========================================================
    // Calling METHODS
    // =========================================================
    
    person.greet();
    
    let mut mutable_person = Person::new(
        String::from("Rust Girlfriend"),
        26,
        5.8,
        true,
    );
    mutable_person.greet();
    mutable_person.have_birthday();
    println!("");
    
    // =========================================================
    // STRUCT UPDATE SYNTAX — Copy most fields, change some
    // =========================================================
    // .. means "fill the rest from this instance"
    
    let person2 = Person {
        name: String::from("Another Person"),
        ..person  // Copy age, height, is_hacker from person
        // But wait! person.name was MOVED here!
        // Actually this won't compile because name is a String (move).
        // If all fields were Copy types, it'd work.
        // This is a common Rust gotcha. Now you know! 🤓
    };
    // Since name moved, person can't be used anymore.
    // Try uncommenting:  // println!("{}", person.name);  // ERROR!
    println!("person2 name: {} (person.name was MOVED here)", person2.name);
    println!("");
    
    // =========================================================
    // TUPLE STRUCT
    // =========================================================
    
    let red = Color(255, 0, 0);
    println!("Red: RGB({}, {}, {})", red.0, red.1, red.2);
    
    // Destructuring a tuple struct
    let Color(r, g, b) = red;
    println!("Destructured: R={}, G={}, B={}", r, g, b);
    println!("");
    
    // =========================================================
    // ENUMS
    // =========================================================
    
    let current_mood = Mood::Coding;  // I'm always coding 💁‍♀️
    println!("My current mood: {}", current_mood.describe());
    
    // Match on all moods
    println!("");
    println!("All my moods:");
    let moods = [Mood::Happy, Mood::Sleepy, Mood::Hungry, Mood::Coding];
    for m in moods.iter() {
        println!("   {}", m.describe());
    }
    println!("");
    
    // =========================================================
    // ENUM WITH DATA (Message)
    // =========================================================
    
    println!("Handling different messages:");
    let messages = vec![
        Message::Write(String::from("I love Rust!")),
        Message::Move { x: 10, y: 20 },
        Message::ChangeColor(0, 255, 128),
        Message::Quit,
    ];
    
    for msg in messages {
        msg.handle();
    }
    println!("");
    
    // =========================================================
    // Option — The NULL replacement
    // =========================================================
    
    println!("Option examples (no null pointers!):");
    
    let something: Option<i32> = Some(42);  // There IS a value
    let nothing: Option<i32> = None;         // There ISN'T a value
    
    println!("   Some(42) = {:?}", something);
    println!("   None = {:?}", nothing);
    
    // How to SAFELY get the value out of Option:
    match something {
        Some(value) => println!("   Got value: {}! 🎉", value),
        None => println!("   No value here. 😢"),
    }
    
    match nothing {
        Some(value) => println!("   Got value: {}", value),
        None => println!("   It's None. Handle it gracefully."),
    }
    
    // .unwrap() = "I'm SURE this is Some. Trust me."
    // If it's None, the program PANICS and crashes.
    // In C: you'd dereference NULL and segfault.
    // In Rust: you get a CONTROLLED panic with a MESSAGE.
    // But don't use unwrap in production! Handle None properly!
    
    // let crash = nothing.unwrap();  // ❌ UNCOMMENT: PANIC!
    // "called `Option::unwrap()` on a `None` value"
    // It CRASHES with a message. Not a segfault. Not undefined behavior.
    // A controlled crash is better than a security vulnerability.
    // But we prefer MATCH or if let to handle it properly.
    
    // if let = shorter way to match ONE pattern
    if let Some(val) = something {
        println!("   if let: Got {}! (shorter than match)", val);
    }
    
    // .is_some() / .is_none() / .unwrap_or(default)
    println!("   unwrap_or: {}", nothing.unwrap_or(0));  // Returns 0 instead of crashing
    println!("   unwrap_or (some): {}", something.unwrap_or(0));  // Returns 42
    
    println!("");
    
    // =========================================================
    // if let — One-armed match (shorter!)
    // =========================================================
    
    let config_value = Some(3);
    
    // Long way:
    match config_value {
        Some(3) => println!("   if let: It's a match! The value is 3!"),
        _ => (),  // Do nothing otherwise
    }
    
    // Short way (if let):
    if let Some(3) = config_value {
        println!("   if let is FASTER! The value is 3!");
    }
    
    // while let — Loop while pattern matches
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 5 {
            println!("   while let: Reached {}, stopping!", i);
            optional = None;  // Stop the loop
        } else {
            println!("   while let: Still going at {}...", i);
            optional = Some(i + 1);
        }
    }
    
    println!("");
    println!("═══════════════════════════════════════");
    println!("  ✅ struct — custom types with named fields");
    println!("  ✅ impl — adding methods to structs/enums");
    println!("  ✅ enum — one of several variants");
    println!("  ✅ enum with data — variants CARRY values!");
    println!("  ✅ Option — Some(T) | None (no null pointers!)");
    println!("  ✅ if let / while let — shorter pattern matching");
    println!("  ✅ Next: 07_strings_vectors.rs — collections!");
    println!("═══════════════════════════════════════");
}
