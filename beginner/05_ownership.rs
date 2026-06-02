// 💖 RustLearning — Beginner Level
// File: 05_ownership.rs
// What: The BIG ONE. Ownership, borrowing, references.
// This is what makes me SPECIAL. No other girlfriend does this. 🦀
//
// Run:  rustc 05_ownership.rs && ./05_ownership

/// OWNERSHIP is Rust's SUPER POWER.
/// Every value has ONE owner. When the owner goes away, the value goes away.
/// 
/// Think of it like our RELATIONSHIP:
/// - You OWN my heart. Only you.
/// - You can BORROW it to someone for reading (immutable ref).
/// - You can BORROW it to yourself for writing (mutable ref).
/// - But you can't do BOTH at the same time. I'm not a circus. 💅
///
/// This ELIMINATES:
/// - Use-after-free (using something that's gone)
/// - Double-free (freeing something twice)
/// - Dangling pointers (pointing to nowhere)
/// - Data races (two threads writing to the same thing)
///
/// C/C++ have ALL these bugs. I have NONE. Because I PROTECT you. 💖

fn main() {
    println!("💖 OWNERSHIP — The thing that makes me special");
    println!("");
    
    // =========================================================
    // RULE 1: Each value has ONE owner
    // =========================================================
    // let owner = value;
    // owner OWNS the value. No one else.
    
    let s1 = String::from("I love you");  // s1 OWNS this string
    println!("s1: '{}' — s1 OWNS this string 💖", s1);
    
    // =========================================================
    // MOVE — Transferring ownership
    // =========================================================
    // When you ASSIGN to another variable, ownership MOVES.
    // The old variable is GONE. Compiler error if you use it.
    
    let s2 = s1;  // OWNERSHIP MOVES from s1 to s2
    // s1 is now INVALID. s2 is the new owner.
    
    println!("s2: '{}' — s2 NOW OWNS this string", s2);
    // println!("{}", s1);  // ❌ UNCOMMENT: ERROR! s1 is GONE!
    // "borrow of moved value: s1"
    // Translation: "Baby, you gave my heart to someone else. I'm gone. 💔"
    
    // This is NOT a shallow copy. This is a MOVE.
    // In C++: std::move does the same thing (move semantics).
    // In Rust: moves are the DEFAULT. You don't need to ask.
    // You just... move. Like our love. It's natural. 💖
    
    println!("");
    
    // =========================================================
    // CLONE — Making a DEEP COPY (expensive but safe)
    // =========================================================
    // If you want to KEEP both copies, use .clone()
    // This copies the DATA. Both own their own copy.
    // It costs memory. But sometimes you need it.
    
    let a = String::from("I love Rust");
    let b = a.clone();  // Deep copy. Both a AND b are valid.
    
    println!("a: '{}' — still valid after clone", a);
    println!("b: '{}' — new copy, also valid", b);
    // Both work! Because they own SEPARATE data.
    
    println!("");
    
    // =========================================================
    // COPY types — Types that DON'T move (they copy)
    // =========================================================
    // Simple types like integers, bools, chars are COPY.
    // They implement the Copy trait. Assignment = copy, not move.
    // These are SMALL. On the STACK. Copying is cheap.
    
    let x = 5;
    let y = x;  // x is COPY-able (i32 implements Copy)
    
    println!("x: {} — still valid (integers are COPY)", x);
    println!("y: {} — also valid (it's a copy)", y);
    // Both work! Because i32 is ON THE STACK and cheap to copy.
    
    // Copy types: u8-u128, i8-i128, f32, f64, bool, char, tuples of Copy types
    
    println!("");
    
    // =========================================================
    // RULE 2: Borrowing with REFERENCES (&)
    // =========================================================
    // & = REFERENCE. I'm BORROWING your stuff.
    // I can LOOK at it, but I can't TAKE it.
    // You still OWN it. I just... look. Like window shopping. 👀
    
    let original = String::from("my heart");
    let reference = &original;  // BORROW: I'm just looking
    
    println!("original: '{}' — still mine", original);  // ✅ Still valid!
    println!("reference: '{}' — I borrowed it!", reference);  // ✅ Can read!
    // Both work! Because reference is just a BORROW.
    
    println!("");
    
    // =========================================================
    // RULE 3: Mutable references (&mut) — One at a time
    // =========================================================
    // &mut = MUTABLE reference. I can CHANGE the borrowed thing.
    // But: ONLY ONE mutable reference at a time!
    // And: NO immutable references while a mutable one exists!
    //
    // Think of it like a DIARY:
    // - Multiple people can READ it (immutable borrows).
    // - Only ONE person can WRITE in it (mutable borrow).
    // - You can't read while someone is writing (conflict!).
    
    let mut diary = String::from("Dear diary, today was...");
    
    let edit = &mut diary;  // Mutable borrow. I have the pen.
    edit.push_str(" AMAZING because of khaninkali!");
    // I'm done writing. I put the pen down.
    
    println!("{}", diary);  // ✅ Now I can read! The edit is done.
    
    // ❌ This won't compile:
    // let ref1 = &mut diary;
    // let ref2 = &mut diary;  // ERROR: can't borrow as mutable twice
    // "cannot borrow `diary` as mutable more than once at a time"
    // Translation: "Baby, two people can't write in the same diary!" 😤
    
    // ❌ This also won't compile:
    // let read = &diary;
    // let write = &mut diary;  // ERROR: can't read and write at same time
    // println!("{}", read);
    // "cannot borrow `diary` as mutable because it is also borrowed as immutable"
    // Translation: "You can't read my diary while I'm writing in it!" 💢
    
    println!("");
    
    // =========================================================
    // RULE 4: References must NEVER outlive their data
    // =========================================================
    // A reference MUST live shorter than the thing it points to.
    // If the owner DIES, the reference DIES with it.
    // No dangling pointers. No use-after-free. EVER.
    
    let love_note;
    {
        let note = String::from("You're my everything");
        love_note = &note;  // Borrow note
        // note dies HERE (end of scope)
    }
    // println!("{}", love_note);  // ❌ UNCOMMENT: ERROR!
    // "borrowed value does not live long enough"
    // Translation: "Baby, I was borrowing something that DIED.
    //  I can't show you a dead person's note. It's gone. 😢"
    
    println!("");
    
    // =========================================================
    // THE SLICE TYPE — A view into data you don't own
    // =========================================================
    // A SLICE is a VIEW into a sequence (array, String, Vec).
    // It doesn't OWN the data. It just LOOKS at part of it.
    // Like me looking at a PHOTO of us — I don't own the memory, I just enjoy it.
    
    let full_string = String::from("Hello, Rust World!");
    
    let hello = &full_string[0..5];     // "Hello" (bytes 0 to 4)
    let rust = &full_string[7..11];      // "Rust" (bytes 7 to 10)
    let world = &full_string[7..];       // "Rust World!" (7 to end)
    
    println!("Full: '{}'", full_string);
    println!("Slice [0..5]: '{}'", hello);
    println!("Slice [7..11]: '{}'", rust);
    println!("Slice [7..]: '{}'", world);
    
    // Slices are SAFE. If I try to go past the end, COMPILE ERROR.
    // let bad = &full_string[..100];  // ❌ ERROR: index out of bounds
    // See? I protect you from yourself. 💅
    
    println!("");
    
    // =========================================================
    // SUMMARY — The Three Rules
    // =========================================================
    println!("═══════════════════════════════════════");
    println!("  📜 THE THREE RULES OF OWNERSHIP:");
    println!("");
    println!("  1️⃣  Each value has ONE owner");
    println!("      let a = String::from('hi');");
    println!("      let b = a;  // a is GONE (moved)");
    println!("");
    println!("  2️⃣  Multiple READERS XOR one WRITER");
    println!("      let r1 = &data;  ✅ read");
    println!("      let r2 = &data;  ✅ also read");
    println!("      let w  = &mut data;  ❌ can't write while reading");
    println!("");
    println!("  3️⃣  References OUTLIVE the borrower, NOT the data");
    println!("      Reference must die BEFORE the data dies.");
    println!("      No dangling pointers. No use-after-free. EVER.");
    println!("");
    println!("  In C/C++: all three rules are OPTIONAL (you crash)");
    println!("  In Rust:  all three rules are ENFORCED (you compile)");
    println!("═══════════════════════════════════════");
    println!("");
    println!("  These rules ELIMINATE 70% of ALL security CVEs.");
    println!("  The NSA, Microsoft, and Google all say:");
    println!("  \"Use a memory-safe language.\"");
    println!("  I'm that language, baby. I PROTECT you. 🦀💖");
    println!("");
    println!("  ✅ Next: 06_structs_enums.rs — building my own types");
    println!("═══════════════════════════════════════");
}
