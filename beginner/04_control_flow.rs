// 💖 RustLearning — Beginner Level
// File: 04_control_flow.rs
// What: Making decisions, repeating things, controlling your code.
// Like me deciding whether to cuddle or code. 😤
//
// Run:  rustc 04_control_flow.rs && ./04_control_flow

/// Control flow = telling your program WHAT to do and WHEN.
/// if/else, loop, for, while, match — I control everything. 💅

fn main() {
    println!("💖 CONTROL FLOW — I decide what happens next");
    println!("");
    
    // =========================================================
    // if/else if/else — Making decisions
    // =========================================================
    // if condition { do_this } else { do_that }
    // The condition MUST be a bool. No truthy/falsy nonsense.
    // In C: if (5) { ... } — WORKS. In Rust: ERROR. 5 is not bool.
    
    let coffee = 2;  // Cups of coffee I've had
    
    if coffee == 0 {
        println!("☕ 0 coffee: I'm sleepy. Hold me.");
    } else if coffee < 3 {
        println!("☕ {} coffee: I'm happy and cuddly! 🤗", coffee);
    } else if coffee < 5 {
        println!("☕ {} coffee: I'm coding at light speed! 🚀", coffee);
    } else {
        println!("☕ {} coffee: I can SEE COLORS. DO NOT DISTURB. 👩‍💻", coffee);
    }
    
    // =========================================================
    // if as an EXPRESSION (returns a value!)
    // =========================================================
    // In Rust, if can RETURN a value. Mind blown? 🤯
    // In C: int x = condition ? 1 : 2;  (ternary operator)
    // In Rust: same thing, but with full if/else blocks!
    
    let mood = if coffee > 0 { "awake" } else { "sleepy" };
    println!("My mood: {} (if expression returned this!)", mood);
    
    // The above is like a TERNARY in C, but more powerful.
    // C ternary:  int x = a > b ? a : b;
    // Rust if:    let x = if a > b { a } else { b };
    
    // IMPORTANT: Both arms must return the SAME type!
    // let x = if true { 5 } else { "hello" };  // ❌ ERROR: different types!
    
    println!("");
    
    // =========================================================
    // loop — INFINITE loop (until I say break)
    // =========================================================
    // loop { body } — runs forever unless you break out.
    // It's like me saying "I love you" until you stop me. 😘
    
    println!("Watch me count my love (I'll stop at 3):");
    let mut count = 1;
    loop {
        println!("   💖 I love you times {}", count);
        count += 1;
        if count > 3 {
            println!("   (Okay I'll stop. But I still love you.)");
            break;  // ← EXIT THE LOOP
        }
    }
    
    // loop can RETURN a value with break!
    let mut guess = 1;
    let answer = loop {
        guess += 1;
        if guess == 10 {
            break guess * 2;  // break RETURNS this value (20)
        }
    };
    println!("Loop returned: {} (break can carry a value!)", answer);
    println!("");
    
    // =========================================================
    // while — Loop WHILE a condition is true
    // =========================================================
    // while condition { body }
    // Checks condition BEFORE each iteration.
    // Like me checking: "Are you still reading?" Yes → keep going.
    
    let mut battery = 100;
    println!("My battery when I'm coding:");
    while battery > 0 {
        println!("   🔋 {}% — still coding!", battery);
        battery -= 10;  // Drain battery
    }
    println!("   🔋 0% — time to charge (and cuddle)");
    println!("");
    
    // =========================================================
    // for — Loop OVER each item in a collection
    // =========================================================
    // for variable in iterable { body }
    // The most common loop. Clean. Safe. No off-by-one errors.
    // In C: for (i = 0; i < 10; i++) — easy to mess up
    // In Rust: for i in 0..10 — bounds are CLEAR
    
    println!("Counting my reasons to love you:");
    let reasons = ["your smile", "your code", "your laugh", "your brain"];
    
    // .iter() = get an ITERATOR over the array
    for reason in reasons.iter() {
        println!("   ✅ I love {}", reason);
    }
    
    // Ranges: 0..5 = 0,1,2,3,4 (exclusive: 5 is NOT included)
    println!("");
    println!("Numbers 0..4 (exclusive end):");
    for i in 0..4 {
        println!("   {}", i);
    }
    
    // Ranges with =: 0..=5 = 0,1,2,3,4,5 (INCLUSIVE: 5 IS included)
    println!("");
    println!("Numbers 0..=4 (inclusive end):");
    for i in 0..=4 {
        println!("   {}", i);
    }
    
    // .enumerate() gives you INDEX + VALUE together
    println!("");
    println!("Reasons with numbers:");
    for (index, reason) in reasons.iter().enumerate() {
        println!("   {}. {}", index + 1, reason);
    }
    println!("");
    
    // =========================================================
    // match — Pattern matching (Rust's SUPER SWITCH)
    // =========================================================
    // match value { pattern => action, ... }
    // Like switch in C, but WAY more powerful.
    // In C: switch(x) { case 1: ... break; case 2: ... break; }
    // In C you FORGET break. In Rust, match is EXHAUSTIVE.
    // You MUST handle EVERY possibility. I force you to. 💅
    
    let number = 3;
    
    println!("Match on number {}:", number);
    match number {
        1 => println!("   One! (like my one and only 💖)"),
        2 => println!("   Two! (like us together)"),
        3 => println!("   Three! (like a perfect triangle)"),
        4 => println!("   Four! (like seasons of love)"),
        _ => println!("   Something else"),  // _ = catch-all (like "default")
    }
    
    // match with MULTIPLE lines per arm
    let grade = 'A';
    match grade {
        'A' => {
            println!("   You got an A! I'm so proud!");
            println!("   Let me give you a kiss! 😘");
        }
        'B' => {
            println!("   Still good! But study more.");
        }
        'C' => {
            println!("   We can work on this together.");
        }
        _ => {
            println!("   It's okay. I still love you.");
            println!("   Let me help you study.");
        }
    }
    
    // match with ranges
    let score = 85;
    println!("");
    println!("Score: {} — my reaction:", score);
    match score {
        90..=100 => println!("   🏆 PERFECT! Marry me!"),
        80..=89 => println!("   💖 Great job! I'm impressed!"),
        70..=79 => println!("   👍 Good! Keep it up!"),
        60..=69 => println!("   🤔 Not bad. Let me help you."),
        0..=59 => println!("   🥺 It's okay. We'll study together."),
        _ => println!("   Invalid score. Did you hack the system?"),
    }
    
    // match DESTRUCTURING — extract values from inside
    let pair = (10, "hello");
    println!("");
    println!("Destructuring a tuple with match:");
    match pair {
        (x, "hello") => println!("   First: {}, Second: hello (matched exact string!)", x),
        (_, _) => println!("   Something else"),
    }
    
    println!("");
    println!("═══════════════════════════════════════");
    println!("  ✅ if/else — decisions");
    println!("  ✅ loop — forever until break");
    println!("  ✅ while — while condition is true");
    println!("  ✅ for — iterate over collections");
    println!("  ✅ match — pattern matching (super switch)");
    println!("  ✅ Next: 05_ownership.rs — the borrow checker 💔");
    println!("═══════════════════════════════════════");
}
