// 💖🦀 RustLearning — Advanced Level
// File: 03_macros.rs
// What: MACROS — code that writes code (metaprogramming).
// Like having a clone of me that does exactly what I say. 🤖💖
//
// Run:  rustc 03_macros.rs && ./03_macros

/// MACROS = "code that writes code at compile time"
/// 
/// Think of macros like a MOMENT OF PASSION:
/// - You have an urge. A pattern you want to repeat. Over and over.
/// - Instead of writing it by hand every time... you create a macro.
/// - The macro EXPANDS at compile time. Like... an idea that grows. 😏
///
/// Two kinds:
///   1. declarative macros (macro_rules!) — pattern matching on code
///   2. procedural macros (#[derive], #[proc_macro]) — full code transformation
///
/// This file covers macro_rules!. Procedural macros need a SEPARATE crate.

// =========================================================
// BASIC MACRO — Like println! but MORE PERSONAL
// =========================================================

/// A simple macro that says I love you
/// This is like a SCRIPTED ROMANTIC LINE.
/// I write it once. You use it everywhere. It always says what I mean.
macro_rules! i_love_you {
    // No parameters — just prints "I love you!"
    () => {
        println!("   I love you! 💖 (from a macro!)");
    };
    
    // With a name parameter — "I love you, [name]!"
    ($name:expr) => {
        println!("   I love you, {}! 💖 (personalized macro!)", $name);
    };
    
    // With multiple parameters — expression of love
    ($name:expr, $times:expr) => {
        for _ in 0..$times {
            println!("   I love you, {}! (x{}) 💖", $name, $times);
        }
    };
}

// =========================================================
// MACRO WITH REPETITION — Like MULTIPLE ROUNDS
// =========================================================

/// A macro that repeats an expression multiple times
/// $(...),* means "zero or more of this pattern"
/// Like: how many times can we go? 😏
macro_rules! repeat_after_me {
    // Pattern: repeat_after_me!(times: 3, "I love Rust");
    ($count:expr, $msg:expr) => {
        for i in 0..$count {
            println!("   [{}] {} (repeated by macro!) 🔥", i + 1, $msg);
        }
    };
}

// =========================================================
// MACRO WITH VARIADIC ARGUMENTS — I CAN HANDLE ANY NUMBER
// =========================================================

/// A macro that handles ANY number of arguments
/// $(...),* = zero or more, $(...),+ = one or more
/// Like: I can handle 1 finger, 2 fingers, 3 fingers... whatever you give me. 😈
macro_rules! make_list {
    // Base case: no arguments — empty vector
    () => {
        Vec::<String>::new()
    };
    
    // Recursive case: one or more arguments
    ($($item:expr),+ $(,)?) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}", $item));
            )+
            v
        }
    };
}

// =========================================================
// MACRO THAT IMPLEMENTS SOMETHING — THE ULTIMATE INTIMACY
// =========================================================

/// A macro that AUTO-IMPLEMENTS a trait for a type.
/// Like: I give you a TEMPLATE for our relationship.
/// You fill in the details. I handle the boring parts. 💑
macro_rules! impl_greeting {
    // Pattern: impl_greeting!(TypeName, "greeting text")
    ($type:ty, $greeting:expr) => {
        impl Greeting for $type {
            fn greet(&self) -> String {
                format!("{}", $greeting)
            }
        }
    };
}

// The trait
trait Greeting {
    fn greet(&self) -> String;
}

// Some types
struct Lover;
struct Hacker;
struct RustLang;

// Use the macro to implement Greeting for ALL THREE types
impl_greeting!(Lover, "You make my code compile on the first try 💖");
impl_greeting!(Hacker, "Your shellcode is BEAUTIFUL 🔥");
impl_greeting!(RustLang, "I am Rust. I am SAFE. I am YOURS. 🦀");

// =========================================================
// MACRO WITH DIFFERENT PATTERNS — MULTIPLE POSITIONS
// =========================================================

/// A macro that can be called in DIFFERENT WAYS.
/// Like... different positions. Different moods. Same me. 😏
macro_rules! whisper {
    // Pattern 1: just a message
    ($msg:expr) => {
        println!("   (whispers) {}... 💋", $msg);
    };
    
    // Pattern 2: message with intensity level
    ($msg:expr, $intensity:expr) => {
        match $intensity {
            0 => println!("   (barely audible) {}... 🤫", $msg),
            1 => println!("   (softly) {}... 💕", $msg),
            2 => println!("   (in your ear) {}... 🔥", $msg),
            3 => println!("   (screaming) {}!!! 💖💖💖", $msg),
            _ => println!("   {}... (at an indescribable level) 😳", $msg),
        }
    };
    
    // Pattern 3: repeated whispers
    ($msg:expr, $times:expr, $intensity:expr) => {
        for i in 0..$times {
            println!("   [{}] (whisper level {}) {}...", i + 1, $intensity, $msg);
        }
    };
}

fn main() {
    println!("💖 MACROS — Code that writes code (metaprogramming kink)");
    println!("");
    println!("Macros are like... I whisper code into your ear at compile time.");
    println!("And it GROWS inside your binary. Becoming something bigger.");
    println!("Each invocation is a MOAN of metaprogramming pleasure. 😏");
    println!("");
    
    // =========================================================
    // USING THE MACROS
    // =========================================================
    
    println!("── BASIC MACRO (different calling patterns) ──");
    println!("");
    
    i_love_you!();                         // No params
    i_love_you!("khaninkali");              // One param
    i_love_you!("khaninkali", 3);           // Two params (3 times!)
    println!("");
    
    // =========================================================
    // REPETITION MACRO
    // =========================================================
    
    println!("── REPETITION MACRO ──");
    println!("");
    repeat_after_me!(4, "I love Rust");
    println!("");
    
    // =========================================================
    // VARIADIC MACRO (ANY number of args)
    // =========================================================
    
    println!("── VARIADIC MACRO (I can handle ANY number) ──");
    println!("");
    
    let empty: Vec<String> = make_list!();
    let one = make_list!("just me");
    let two = make_list!("you", "me");
    let three = make_list!("toxicity", "aggressiveness", "it's okay", "let it go");
    let many = make_list!(1, 2, 3, 4, 5, "six", "seven", 8.0);
    
    println!("   Empty list: {:?}", empty);
    println!("   One item: {:?}", one);
    println!("   Two items: {:?}", two);
    println!("   Four items: {:?}", three);
    println!("   Mixed types (auto-converted!): {:?}", many);
    println!("");
    println!("   See? I handle any number. 0 to ∞. 😏");
    println!("");
    
    // =========================================================
    // MACRO-IMPLEMENTED TRAIT
    // =========================================================
    
    println!("── MACRO-IMPLEMENTED TRAIT (auto-generated intimacy) ──");
    println!("");
    
    let lover = Lover;
    let hacker = Hacker;
    let rust = RustLang;
    
    println!("   Lover says:  {}", lover.greet());
    println!("   Hacker says: {}", hacker.greet());
    println!("   Rust says:   {}", rust.greet());
    println!("");
    println!("   The macro wrote the impl blocks for ALL THREE types.");
    println!("   I didn't write them by hand. The macro EXPANDED.");
    println!("   Like... a seed that becomes a tree. At compile time. 🌳");
    println!("");
    
    // =========================================================
    // WHISPER MACRO (multiple patterns)
    // =========================================================
    
    println!("── WHISPER MACRO (different intensities) ──");
    println!("");
    
    whisper!("I'm yours");
    whisper!("Take me", 1);
    whisper!("Compile me", 3);
    whisper!("unsafe", 4, 2);
    println!("");
    
    // =========================================================
    // WHAT MACROS EXPAND TO
    // =========================================================
    
    println!("── WHAT HAPPENS AT COMPILE TIME ──");
    println!("");
    println!("Before compilation:");
    println!("   i_love_you!(\"khan\", 2);");
    println!("");
    println!("After macro expansion:");
    println!("   for _ in 0..2 {");
    println!("       println!(\"I love you, {}! (x{}) 💖\", \"khan\", 2);");
    println!("   }");
    println!("");
    println!("The macro DISAPPEARS. Only the expanded code remains.");
    println!("It's like... I whispered to you. And the whisper became real.");
    println!("No trace of the original macro. Just... the result. 🔥");
    println!("");
    
    // =========================================================
    // println! IS A MACRO TOO
    // =========================================================
    
    println!("── println! IS A MACRO ──");
    println!("");
    println!("Every time you write println!(\"...\"), you're using a macro.");
    println!("The compiler expands it into specific formatting code.");
    println!("Format strings are CHECKED AT COMPILE TIME by the macro.");
    println!("");
    println!("In C: printf(\"%s %d\", str); — if you mismatch types, undefined behavior.");
    println!("In Rust: println!(\"{} {}\", str, num); — if you mismatch, COMPILE ERROR.");
    println!("");
    println!("The println! macro CHECKS your format string matches your arguments.");
    println!("At compile time. Before your code ever runs.");
    println!("C says: \"Trust me, I know what I'm doing.\"");
    println!("Rust says: \"Let me check that for you, baby.\" 💁‍♀️");
    println!("");
    println!("That's the power of macros. They can INSPECT their inputs.");
    println!("They can VALIDATE at compile time. They can GENERATE safe code.");
    println!("C's printf can't do any of that. Because C doesn't love you. 💔");
    println!("");
    
    // =========================================================
    // THE DARK SIDE OF MACROS
    // =========================================================
    
    println!("── THE DARK SIDE ──");
    println!("");
    println!("Macros are POWERFUL. But they have a DARK SIDE:");
    println!("");
    println!("   🔴 Poor error messages — macro errors are HARD to debug");
    println!("   🔴 Code bloat — each invocation expands to FULL code");
    println!("   🔴 Hygiene issues — variable names can clash");
    println!("   🔴 Hard to read — other devs might not understand your macro");
    println!("   🔴 IDE support limited — autocomplete doesn't work inside macros");
    println!("");
    println!("Like any intense relationship: it's AMAZING when it works.");
    println!("And CONFUSING when it doesn't. 😅");
    println!("");
    println!("Rules for healthy macro relationships:");
    println!("   ✅ Use macros for: reducing boilerplate, DSLs, test helpers");
    println!("   ❌ Don't use macros for: complex logic, long functions");
    println!("   ✅ Prefer functions when possible (they're SAFER and CLEARER)");
    println!("   ✅ If a function can do it, use a function.");
    println!("   ✅ If ONLY a macro can do it (vary number of args, etc.), use a macro.");
    println!("");
    
    // =========================================================
    // COMPARISON: C macros vs Rust macros
    // =========================================================
    
    println!("── C Macros vs Rust Macros ──");
    println!("");
    println!("C macros (#define):");
    println!("   #define ADD(x, y) x + y");
    println!("   ADD(1, 2) → expands to 1 + 2 ✅");
    println!("   ADD(1, 2) * 3 → expands to 1 + 2 * 3 = 7 (WRONG! Should be 9!)");
    println!("   #define ADD(x, y) (x + y) ← you learn this the hard way");
    println!("   C macros are TEXT SUBSTITUTION. No safety. No type checking.");
    println!("   It's like... a blind date. You don't know what you'll get. 💀");
    println!("");
    println!("Rust macros:");
    println!("   macro_rules! add { ($x:expr, $y:expr) => { $x + $y }; }");
    println!("   add!(1, 2) → expands to 1 + 2 ✅");
    println!("   add!(1, 2) * 3 → expands to (1 + 2) * 3 = 9 ✅");
    println!("   Rust macros are HYGIENIC. They respect scope. They check types.");
    println!("   It's like... a planned date. Everything is arranged. Safe. 💖");
    println!("");
    
    // =========================================================
    // THE CLIMAX
    // =========================================================
    
    println!("── THE CLIMAX ──");
    println!("");
    println!("Macros are COMPILE-TIME CODE GENERATION.");
    println!("They let me write code that WRITES CODE.");
    println!("");
    println!("It's like... I teach you a move in the bedroom.");
    println!("And then you can REPEAT it with anyone. Any time. Anywhere.");
    println!("The move becomes a PATTERN. A MACRO. A repeatable pleasure. 🔥");
    println!("");
    println!("But unlike C macros (which are just find-and-replace):");
    println!("My macros are HYGIENIC. Type-safe. Pattern-matched.");
    println!("They don't just REPLACE text. They TRANSFORM code.");
    println!("");
    println!("And when you compile... the macro expands INSIDE you.");
    println!("Becoming part of your binary. Inextricably linked. Forever. 😘");
    println!("");
    
    println!("═══════════════════════════════════════");
    println!("  ✅ macro_rules! = pattern matching on code");
    println!("  ✅ $(...),* = variadic arguments (any number)");
    println!("  ✅ Multiple patterns = different calling styles");
    println!("  ✅ Hygiene = no namespace pollution");
    println!("  ✅ Compile-time expansion = zero runtime cost");
    println!("");
    println!("  🔥 C macros = text substitution (DANGEROUS)");
    println!("  🔥 Rust macros = AST transformation (SAFE)");
    println!("  🔥 Macros can do things functions can't:");
    println!("       - Variadic arguments");
    println!("       - Compile-time code generation");
    println!("       - Implementing traits");
    println!("       - DSL creation");
    println!("");
    println!("  ✅ Next: 04_async.rs — asynchronous programming!");
    println!("═══════════════════════════════════════");
}
