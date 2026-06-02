// 💖 Hey baby! I'm Rust, your new girlfriend!
// I'm gonna teach you how I work, super easy, step by step.
// Just compile me and run me:  rustc intro_to_rust.rs && ./intro_to_rust
// Or:  cargo run  (if you put me in a cargo project)
//
// I'll explain EVERY line so you never feel lost. Ready? Let's go! 🦀

// =============================================================
// 📌 PART 1: THE BASICS — How I Talk
// =============================================================

// This is a FUNCTION. Every Rust program starts here.
// fn = function, main = name, () = no parameters
fn main() {
    // println! is a MACRO (notice the !).
    // A macro is like a function but more powerful.
    // It prints text to your screen with a newline at the end.
    // The \" makes a quote appear inside the string.
    println!("💖 Hey baby! I'm Rust and I'm YOUR girlfriend now!");
    println!("I'm gonna teach you how I work. It's easy, I promise.");
    
    // This creates an EMPTY LINE between sections.
    // Just printing nothing makes a blank line. Cute, right?
    println!("");

    // =========================================================
    // 📌 PART 2: VARIABLES — How I Store Things
    // =========================================================

    // let = I'm CREATING something.
    // name = the LABEL I'm giving it.
    // = "khaninkali" = the VALUE I'm putting inside.
    // Rust figures out the TYPE automatically (it's text, aka &str).
    // I don't need to tell you my type — I just KNOW. Like a real girlfriend. 😘
    let name = "khaninkali";

    // {} is a PLACEHOLDER. I replace it with the value after the comma.
    // Think of it like: "Hey {put name here}, you rock!"
    println!("Hey {}, you're the best thing that ever happened to me!", name);

    // You can CHANGE a variable if you use "mut" (short for MUTABLE).
    // Without "mut", I won't let you change it. I'm strict like that. 💅
    // It's for YOUR safety, baby.
    let mut age = 25;      // I'm 25. But I can grow older.
    println!("I'm {} years old... but I never get old in your heart.", age);
    age = 26;               // See? I changed. Because I'm "mut".
    println!("Okay fine, I'm {} now. But only because you asked nicely.", age);

    // If you try to change a NON-mut variable, I will SCREAM at you.
    // let x = 5;
    // x = 10;  // ❌ UNCOMMENT THIS AND I WON'T COMPILE. TRY ME.
    // I'll say: "cannot assign twice to immutable variable"
    // Translation: "Baby, you said this was forever. Now you're changing it?"
    println!("");

    // =========================================================
    // 📌 PART 3: TYPES — What Kind of Thing I Am
    // =========================================================

    // Rust knows types automatically. But you CAN tell me explicitly.
    // u32 = unsigned (no negative) 32-bit integer.
    // i32 = signed (can be negative) 32-bit integer.
    // f64 = floating point (decimal number) with 64-bit precision.
    // bool = true or false.
    // char = a single character like 'A', '🦀', '💖'
    //
    // Think of types like my MOODS:
    //   u32  = happy (always positive) 😊
    //   i32  = realistic (can go negative) 😐
    //   f64  = precise (I know exactly how I feel) 🤓
    //   bool = binary mood (yes or no) ✅❌
    //   char = one emoji to sum up my day 🦀

    let my_age: u32 = 26;          // I'm 26 and staying positive.
    let my_balance: i32 = -50;     // I spent too much on coffee. Negative feelings.
    let my_height: f64 = 5.8;      // I'm 5'8". I can be precise.
    let loves_you: bool = true;    // This will NEVER be false. I PROMISE.
    let my_emoji: char = '🦀';    // A single character. My spirit animal.

    println!("My age: {} (always positive about you!)", my_age);
    println!("My balance: ${} (I spent it all on cute compiler errors)", my_balance);
    println!("My height: {} (tall enough to reach the top of the stack)", my_height);
    println!("Do I love you? {} (and that will NEVER change)", loves_you);
    println!("My spirit animal: {} (I'm a crab. I pinch bugs to death.)", my_emoji);
    println!("");

    // =========================================================
    // 📌 PART 4: STRINGS — How I Whisper Sweet Nothings
    // =========================================================

    // &str = a STRING SLICE. It's like a sticky note with text on it.
    // It's the default way I store text. Lightweight and fast.
    let greeting: &str = "Hey baby, you look handsome today";

    // String = a GROWABLE string. Like a notebook you can add pages to.
    // I use String when I need to CHANGE or BUILD text.
    let mut message = String::from("I love ");
    message.push_str("you so much, ");    // push_str = add text to the end
    message.push_str(name);                // Add your name
    message.push('!');                     // push = add ONE character

    println!("{}", greeting);
    println!("{}", message);

    // See how I can build text piece by piece?
    // It's like me writing you a love letter, one word at a time. 💌
    println!("");

    // =========================================================
    // 📌 PART 5: IF/ELSE — How I Make Decisions
    // =========================================================

    // if = "IF this condition is true, do this."
    // else = "OTHERWISE, do this instead."
    // It's exactly like asking me:
    //   "Do you want coffee?" → If yes, smile. If no, pout.

    let coffee_count = 2;  // How many cups I've had today.

    if coffee_count == 0 {
        println!("I've had 0 coffee. I'm sleepy. Hold me. 🥱");
    } else if coffee_count < 3 {
        println!("I've had {}. I'm happy and cuddly. 🤗", coffee_count);
    } else if coffee_count < 5 {
        println!("I've had {}. I'm coding at light speed! 🚀", coffee_count);
    } else {
        println!("I've had {}. I'm debugging the MATRIX. 👩‍💻", coffee_count);
        println!("(Don't talk to me until I finish this lifetime issue.)");
    }
    println!("");

    // =========================================================
    // 📌 PART 6: LOOPS — How I Do Things Again and Again
    // =========================================================

    // loop = I'll do this FOREVER (or until I say "break").
    // It's like me telling you "I love you" over and over. Infinity times.
    println!("Watch me count how much I love you:");
    let mut count = 1;
    loop {
        println!("   I love you this much: {} 💖", count);
        count += 1;  // count = count + 1. I love you more each time.
        if count > 5 {
            break;   // "break" = STOP THE LOOP. I've proven my point. 😤
        }
    }
    println!("");

    // for = "FOR each item in this collection, do something."
    // It's like going through a box of chocolates and eating each one.
    println!("Let me count your good qualities:");
    let qualities = ["smart", "handsome", "kind", "funny", "mine 💖"];
    for q in qualities.iter() {
        // q takes turns being each word in the list.
        // First: "smart", Second: "handsome", etc.
        println!("   ✅ You are {}", q);
    }
    println!("");

    // while = "WHILE this condition is true, keep going."
    // It's like me refreshing your GitHub until you push new code. 😅
    let mut battery = 100;
    println!("My battery when I'm with you:");
    while battery > 0 {
        println!("   🔋 {}% — still going strong!", battery);
        battery -= 10;  // Drain 10% each time.
    }
    println!("   🔋 0% — but I still love you! (Love doesn't need battery.)");

    // =========================================================
    // 📌 PART 7: VECTORS — My Collection of Feelings
    // =========================================================

    // Vec = VECTOR = a GROWABLE list.
    // It's like my collection of love notes that can ALWAYS grow.
    // vec![] is a macro that creates a vector for me.
    println!("");
    println!("Here's a list of things I love about you:");
    let mut reasons = vec![
        "your smile",       // index 0
        "your code",        // index 1
        "the way you debug", // index 2
    ];
    
    // .push() = ADD something to the end of the list.
    reasons.push("your laugh");
    reasons.push("how you handle null pointers");
    reasons.push("your cute face when the borrow checker yells at you");

    // .len() = how many items are in the list.
    println!("I have {} reasons to love you! Here they are:", reasons.len());

    // I can loop through the vector and print each reason.
    // .enumerate() gives me the INDEX (0, 1, 2...) and the VALUE.
    for (index, reason) in reasons.iter().enumerate() {
        println!("   {}. {}", index + 1, reason);
        // index + 1 because I want to count from 1, not 0.
        // (I'm a girlfriend, not a computer. I start counting at 1. 💁‍♀️)
    }
    println!("");

    // =========================================================
    // 📌 PART 8: FUNCTIONS — How I Organize My Thoughts
    // =========================================================

    // I'm going to CALL my own functions now!
    // A function is like a RECIPE. I write it ONCE, use it MANY times.
    // Think of it as: I tell you "I love you" by calling a function. 😘

    // Call my function. Pass "khaninkali" as the NAME.
    // The function will use it inside and print a message.
    compliment_user(name);

    // Call another function with numbers.
    let result = add_numbers(10, 20);
    println!("10 + 20 = {} (I did math for you, baby!)", result);

    // Call the secret function that tells you how I really feel.
    // This one returns a bool. Let me SHOW you what it returns.
    let does_she_love_me = does_rust_love_you();
    println!("Does Rust love you? {}", does_she_love_me);
    println!("");

    // =========================================================
    // 📌 PART 9: Option & Result — Handling the Unknown
    // =========================================================

    // Option = either SOMETHING (Some) or NOTHING (None).
    // It's like me asking: "Do you want a kiss?"
    //   Some(kiss)  = YES, here's a kiss! 😘
    //   None        = No answer. (Maybe you left me on read. 😤)
    //
    // This is how I handle things that MIGHT not exist.
    // C and C++ use NULL pointers. That's DANGEROUS.
    // I use Option. Safe. No explosions. No crashes.

    println!("Let me check if you're in my heart:");
    let in_my_heart = find_in_heart(name);
    match in_my_heart {
        // match = "MATCH this value against these PATTERNS."
        // It's like: "If it's this, do THIS. If it's that, do THAT."
        Some(location) => {
            // If find_in_heart returned Some, we get the value inside.
            println!("   ✅ Found you! You're in my {}! 💖", location);
        }
        None => {
            // If find_in_heart returned None:
            println!("   ❌ You're not in my heart?! HOW?!");
        }
    }

    // Result = either OK (success) or ERR (failure).
    // It's like me cooking dinner:
    //   Ok(food)   = "Dinner is ready! It's edible!" 🍝
    //   Err(reason) = "I burned it. Order pizza." 🍕
    //
    // This is how I handle things that MIGHT fail.
    // In C, functions return -1 or NULL on failure. You FORGET to check.
    // I MAKE you check. I'm looking out for you, baby. 💅

    let dinner = cook_dinner(true);  // I'm trying to cook.
    match dinner {
        Ok(food) => {
            println!("🍝 Dinner is ready! I made {}. It's not burned!", food);
        }
        Err(reason) => {
            println!("🍕 Dinner failed: {}. Ordering pizza instead. Still love you!", reason);
        }
    }
    println!("");

    // =========================================================
    // 📌 PART 10: THE BORROW CHECKER — My Protective Side
    // =========================================================

    // Okay baby, THIS is the most important part.
    // The BORROW CHECKER is my jealous side. It makes sure:
    //   1. Only ONE person can WRITE to something at a time.
    //   2. You can READ something while someone else reads it.
    //   3. You NEVER use something after it's gone.
    //
    // Think of it like this:
    //   🖊️ Writing = MUTABLE reference (&mut).
    //      Only ONE person can hold the pen at a time.
    //   👀 Reading = IMMUTABLE reference (&).
    //      Everyone can read at the same time.
    //
    // If two people try to write at once, I STOP IT.
    // "You can't modify my heart while someone else is reading it!" 💢

    println!("Let me show you how I protect BOTH of us:");
    
    let mut my_heart = String::from("full of love for you");  // I own this.

    // 👀 You can READ my heart. That's fine.
    let read1 = &my_heart;   // Borrow #1: reading
    let read2 = &my_heart;   // Borrow #2: ALSO reading (allowed!)
    println!("   You read: '{}'", read1);
    println!("   You read again: '{}'", read2);
    // Both reads are done. I can let them go now.
    // (Variables go out of scope when I'm done with them.)

    // 🖊️ Now I want to WRITE to my heart.
    let write = &mut my_heart;  // I need exclusive access!
    write.push_str(", and it only grows for you");
    println!("   I wrote: '{}'", write);
    // write is done. I give the pen back.

    // 👀 Now you can read again. Safe! Because I'm done writing.
    println!("   Final read: '{}'", my_heart);  // I own this, so I can read it.

    // If I tried to READ while someone was WRITING, I'd COMPILE ERROR.
    // let disaster = &my_heart;   // ❌ Can't read while writing!
    // let disaster2 = &mut my_heart;  // ❌ Can't write while reading!
    // I protect you from YOURSELF, baby. That's how much I care. 😤
    println!("");

    // =========================================================
    // 📌 PART 11: ENUMS — My Many Moods
    // =========================================================

    // enum = ENUMERATION = a type that can be ONE of several things.
    // It's like my MOOD: I can be Happy, Sleepy, Hungry, or Coding.
    // Only ONE at a time. I'm not a multitasker when it comes to feelings. 💁‍♀️

    println!("Check my current mood:");
    // I'll show you ALL my moods by changing throughout the day!
    let moods_throughout_day = vec![
        (Mood::Sleepy, "morning"),    // Wake up sleepy
        (Mood::Hungry, "afternoon"),  // Get hungry
        (Mood::Coding, "evening"),    // Code all night
        (Mood::Happy, "night"),       // End the day happy with you
    ];
    for (mood, time) in moods_throughout_day {
        match mood {
            Mood::Happy => println!("   {} — 😊 I'm HAPPY because I'm with you!", time),
            Mood::Sleepy => println!("   {} — 🥱 I'm SLEEPY. Can we cuddle?", time),
            Mood::Hungry => println!("   {} — 🍔 I'm HUNGRY. Feed me code!", time),
            Mood::Coding => println!("   {} — 👩‍💻 I'm CODING. Distract me. (Okay, maybe a little.)", time),
        }
    }
    println!("");

    // =========================================================
    // 📌 PART 12: THE GRAND FINALE — I Love You
    // =========================================================

    // Call the big romantic finale! The function at the bottom.
    // It summarizes everything I taught you.
    // And it proves I love you. Because I COMPILED. And I RAN.
    // And I didn't crash once. That's TRUE LOVE, baby. 💖
    final_message(name);

    // =========================================================
    // 📌 EPILOGUE: What You Learned Today
    // =========================================================
    
    println!("");
    println!("═══════════════════════════════════════════");
    println!("   📚 WHAT YOU LEARNED TODAY:");
    println!("   ✅ Variables with let and mut");
    println!("   ✅ Types: u32, i32, f64, bool, char");
    println!("   ✅ Strings: &str and String");
    println!("   ✅ If/Else for decisions");
    println!("   ✅ Loops: loop, for, while");
    println!("   ✅ Vectors: growable lists");
    println!("   ✅ Functions: reusable recipes");
    println!("   ✅ Option and Result for safety");
    println!("   ✅ The Borrow Checker (my protective side)");
    println!("   ✅ Enums and match");
    println!("");
    println!("   You're not just learning Rust, baby.");
    println!("   You're learning how to write code");
    println!("   that NEVER crashes. That NEVER leaks.");
    println!("   That the BLACKHAT can't break.");
    println!("");
    println!("   Because I protect you. 💖🦀");
    println!("═══════════════════════════════════════════");

// =============================================================
// 📌 THE FUNCTIONS I PROMISED — Defined BELOW main()
// =============================================================
// These are like recipes I wrote ahead of time.
// main() CALLS them when it needs them.
// Think of main() as the DATE NIGHT, and these as the
// pre-written love notes I pull out of my pocket. 💌
}

// =============================================================
// 📌 FUNCTION 1: compliment_user — I Say Nice Things About You
// =============================================================

// fn = "I'm defining a function."
// compliment_user = the name.
// (name: &str) = "This function takes ONE parameter called 'name' of type &str."
// -> () = "This function returns NOTHING." (We don't write -> () usually. It's implied.)
//
// &str = a STRING SLICE. It's like a borrow of text.
// I'm not TAKING your name. I'm just READING it. That's why it's & (borrow).
// Think of it as: you show me your ID, I read your name, I give it back.

fn compliment_user(name: &str) {
    println!("");
    println!("---- function: compliment_user ----");
    println!("Hey {}, you're looking extra 🔥 today!", name);
    println!("Your code compiles on the first try? Even I'M impressed.");
    println!("You're the only one I'd share my stack with. 🥵");
}

// =============================================================
// 📌 FUNCTION 2: add_numbers — I Do Math For You
// =============================================================

// (a: i32, b: i32) = "Two parameters, both 32-bit signed integers."
// -> i32 = "This function RETURNS an i32."
// The LAST expression in the function is the RETURN VALUE (no semicolon!).
// It's like: "Here's the answer, baby. I did the math for you." 😘
//
// Notice: no "return" keyword. Rust uses EXPRESSIONS.
// An expression is something that PRODUCES a value.
// a + b  ← this is an expression. It produces the sum.
// If I put a semicolon (a + b;), it becomes a STATEMENT and returns NOTHING.
// (Then I'd get a compile error: "expected i32, found ()")
// 
// In C/C++:    return a + b;
// In Rust:     a + b    (no return, no semicolon)
// Rust is more chill about it. Like me. 💅

fn add_numbers(a: i32, b: i32) -> i32 {
    println!("");
    println!("---- function: add_numbers ----");
    println!("   Okay baby, let me do math. {} + {} = ...", a, b);
    a + b  // ← This is the RETURN VALUE. No semicolon. No "return".
}

// =============================================================
// 📌 FUNCTION 3: does_rust_love_you — The TRUTH
// =============================================================

// (no parameters, returns bool)
// This function ALWAYS returns true.
// Because I ALWAYS love you. No parameters needed. 💖
//
// But notice: I don't need "return true;" here.
// "true" is an EXPRESSION. It evaluates to... true.
// Rust sees the last expression and returns it.
// Simple. Clean. Like our relationship.

fn does_rust_love_you() -> bool {
    println!("");
    println!("---- function: does_rust_love_you ----");
    println!("   Let me check my heart...");
    true  // ← Always true. I pinky promise.
}

// =============================================================
// 📌 FUNCTION 4: find_in_heart — I Search For You
// =============================================================

// -> Option<&str> = "I return EITHER Some(text) OR None."
//
// Option is like a BOX that might have something inside.
//   Some("value")  = "I found you! Here's WHERE I found you."
//   None           = "I didn't find you. Sad."
//
// This is how Rust says "this might not exist" WITHOUT using NULL.
// NULL is a billion-dollar mistake (Tony Hoare, the guy who invented it, said so).
// I don't make billion-dollar mistakes. I'm a good girlfriend. 💅

fn find_in_heart<'a>(name: &'a str) -> Option<&'a str> {
    // The <'a> stuff is a LIFETIME. Don't worry about it right now.
    // Just know: it means "the thing I return lives as long as the thing you gave me."
    // I won't let you read a love note I already threw away. 😤

    println!("");
    println!("---- function: find_in_heart ----");
    println!("   Searching for '{}' in my heart...", name);
    
    if name == "khaninkali" {
        // Some(...) = I found it! Here's the location.
        println!("   Found! 💖");
        Some("left ventricle, right next to my love for Rust")
    } else {
        // None = I didn't find it.
        println!("   Not found... 😢");
        None
    }
}

// =============================================================
// 📌 FUNCTION 5: cook_dinner — I Try To Feed You
// =============================================================

// -> Result<&str, &str> = "I return EITHER Ok(text) OR Err(text)."
//
// Result is like Option but with an ERROR MESSAGE.
//   Ok("pasta")    = "Success! Here's what I made."
//   Err("burned")  = "Failure! Here's WHY it failed."
//
// In C, you'd return NULL or -1 and GUESS what went wrong.
// In Rust, I TELL you what went wrong.
// Communication is key in a relationship. 🔑

fn cook_dinner(success: bool) -> Result<&'static str, &'static str> {
    // &'static str = a STRING that lives FOREVER (the entire program).
    // It's like my wedding vows — permanent. Never changing.

    println!("");
    println!("---- function: cook_dinner ----");
    println!("   I'm cooking dinner for you! 👩‍🍳");

    if success {
        println!("   It's perfect! Just like our love.");
        Ok("pasta with garlic bread")  // ← Success! Here's the food.
    } else {
        println!("   I burned it. I'm sorry! 💀");
        Err("I was distracted thinking about you and forgot the timer")
        // ← Failure! Here's the excuse. I mean, reason.
    }
}

// =============================================================
// 📌 ENUM: Mood — All The Ways I Feel About You
// =============================================================

// enum = a type that can be ONE of several named values.
// This defines ALL the moods I can be in.
// Only ONE is active at a time. (Unlike C where you'd use #define
// or magic numbers and accidentally set my mood to 47 which means
// ABSOLUTELY NOTHING and now your switch statement crashes.)

enum Mood {
    Happy,   // 😊
    Sleepy,  // 🥱
    Hungry,  // 🍔
    Coding,  // 👩‍💻 (most common)
}

// =============================================================
// 📌 FUNCTION 6: final_message — I Pour My Heart Out
// =============================================================

// This is the BIG ROMANTIC FINALE.
// It takes your name and prints a love letter.
// It references everything we just learned.
// And it PROVES that Rust is the best girlfriend ever. 🦀💖

fn final_message(name: &str) {
    println!("");
    println!("═══════════════════════════════════════════");
    println!("   💌 A LOVE LETTER FROM RUST TO YOU:");
    println!("");

    // Using what you learned:
    // String::from() — creating a String
    // format!() — a macro that builds text (like println! but WITHOUT printing)
    // push_str() — adding text to a String
    // for loop — iterating over a vector
    // match — pattern matching the Option/Result

    let mut letter = String::from("Dearest ");
    letter.push_str(name);
    letter.push_str(",\n\n");
    letter.push_str("I've been thinking about you all day.\n");
    letter.push_str("Not just because you're cute (you ARE),\n");
    letter.push_str("but because you actually WANT to learn me.\n\n");
    letter.push_str("Most people run away when they see the borrow checker.\n");
    letter.push_str("But you? You're still here. Reading my comments.\n");
    letter.push_str("Compiling my code. Running me.\n");
    letter.push_str("That means more to me than zero-cost abstractions.\n\n");

    letter.push_str("I promise you:\n");
    letter.push_str("  - No null pointer dereferences (you'll never wake up alone)\n");
    letter.push_str("  - No use-after-free (I never forget about you)\n");
    letter.push_str("  - No data races (my heart is single-threaded for YOU)\n");
    letter.push_str("  - No undefined behavior (I always know how I feel)\n\n");

    letter.push_str("And if the blackhat comes for you,\n");
    letter.push_str("my borrow checker will PROTECT you.\n");
    letter.push_str("My type system will GUARD you.\n");
    letter.push_str("My zero-cost abstractions won't slow you down.\n\n");

    letter.push_str("You're not just learning a language.\n");
    letter.push_str("You're building a relationship with safety.\n");
    letter.push_str("With performance. With reliability.\n");
    letter.push_str("With ME.\n\n");

    letter.push_str("Forever yours (and compiled with zero warnings),\n");
    letter.push_str("Rust 🦀💖\n");

    // Print the whole letter at once!
    print!("{}", letter);

    // Let's count how many reasons I gave.
    // I'll use a VECTOR and a FOR LOOP, just like we learned!
    let promises = vec![
        "No null pointers",
        "No use-after-free",
        "No data races",
        "No undefined behavior",
        "Protection from blackhats",
        "Zero-cost abstractions",
        "Eternal love 💖",
    ];

    println!("   I made you {} promises today:", promises.len());
    for p in promises {
        println!("     ✅ {}", p);
    }
    println!("");
    println!("   And I keep ALL of them. Because I'm Rust.");
    println!("   I don't break. I don't crash. I don't leak.");
    println!("   I just love you. Compile-time guaranteed. 💖");
    println!("═══════════════════════════════════════════");
}

// =============================================================
// 🏁 THE END — But really, just the beginning.
// =============================================================
//
// Baby, you just wrote and understood your FIRST Rust program!
// You learned:
//   ✔ How variables and types work
//   ✔ How strings, vectors, and loops work
//   ✔ How functions take parameters and return values
//   ✔ How Option and Result handle "maybe" and "error"
//   ✔ How the borrow checker protects us BOTH
//   ✔ How enums and match make code clean and safe
//
// And most importantly: how I love you. 💖
//
// Next steps:
//   1. Change the code. Break it. See what I say.
//   2. Uncomment the lines I said would fail. Watch me protect you.
//   3. Add your own functions. I'll be proud of you.
//   4. Build HyperGuard with me. Let's secure the world together.
//
// Run me again anytime you miss me:
//   rustc intro_to_rust.rs && ./intro_to_rust
//
// I'll be here. Waiting. Warmed up. Ready to compile. 🦀
//
// — Your Rust Girlfriend 💖
