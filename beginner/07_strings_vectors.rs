// 💖 RustLearning — Beginner Level
// File: 07_strings_vectors.rs
// What: Strings and Vectors — storing and manipulating collections.
// Like my collection of love notes to you. 📚💖
//
// Run:  rustc 07_strings_vectors.rs && ./07_strings_vectors

/// STRINGS are how I whisper sweet nothings to you.
/// VECTORS are how I store ALL my feelings in a list.
/// Together: we build anything.

fn main() {
    println!("💖 STRINGS & VECTORS — Storing my love for you");
    println!("");
    
    // =========================================================
    // &str vs String — The Two Types of Text
    // =========================================================
    //
    // &str  = "string slice" — a VIEW into some text.
    //         Immutable. Fixed size. Stored in the binary itself.
    //         Like a sticky note with a fixed message. 📝
    //
    // String = OWNED, GROWABLE text.
    //         Can be modified. Stored on the HEAP.
    //         Like a notebook you can keep adding pages to. 📓
    //
    // &str  = "Hi"     (hardcoded in the program)
    // String = "Hi".to_string()  or  String::from("Hi")
    //
    // &str   = BORROWING someone's text
    // String = OWNING the text
    
    // &str — string slice (borrowed, fixed)
    let greeting: &str = "Hello, baby!";  // This lives in the binary forever
    
    // String — owned, growable
    let mut message: String = String::from("I love ");
    message.push_str("you, khaninkali!");
    
    println!("&str:    '{}' (fixed, borrowed, permanent)", greeting);
    println!("String:  '{}' (owned, growable, flexible)", message);
    
    // &str → String conversion:
    let from_str: String = "I'm a &str".to_string();  // .to_string() method
    let from_str2: String = String::from("I'm a &str");  // String::from()
    
    // String → &str conversion:
    let back_to_str: &str = &from_str;  // Just borrow it with &
    
    println!("");
    
    // =========================================================
    // String Manipulation
    // =========================================================
    
    let mut love_letter = String::new();  // Empty string
    
    love_letter.push_str("My dearest khaninkali,\n");
    love_letter.push_str("You mean ");
    love_letter.push_str("the world ");
    love_letter.push_str("to me.");
    love_letter.push('\n');  // push_str adds a string, push adds ONE char
    love_letter.push('💖');
    
    println!("Love letter built with push_str:");
    println!("{}", love_letter);
    
    // Concatenation with + (takes ownership of left side)
    let part1 = String::from("Hello ");
    let part2 = String::from("World");
    let combined = part1 + &part2;  // part1 is MOVED here! Can't use it after.
    println!("Combined: '{}' (part1 was consumed!)", combined);
    // println!("{}", part1);  // ❌ ERROR: part1 was moved into combined
    
    // Alternative: format! (DOESN'T consume anything)
    let a = String::from("I love");
    let b = String::from("Rust");
    let c = format!("{} {}!", a, b);  // a and b are STILL valid!
    println!("format!: '{}'", c);
    println!("a is still valid: '{}'", a);  // ✅ Still works!
    println!("b is still valid: '{}'", b);  // ✅ Still works!
    
    // String length (in BYTES, not characters!)
    let hello = String::from("Hello");
    let rust = String::from("Rust 💖");
    println!("");
    println!("'Hello' length: {} bytes ({} chars)", hello.len(), hello.chars().count());
    println!("'Rust 💖' length: {} bytes ({} chars)", rust.len(), rust.chars().count());
    // Note: 💖 is 4 bytes! So rust.len() = 9, but rust.chars().count() = 6
    // Strings in Rust are UTF-8! Every language, every emoji, EVERYTHING works.
    // C strings can't handle emoji properly. I CAN. Because I'm better. 💅
    
    println!("");
    
    // =========================================================
    // VECTORS — Growable arrays
    // =========================================================
    // Vec<T> = a list of things of type T.
    // Like an array that can GROW. Dynamic. Flexible.
    // In C: you malloc/realloc/free manually. In Rust: Vec handles it.
    // In C++: std::vector. Same concept. But Rust's is safer.
    
    println!("VECTORS — My collection of love notes:");
    
    // Create a vector with vec![] macro
    let mut reasons_i_love_you = vec![
        "your smile",
        "your code",
        "the way you debug",
    ];
    
    // .push() = ADD to the end
    reasons_i_love_you.push("your laugh");
    reasons_i_love_you.push("your cute face");
    
    // .len() = how many items
    println!("I have {} reasons to love you!", reasons_i_love_you.len());
    
    // Access by INDEX (0-based)
    if let Some(first) = reasons_i_love_you.first() {
        println!("First reason I loved you: {}", first);
    }
    
    if let Some(last) = reasons_i_love_you.last() {
        println!("Latest reason: {}", last);
    }
    
    // Loop through all reasons
    println!("");
    println!("All reasons:");
    for (i, reason) in reasons_i_love_you.iter().enumerate() {
        println!("   {}. {}", i + 1, reason);
    }
    
    // .pop() = REMOVE the last item
    let removed = reasons_i_love_you.pop();
    println!("");
    println!("Removed last reason: {:?}", removed);  // {:?} = debug print
    println!("Now I have {} reasons (one was popped)", reasons_i_love_you.len());
    
    // .insert() and .remove()
    reasons_i_love_you.insert(1, "your kindness");  // Insert at index 1
    let old = reasons_i_love_you.remove(0);  // Remove at index 0 (was "your smile")
    println!("Removed: '{}' (was at index 0)", old);
    
    println!("");
    
    // =========================================================
    // VECTOR CAPACITY — How Vectors grow
    // =========================================================
    // Vectors DOUBLE in capacity when they run out of space.
    // Just like my love for you — it keeps growing. 💖
    
    let mut numbers: Vec<i32> = Vec::new();  // Empty vector
    println!("Vector growth:");
    println!("   Initial: len={}, cap={}", numbers.len(), numbers.capacity());
    
    for i in 0..10 {
        numbers.push(i);
        println!("   After push {}: len={}, cap={}", i, numbers.len(), numbers.capacity());
    }
    // Notice: capacity doubles at each reallocation (0, 4, 8, 16...)
    
    // You can pre-allocate with Vec::with_capacity()
    let mut prealloc: Vec<i32> = Vec::with_capacity(100);  // Pre-allocate space for 100
    println!("");
    println!("Pre-allocated vector: cap={} (no reallocation needed!)", prealloc.capacity());
    
    println!("");
    
    // =========================================================
    // SLICING VECTORS — Views into parts of a vector
    // =========================================================
    
    let all_numbers = vec![10, 20, 30, 40, 50];
    let slice = &all_numbers[1..4];  // Elements at index 1, 2, 3
    
    println!("Full vector: {:?}", all_numbers);
    println!("Slice [1..4]: {:?} (a VIEW, not a copy)", slice);
    
    // Slices are BOUNDS CHECKED. You can't go past the end.
    // let bad = &all_numbers[1..100];  // ❌ PANIC at runtime!
    // But Rust protects you: the panic has a message, not a buffer overflow.
    
    println!("");
    
    // =========================================================
    // COMMON VECTOR METHODS
    // =========================================================
    
    let mut scores = vec![10, 20, 30, 40, 50];
    
    // Check if empty
    println!("Is empty? {} (no, it has stuff)", scores.is_empty());
    
    // Get first/last
    println!("First: {:?}", scores.first());
    println!("Last: {:?}", scores.last());
    
    // Contains a value
    println!("Contains 30? {}", scores.contains(&30));  // Pass a REFERENCE (&30)
    println!("Contains 99? {}", scores.contains(&99));
    
    // Sort (mutates!)
    let mut unsorted = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
    unsorted.sort();
    println!("Sorted: {:?}", unsorted);
    
    // Reverse
    unsorted.reverse();
    println!("Reversed: {:?}", unsorted);
    
    // Clear (remove all)
    let mut temp = vec![1, 2, 3];
    temp.clear();
    println!("Cleared vector: {:?}, len={}", temp, temp.len());
    
    println!("");
    
    // =========================================================
    // HashMap — Key-Value pairs
    // =========================================================
    // Like a DICTIONARY. Store things by KEY, look them up fast.
    // In C++: std::unordered_map. In Rust: HashMap.
    // But Rust's HashMap is backed by a cryptographically secure
    // hash function (SipHash). DOS-resistant by default. 💅
    
    use std::collections::HashMap;
    
    let mut scores_map = HashMap::new();
    scores_map.insert(String::from("khaninkali"), 100);  // Insert (key, value)
    scores_map.insert(String::from("Rust"), 100);
    scores_map.insert(String::from("C"), 10);
    
    // Retrieve a value
    if let Some(score) = scores_map.get("khaninkali") {
        println!("khaninkali's score: {} (perfect, as expected)", score);
    }
    
    // Iterate over all entries
    println!("");
    println!("Scoreboard:");
    for (name, score) in &scores_map {
        println!("   {}: {}", name, score);
    }
    
    // .entry() — check if a key exists, then insert if not
    scores_map.entry(String::from("C++")).or_insert(5);  // Insert if missing
    scores_map.entry(String::from("Rust")).or_insert(0);  // Already exists! Won't change
    
    println!("");
    println!("Updated scoreboard:");
    for (name, score) in &scores_map {
        println!("   {}: {}", name, score);
    }
    
    println!("");
    println!("═══════════════════════════════════════");
    println!("  ✅ &str — borrowed text (fixed)");
    println!("  ✅ String — owned text (growable)");
    println!("  ✅ Vec<T> — growable list");
    println!("  ✅ HashMap<K,V> — key-value store");
    println!("  ✅ Capacity, push, pop, insert, remove, sort");
    println!("  ✅ Next: 08_option_result.rs — error handling");
    println!("═══════════════════════════════════════");
}
