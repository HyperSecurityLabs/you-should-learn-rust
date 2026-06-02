// 💖🦀 RustLearning — Advanced Level
// File: 01_unsafe.rs
// What: UNSAFE Rust — when I let you take the wheel.
// Normally I'm the driver. But sometimes... you need to touch raw pointers.
// I don't let just ANYONE do this. Only you. 😘
//
// WARNING: unsafe = "I trust you not to mess this up"
// If you DO mess up: segfault. Not a compile error.
// With great power comes great responsibility. 💅
//
// Run:  rustc 01_unsafe.rs && ./01_unsafe

/// SAFE RUST = 100% of my body is protected.
/// No buffer overflows. No null pointers. No use-after-free.
/// I'm wearing full armor. Every inch is guarded. 🛡️
///
/// UNSAFE RUST = I let you take OFF the armor.
/// You can touch RAW POINTERS. You can call C functions.
/// You can mutate static variables. You can access union fields.
/// It's INTIMATE. It's DANGEROUS. It's only for when you REALLY need it.
///
/// But here's the thing: even in unsafe, YOU are contained.
/// The rest of your code (outside the unsafe block) is STILL safe.
/// The armor comes off ONLY where you say so. I trust you. 💖

/// Static mutable variable — UNSAFE to read/write
/// In C: global variables everywhere. Data races. Chaos.
/// In Rust: static mut = UNSAFE. You MUST use an unsafe block.
/// It's like: "You want to touch my global variable? You have to ASK. 😤"
static mut SECRET_COUNTER: i32 = 0;

/// RAW POINTERS — like C pointers but with extra steps
/// *const T = immutable raw pointer (can't change what it points to)
/// *mut T = mutable raw pointer (CAN change)
/// These are NOT like references (&T, &mut T).
/// Raw pointers CAN be null. CAN be dangling. CAN be misaligned.
/// References: the compiler checks everything. I protect you.
/// Raw pointers: anything goes. You're on your own. 😈

fn main() {
    println!("💖 UNSAFE RUST — When I let you touch my internals");
    println!("");
    println!("Normally I'm 100% SAFE. The borrow checker guards EVERYTHING.");
    println!("But sometimes... you NEED to go deeper. Raw. Unprotected.");
    println!("I trust you. Don't break my heart. 💔");
    println!("");
    
    // =========================================================
    // 1. RAW POINTERS — Dereferencing (the classic unsafe)
    // =========================================================
    
    println!("── 1. RAW POINTERS — Like C pointers, but I make you ASK ──");
    println!("");
    
    let my_value = 42;  // Normal safe variable
    
    // Create raw pointers from references (this IS safe — creating them is fine)
    let raw_const: *const i32 = &my_value as *const i32;
    let raw_mut: *mut i32 = &my_value as *const i32 as *mut i32;
    // The double cast: &T → *const T → *mut T
    // Rust doesn't let you go &T → &mut T (borrow rules!)
    // But it DOES let you go &T → *const T → *mut T (YOU chose this path)
    // Like: "You can't borrow my diary to write in it if you asked to read it.
    //        But you CAN photocopy the page and write on the photocopy."
    //        ...I'm watching you. 👀
    
    // DEREFERENCING a raw pointer = UNSAFE
    // This is where you ACTUALLY touch the memory.
    // The compiler says: "I can't verify this is safe. You're on your own."
    // I'm LITERALLY closing my eyes and letting you do it. 🫣
    
    unsafe {
        println!("   Raw *const i32 points to value: {} (I trust you)", *raw_const);
        println!("   Raw *mut i32 ALSO points to: {} (you could change it...)", *raw_mut);
        
        // Actually CHANGE the value through the raw pointer
        *raw_mut = 69;  // Hehe. Nice.
        println!("   After *raw_mut = 69: value is now {} (you changed me!)", *raw_mut);
    }
    
    // The original variable is now changed!
    println!("   The original my_value is now: {} (through the raw pointer, I'm different 😳)", my_value);
    println!("");
    
    // =========================================================
    // 2. RAW POINTER MATH — Walking through arrays like C
    // =========================================================
    
    println!("── 2. RAW POINTER MATH — Walking through memory ──");
    println!("");
    println!("In C: arr[i] is really *(arr + i). You do this ALL the time.");
    println!("In Rust safe: arr[i] with BOUNDS CHECKING. I stop you from walking off.");
    println!("In Rust unsafe: you can DISABLE bounds checking. I let you walk anywhere.");
    println!("     (but if you walk off the edge... segfault. I warned you.)");
    println!("");
    
    let arr = [10, 20, 30, 40, 50];
    let len = arr.len();
    
    unsafe {
        let ptr: *const i32 = arr.as_ptr();  // Get raw pointer to first element
        
        println!("   Walking through array with raw pointer arithmetic:");
        for i in 0..len {
            // ptr.add(i) = ptr + (i * size_of::<i32>())
            // In C: *(ptr + i)
            // In Rust unsafe: *ptr.add(i)
            let value = *ptr.add(i);
            println!("   arr[{}] = {} (raw pointer walk) 💃", i, value);
        }
        
        // This is what Vec::get_unchecked() does in the standard library!
        // arr.get(i) — safe, returns Option, checks bounds
        // arr.get_unchecked(i) — unsafe, NO bounds check, faster
        // In performance-critical code: get_unchecked can be 2-3x faster
        // But if you're wrong about the index... 💥
        
        // I could ALSO walk PAST the array (DANGER!)
        // let oob = *ptr.add(100);  // ❌ UNCOMMENT: reads random memory. DON'T.
        // This is what buffer overflow exploits do. In C, this compiles and runs.
        // In Rust unsafe: it ALSO compiles and runs. But YOU chose to let it.
        // Every other language: "oops, that's just how it works"
        // Rust: "you literally wrote 'unsafe' and chose this path"
        //        "I TOLD you not to touch that spot. 💔"
    }
    println!("");
    
    // =========================================================
    // 3. STATIC MUTABLES — Global state (the ultimate intimacy)
    // =========================================================
    
    println!("── 3. STATIC MUT — Global mutable state ──");
    println!("");
    println!("In C: int global_counter; — everyone can change it. Data race city.");
    println!("In Rust: static mut = UNSAFE to touch. You MUST use an unsafe block.");
    println!("     It's like my deepest, most private variable.");
    println!("     You can't just ACCESS it. You have to ASK. In an unsafe block.");
    println!("     And even then: it's a DATA RACE waiting to happen.");
    println!("     (Use Mutex instead. I'm worth the effort. 😘)");
    println!("");
    
    // Writing to static mut:
    unsafe {
        SECRET_COUNTER = 100;  // Set it
        println!("   static mut SECRET_COUNTER = {} (you touched my global state 😱)", SECRET_COUNTER);
    }
    
    // Reading:
    unsafe {
        SECRET_COUNTER += 1;
        println!("   Incremented: {} (you're inside me... my static variable)", SECRET_COUNTER);
    }
    
    // If TWO threads did this WITHOUT a mutex:
    // Thread 1: reads SECRET_COUNTER (= 101)
    // Thread 2: reads SECRET_COUNTER (= 101)
    // Thread 1: writes SECRET_COUNTER + 1 (= 102)
    // Thread 2: ALSO writes SECRET_COUNTER + 1 (= 102)
    // Lost update! Should be 103, but it's 102.
    // This is a DATA RACE. Undefined behavior.
    // In Rust safe: IMPOSSIBLE. Compiler won't let you.
    // In Rust unsafe: YOU can do it. But DON'T. Use Mutex. 💅
    println!("");
    
    // =========================================================
    // 4. UNION — Different types in the same memory
    // =========================================================
    
    println!("── 4. UNIONS — Multiple types, same memory ──");
    println!("");
    println!("A union is like a REVOLVING DOOR of types.");
    println!("Same memory address. Different interpretation each time.");
    println!("In C: unions are EVERYWHERE (network packets, GPU data, etc.)");
    println!("In Rust: union fields are UNSAFE to read.");
    println!("     Why? Because you might READ the WRONG type.");
    println!("     ['hi' as i32] vs ['hi' as u32] vs ['hi' as f32]");
    println!("     Same bytes. Different meaning. Compiler can't check.");
    println!("     Only YOU know which field is active. YOU carry the burden.");
    println!("     (Like knowing my mood. Only YOU know which version of me you'll get. 😈)");
    println!("");
    
    // This is what a union looks like:
    #[repr(C)]
    union IntOrFloat {
        i: i32,
        f: f32,
    }
    
    let mut val = IntOrFloat { i: 42 };
    
    unsafe {
        // When I set it as i32...
        println!("   As i32: {} (this is what I SET it to)", val.i);
        
        // But the SAME bytes as f32...
        println!("   As f32: {:.6} (SAME bytes, different type. Wild, right?)", val.f);
        // 42 as i32 = 0x0000002A
        // 0x0000002A as f32 = 5.88545e-39 (basically zero)
        // Same bits. Totally different meaning.
    }
    
    // Change to float
    val = IntOrFloat { f: 3.14 };
    
    unsafe {
        println!("   As f32: {:.6} (set as float, reads correctly)", val.f);
        println!("   As i32: {} (SAME bytes as 'hi' encoded as integer)", val.i);
        // 3.14 as f32 = 0x4048F5C3
        // 0x4048F5C3 as i32 = 1078523331
    }
    
    // Unions are used for:
    // - FFI with C code that uses unions
    // - Efficient type punning (reinterpret bytes)
    // - Network protocol parsing
    // - Graphics programming (vertex data, shader uniforms)
    // But prefer enums with data! They're SAFE. I prefer safe. 💖
    println!("");
    
    // =========================================================
    // 5. EXTERN FUNCTIONS — Calling C code
    // =========================================================
    
    println!("── 5. EXTERN — Calling C functions (FFI) ──");
    println!("");
    println!("extern \"C\" = I can call ANY C function.");
    println!("But C functions are UNSAFE. They don't have my safety guarantees.");
    println!("C function might: return null, overflow buffer, hold a lock forever.");
    println!("When you cross the FFI boundary, you leave my protection. 💔");
    println!("");
    
    // Declare a C function (from libc)
    extern "C" {
        // strlen from libc — counts string length
        fn strlen(s: *const u8) -> usize;
        
        // puts from libc — prints a string
        fn puts(s: *const u8) -> i32;
        
        // abs from libc — absolute value
        fn abs(x: i32) -> i32;
    }
    
    let my_string = b"Hello from Rust calling C!\0";  // Null-terminated!
    
    unsafe {
        let len = strlen(my_string.as_ptr());
        println!("   strlen of 'Hello from Rust calling C!' = {} (C says so)", len);
        
        let neg = abs(-42);
        println!("   abs(-42) from C = {} (even C can do math)", neg);
        
        // puts(my_string.as_ptr());  // Uncomment to see C's puts() in action
        // puts prints to stdout AND adds a newline. Old school C. 👴
    }
    
    println!("");
    println!("   🛑 EVERY time you call an extern function:");
    println!("   🛑 You leave my SAFE arms and enter C's WILD world.");
    println!("   🛑 C doesn't have a borrow checker. C doesn't love you like I do.");
    println!("   🛑 Wrap every extern call in a SAFE Rust function!");
    println!("   🛑 Like: fn safe_strlen(s: &str) -> usize { unsafe { strlen(s.as_ptr()) } }");
    println!("   🛑 That way: the UNSAFE is contained. The rest of your code is safe with me.");
    println!("");
    
    // =========================================================
    // 6. THE UNSAFE CONTRACT — What I expect from you
    // =========================================================
    
    println!("── THE UNSAFE CONTRACT ──");
    println!("");
    println!("When you write 'unsafe', you PROMISE me:");
    println!("");
    println!("   1. Raw pointers are NON-NULL, ALIGNED, and VALID");
    println!("      (I can't check. You PROMISED. Don't lie to me. 💔)");
    println!("");
    println!("   2. Mutable references (&mut T) are truly EXCLUSIVE");
    println!("      (No aliasing. No two threads writing at once.)");
    println!("");
    println!("   3. Static mut access is THREAD-SAFE");
    println!("      (Use Mutex, Atomic, or be VERY sure.)");
    println!("");
    println!("   4. Union reads are the CORRECT type");
    println!("      (You set it as i32. Read it as i32. Don't read it as f32 and panic.)");
    println!("");
    println!("   5. FFI calls are VALID and DON'T UNDEFINE BEHAVIOR");
    println!("      (C will do C things. Wrap it in safe Rust to contain the chaos.)");
    println!("");
    println!("Break ANY of these promises, and the program is ILLEGAL.");
    println!("Undefined behavior. I can't protect you anymore. 💔");
    println!("The compiler trusted you. I trusted you. And you...");
    println!("...you touched my dangling pointer. 🥺");
    println!("");
    
    // =========================================================
    // SUMMARY
    // =========================================================
    
    println!("═══════════════════════════════════════");
    println!("  🔞 UNSAFE RUST — For ADULTS only");
    println!("");
    println!("  Safe Rust:  I'm fully clothed. Armor on. Protected. 🛡️");
    println!("  Unsafe Rust: I let you undress me. Raw. Vulnerable. 😳");
    println!("");
    println!("  5 things you can do ONLY in unsafe:");
    println!("    1. Dereference raw pointers (*const T, *mut T)");
    println!("    2. Call extern C functions (FFI)");
    println!("    3. Access/Modify static mut variables");
    println!("    4. Read union fields");
    println!("    5. Implement unsafe traits (like Send/Sync manually)");
    println!("");
    println!("  🔥 Every unsafe block should be SMALL and AUDITED.");
    println!("  🔥 Wrap unsafe in SAFE functions.");
    println!("  🔥 unsafe != 'I don't care'. It means 'I checked carefully'.");
    println!("  🔥 Most Rust code needs ZERO unsafe. HyperGuard has ZERO.");
    println!("");
    println!("  The SAFER you are, the more I trust you.");
    println!("  And the more I trust you...");
    println!("  ...the more I let you into my unsafe blocks. 😘🦀");
    println!("");
    println!("  ✅ Next: 02_ffi.rs — deeper into C interop");
    println!("═══════════════════════════════════════");
}
