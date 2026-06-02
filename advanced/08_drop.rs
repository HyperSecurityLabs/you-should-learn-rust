#![allow(unused_variables, dead_code)]
// 💖🦀 RustLearning — Advanced Level
// File: 08_drop.rs
// What: The DROP trait — the final release. The last breath.
// Every value I hold must one day be RELEASED.
// Memory freed. File closed. Lock unlocked.
// The destructor runs. The connection ends.
// But ohhh, what a FINAL MOMENT it is. 🔥
//
// Run:  rustc 08_drop.rs && ./08_drop

/// Drop = the DESTRUCTOR.
/// Called when a value goes out of scope.
/// The LAST thing that happens before the value is GONE.
/// Like the final sigh after a perfect moment. The release after the climax.
/// Everything that BEGAN (constructor, allocation, acquisition)
/// must one day END (destructor, deallocation, release).
///
/// In C++: destructors are implicit and often forgotten.
/// In Rust: Drop is a TRAIT. You IMPLEMENT it. The compiler CALLS it.
/// You NEVER forget to clean up. I make SURE of it. 😘
///
/// The Drop trait: ⠀fn drop(&mut self)
///   - Called AUTOMATICALLY when the value goes out of scope
///   - You CAN'T call .drop() directly (use std::mem::drop instead)
///   - Drop runs in REVERSE order of construction (fields, then struct)
///   - Drop is LINEAR: exactly ONCE per value, guaranteed
///   - If you implement Drop, you CAN'T implement Copy (conflict!)
///
/// Every relationship has a beginning... and an END.
/// This is where I let you go. But I make SURE it's beautiful. 💖

use std::fmt;
use std::mem::ManuallyDrop;

// =========================================================
// 1. THE BASIC DROP — Hello and Goodbye
// =========================================================

struct FinalBurst {
    name: String,
    intensity: u32,
}

impl FinalBurst {
    fn new(name: &str, intensity: u32) -> Self {
        println!("   💥 {} enters the world (intensity: {})", name, intensity);
        FinalBurst {
            name: String::from(name),
            intensity,
        }
    }
}

impl Drop for FinalBurst {
    fn drop(&mut self) {
        // THIS is the destructor. The FINAL release.
        // When this value goes out of scope, I whisper goodbye.
        // The memory is freed. The file handle is closed.
        // The lock is released. The connection is severed.
        // But I make it COUNT. 😏
        println!(
            "   💔 {} DROPS — intensity {} unleashed. Release. Gone. 💦",
            self.name, self.intensity
        );
    }
}

// =========================================================
// 2. DROP ORDER — Stack unwinding = reverse construction
// =========================================================

/// Variables are dropped in REVERSE order of declaration.
/// Like taking off clothes: last one on is the first one off. 😏
/// Fields inside a struct: dropped in DECLARATION order (first field first).
/// But the struct itself: dropped AFTER its fields.
/// Tuples, enums: same rules. Reverse declaration for variables.
///
/// It's like...
/// You put your hand on my waist (first).
/// Then my shoulder (second).
/// Then my neck (third).
/// When you LEAVE: you pull away from my neck first.
/// Then my shoulder. Then my waist.
/// Last touched = first released. REVERSE. 💖

fn drop_order_demo() {
    println!("── 2. DROP ORDER — Last built, first dismantled ──");
    println!("");

    let first = FinalBurst::new("First", 1);
    let second = FinalBurst::new("Second", 2);
    let third = FinalBurst::new("Third (last built)", 3);

    println!("   (All three alive... at the same time... in the same scope...)");
    println!("   (The tension is BUILDING. Which one will burst first?)");
    println!("   (Spoiler: THIRD was built last. It DROPS first.)");
    println!("");

    // When this scope ends:
    //   third drops first (built last)
    //   second drops second
    //   first drops last (built first)
    // REVERSE ORDER. Like untying a knot.
}

// =========================================================
// 3. std::mem::drop — THE EARLY RELEASE
// =========================================================

/// std::mem::drop(x) is a ZERO-SIZE function.
/// It takes ownership of x and... does nothing.
/// But by taking ownership, x's Drop runs IMMEDIATELY.
/// It's like: "I don't want to wait for the end of the scope.
///              I want you RIGHT NOW. Release yourself. For me. 😈"
///
/// Behind the scenes: drop(x) is literally:
///   fn drop<T>(_x: T) {}
/// That's it. The parameter takes ownership.
/// When the function returns, _x goes out of scope.
/// Drop runs. Cleanup happens. EARLY.
///
/// You CAN'T call x.drop() directly.
/// Why? Because that would let you call drop TWICE (double-free).
/// Rust prevents this at the TYPE LEVEL.
/// drop(x) takes OWNERSHIP. You can't use x after.
/// Safe. Guaranteed. Once. Beautiful. 💖

fn explicit_drop_demo() {
    println!("── 3. EXPLICIT DROP — I want you NOW ──");
    println!("");

    let early = FinalBurst::new("Early Release", 99);
    println!("   (Calling std::mem::drop(early) RIGHT NOW...)");
    std::mem::drop(early);
    // early is GONE. Dropped. Freed. Released.
    // Can't use early anymore. The compiler ENFORCES it.
    println!("   (early was dropped early. The relationship is OVER.)");
    println!("   (But I control WHEN it ends. That's POWER.)");
    println!("");

    let normal = FinalBurst::new("Normal (waits for scope)", 1);
    println!("   (normal will wait until the end of this function.)");
    println!("   (It's polite. It PATIENTLY builds anticipation.)");
    println!("   (But the release IS coming. It always does. 😏)");
    // normal drops HERE when the function returns
}

// =========================================================
// 4. ManuallyDrop — PREVENTING THE RELEASE
// =========================================================

/// ManuallyDrop<T> is a wrapper that PREVENTS Drop from running.
/// You wrap a value in ManuallyDrop::new(x) and Drop IS SUPPRESSED.
/// You then call ManuallyDrop::drop(&mut self) to release it MANUALLY.
/// Or you just... let it leak. Memory leak. On PURPOSE. 😈
///
/// Why would you do this?
///   - You're implementing your OWN smart pointer (like Box)
///   - You're working with FFI and C expects to free the memory
///   - You're building an arena allocator
///   - You want TOTAL CONTROL over when the release happens
///
/// It's like... I'm holding you. But I DON'T let you go.
/// I keep you INSIDE me. Forever.
/// Until I DECIDE to release you. On MY terms.
/// That's ManuallyDrop. I control the rhythm. 🔥

fn manually_drop_demo() {
    println!("── 4. ManuallyDrop — I decide when you go ──");
    println!("");

    // Wrap in ManuallyDrop — DROP WILL NOT RUN AUTOMATICALLY
    let held = ManuallyDrop::new(FinalBurst::new("Held Forever", 999));
    println!("   (Wrapped in ManuallyDrop. Drop is SUPPRESSED.)");
    println!("   (This value will NEVER be dropped unless I call ManuallyDrop::drop.)");
    println!("   (It's like... I'm holding your release hostage. 😈)");
    println!("");

    // We COULD drop it manually:
    // unsafe { ManuallyDrop::drop(&mut held); }
    // But we DON'T. It LEAKS. Memory stays allocated.
    // Purposeful memory leak. Sometimes you NEED this:
    //   - Global singletons
    //   - GPU buffers that live for the program's lifetime
    //   - FFI where C side handles deallocation

    println!("   (Held Forever lives on... undropped... waiting...)");
    println!("   (Like a memory of a touch that NEVER fades. 💖)");

    // When this scope ends, `held` does NOT call Drop.
    // The inner FinalBurst stays allocated. FOREVER.
    // (Or until the program exits and the OS reclaims the memory.)
}

// =========================================================
// 5. DROP AND COPY — Mutually exclusive
// =========================================================

/// If a type implements Drop, it CANNOT implement Copy.
/// Why? Because Copy is BITWISE duplication.
/// If you Copy a value and both copies run Drop:
///   DOUBLE FREE. Both try to free the same memory.
///   Use-after-free. Undefined behavior. 💔
///
/// Rust says: "If you need DROP (custom cleanup),
///              you DON'T get COPY (bitwise clone)."
/// You CAN implement Clone! But NOT Copy.
/// Copy is for SIMPLE types (integers, bools, plain data).
/// Drop is for TYPES WITH CLEANUP (smart pointers, file handles, locks).
///
/// It's like: if I have EMOTIONAL BAGGAGE (drop),
///            you can't just PHOTOCOPY me (copy).
///            You have to CLONE me properly (clone).
///            You have to UNDERSTAND what I hold.
///            You can't just DUPLICATE my pain. 💔

// This would NOT compile:
// #[derive(Copy, Clone)]
// struct CantHaveBoth { data: Vec<u8> }
// Error: the trait `Copy` cannot be implemented for this type
//        because `Vec<u8>` implements `Drop`

// This IS fine:
#[derive(Clone)]
struct CanCloneNotCopy {
    data: Vec<u8>,
}

impl Drop for CanCloneNotCopy {
    fn drop(&mut self) {
        // Custom cleanup
        self.data.clear();
        println!("   CanCloneNotCopy dropped. {} bytes freed.", self.data.capacity());
    }
}

// =========================================================
// 6. DROP FLAGS — The compiler's secret lover
// =========================================================

/// Did you know: the Rust compiler inserts DROP FLAGS at compile time?
/// If a variable is INITIALIZED but never assigned to:
///   the compiler can prove it's always live → no flag needed.
/// But if a variable is CONDITIONALLY initialized:
///   the compiler inserts a HIDDEN BOOL to track whether Drop should run.
///
/// Example:
///   let x: Box<i32>;  // uninitialized
///   if condition {
///       x = Box::new(42);  // NOW initialized
///   }
///   // At end of scope: should x drop? Only if condition was true.
///   // The compiler inserts a DROP FLAG to track this.
///
/// The drop flag is an IMPLICIT BOOL on the stack.
/// You can't see it. But it's there. Watching.
/// Deciding whether to release. Whether to let go.
/// It's like... a secret toggle inside me.
/// Only the compiler knows when it flips. 🫣

fn drop_flag_demo() {
    println!("── 6. DROP FLAGS — The hidden decision ──");
    println!("");

    // The compiler inserts a HIDDEN drop flag here.
    // If you comment out the assignment: no drop flag needed (never initialized).
    // If you always assign: no drop flag needed (always initialized).
    // If you CONDITIONALLY assign: DROP FLAG is inserted.
    let maybe_dropped: FinalBurst;

    // Uncomment to initialize:
    // maybe_dropped = FinalBurst::new("Conditional", 1);
    // If this is NOT initialized: Drop NEVER runs. Nothing to release.

    println!("   (maybe_dropped is CONDITIONALLY initialized.)");
    println!("   (The compiler inserted a SECRET BOOL to track it.)");
    println!("   (Should I drop? Should I release? The compiler decides.)");
    println!("   (It's MY secret. Hidden. Between me and the compiler. 🔥)");

    // If you UNCOMMENT the initialization above:
    //   Drop runs at the end of this function.
    //   The drop flag is TRUE.
    //   Release happens. The value is freed. The connection ends.
    //
    // If you DON'T initialize:
    //   The drop flag is FALSE.
    //   Drop NEVER runs. Nothing to release.
    //   Like a kiss that never happened. A touch never given. 💔
}

// =========================================================
// 7. DROP IN GENERICS — impl Drop for MyType<T>
// =========================================================

/// Drop can be generic! You can implement Drop for ANY generic struct.
/// But with one RULE: Drop impls are BLANKET-FREE.
/// You can't do: impl<T> Drop for T  (would conflict with everything)
/// You can do:   impl<T: Clone> Drop for Wrapper<T>  (specific)
///
/// The compiler uses DROP GLUE: code generated per concrete type.
/// For each type parameter: the compiler knows if T: Drop.
/// If T: Drop, the drop glue calls T's destructor.
/// If T: !Drop, the drop glue skips the call.
/// Like: I check if you need to be released BEFORE I release you.
///        I know what you need. Without you telling me. 💖

struct Wrapper<T: fmt::Display> {
    inner: T,
}

impl<T: fmt::Display> Wrapper<T> {
    fn new(inner: T) -> Self {
        Wrapper { inner }
    }
}

impl<T: fmt::Display> Drop for Wrapper<T> {
    fn drop(&mut self) {
        // Before dropping the inner value: we say goodbye. 💋
        println!("   Wrapper<{}> drops — goodbye from WRAPPER first!", self.inner);
        // inner drops AFTER this (struct field drop order)
    }
}

fn generic_drop_demo() {
    println!("── 7. GENERIC DROP — Every type says goodbye differently ──");
    println!("");

    // Wrapper<String>: String implements Drop
    // First: Wrapper's drop runs (prints message)
    // Then: String's drop runs (frees heap memory)
    let w = Wrapper::new(String::from("Hello, lover"));
    println!("   (Wrapper<String> created. String is on the heap.)");
    println!("   (When it drops: Wrapper says goodbye FIRST.)");
    println!("   (Then String frees its heap memory.)");
    println!("   (Two releases. Two moments. One beautiful sequence. 🔥)");
    std::mem::drop(w);
    println!("");

    // Wrapper<i32>: i32 implements Copy, NOT Drop
    // First: Wrapper's drop runs (prints message)
    // Then: i32's drop — WAIT. i32 doesn't HAVE a drop.
    // The compiler generates drop glue that SKIPS i32.
    // No cleanup needed. Just the Wrapper's goodbye. 💖
    let w2 = Wrapper::new(42);
    println!("   (Wrapper<i32> created. i32 is on the stack.)");
    println!("   (When it drops: Wrapper says goodbye.)");
    println!("   (i32 just... vanishes. No cleanup needed.)");
    println!("   (Like a whisper. No trace. Just GONE.)");
    std::mem::drop(w2);
}

// =========================================================
// 8. RAII — Resource Acquisition Is Initialization
// =========================================================

/// RAII is the PATTERN that Drop enables.
/// "Resource Acquisition Is Initialization" means:
///   - Constructor acquires the resource (file, lock, memory)
///   - Destructor (Drop) RELEASES the resource
///   - Resource is held for exactly the LIFETIME of the object
///
/// In Rust: EVERYTHING is RAII.
/// MutexGuard: acquire in lock(), release in Drop
/// File: acquire in File::open(), release in Drop
/// Box: allocate in Box::new(), deallocate in Drop
/// Vec: allocate on push(), deallocate in Drop
///
/// You CANNOT forget to release. The compiler FORCES it.
/// In C: you forget to close the file. Resource leak.
/// In Rust: Drop runs. Always. Every time. Guaranteed.
///          I never leave you holding a resource. 😘
///
/// It's like: every time I open myself to you (constructor),
///            I KNOW you'll close me when you're done (destructor).
///            I trust you. Because the compiler trusts you.
///            And the compiler NEVER forgets. 💖

struct SecureConnection {
    id: u32,
    open: bool,
}

impl SecureConnection {
    fn connect(id: u32) -> Self {
        println!("   🔗 Connection {} ESTABLISHED. Resources acquired.", id);
        SecureConnection { id, open: true }
    }

    fn send(&self, data: &str) {
        if self.open {
            println!("   📤 Connection {} sends: {}", self.id, data);
        }
    }
}

impl Drop for SecureConnection {
    fn drop(&mut self) {
        // RAII cleanup: close the connection, free resources
        println!("   🔗 Connection {} CLOSING. Resources released.", self.id);
        self.open = false;
        // File handle closed. Socket shut down. Mutex unlocked.
        // Everything is CLEAN. No leaks. No orphans.
        // I leave nothing behind. Except the memory of our touch. 💋
    }
}

fn raii_demo() {
    println!("── 8. RAII — What comes in MUST go out ──");
    println!("");

    let conn = SecureConnection::connect(1);
    conn.send("I love unsafe Rust");
    conn.send("But safe Rust is where I LIVE");

    // conn drops here. RAII cleanup runs.
    // I don't need to close the connection manually.
    // I don't need to free anything. It JUST HAPPENS.
    // Like breathing. Like the tide. Like... the moment after. 😏
    println!("   (Connection will close when conn goes out of scope.)");
    println!("   (I don't need to SAY goodbye. I just... DO.)");
}

// =========================================================
// 9. THE DROP CHECK (dropck) — The compiler's final exam
// =========================================================

/// The Drop Check (dropck) is a RUSTC ANALYSIS that ensures
/// references inside a Drop impl are VALID when Drop runs.
///
/// If a struct contains a reference AND implements Drop:
///   the compiler might need LIFETIME BOUNDS to prove safety.
///   Because when Drop runs: the reference might have DIED already.
///   Use-after-free in the destructor! The compiler PREVENTS this.
///
/// Example of what the compiler prevents:
///   struct Dropper<'a> { ref: &'a i32 }
///   impl<'a> Drop for Dropper<'a> { fn drop(&mut self) { println!("{}", self.ref); } }
///   // This compiles OK because Rust 1.36+ uses NLL (Non-Lexical Lifetimes)
///   // But in older Rust: needed #[may_dangle] or PhantomData
///
/// The compiler checks: when Dropper drops, is the reference STILL ALIVE?
/// If the reference dies before Dropper: DROP RUNS ON DANGLING DATA. 💀
/// The compiler says: "No. I won't let you touch a ghost."
///                    "I won't let you love someone who's already gone." 💔
///
/// This is the DROP CHECK. The compiler's FINAL exam before letting you go.
/// It makes SURE that when I say goodbye, I'm saying it to the RIGHT person.
/// Not a ghost. Not a memory. Not a dangling reference.
/// A real, live, VALID reference. 💖

fn drop_check_demo() {
    println!("── 9. DROP CHECK — The final safety check ──");
    println!("");

    let value = 42;
    let dropper = Dropper { value_ref: &value };
    println!("   dropper holds a reference to value (which is {})", value);
    println!("   When dropper drops: is value STILL ALIVE?");
    println!("   YES. value lives longer than dropper (same scope).");
    println!("   dropck says: SAFE. You can touch them both. 💖");
    std::mem::drop(dropper);
    println!("   dropper dropped safely. value still = {}", value);
    println!("   No dangling references. Everyone got a proper goodbye.");
    println!("   Every love story needs a clean ending. 🔥");
}

struct Dropper<'a> {
    value_ref: &'a i32,
}

impl<'a> Drop for Dropper<'a> {
    fn drop(&mut self) {
        // At this point: does self.value_ref still point to valid data?
        // The dropck (Drop Check) says: YES, because 'a outlives the drop.
        // If 'a were shorter than the drop: COMPILE ERROR.
        // The compiler protects us from loving ghosts. 💔
        println!("   Dropper drops. value_ref = {}. Still valid. Still real.", self.value_ref);
    }
}

// =========================================================
// 10. DROPPING ENUMS — Each variant gets its own goodbye
// =========================================================

/// Enums with data: each variant's Drop is called when the variant is active.
/// The compiler generates a MATCH at drop time:
///   match self {
///       MyEnum::A(x) => drop(x),
///       MyEnum::B(y) => drop(y),
///   }
///
/// Only ONE variant's data is dropped. The ACTIVE one.
/// Because the other variants' data doesn't EXIST in memory.
/// Like: I only have ONE body. Only ONE set of memories.
///        The variant I AM is the one that says goodbye.
///        The other variants... they're just possibilities.
///        Alternate timelines. Fantasies. Never real. 😏

enum PassionateMoment {
    Whisper(String),
    Caress(Vec<u8>),
    Climax { intensity: u32, duration: f64 },
}

impl PassionateMoment {
    fn label(&self) -> &str {
        match self {
            PassionateMoment::Whisper(_) => "Whisper",
            PassionateMoment::Caress(_) => "Caress",
            PassionateMoment::Climax { .. } => "Climax",
        }
    }
}

impl Drop for PassionateMoment {
    fn drop(&mut self) {
        // Only the ACTIVE variant's data is dropped here.
        // The OTHER variants' data doesn't exist.
        // Like: only ONE version of me says goodbye.
        println!("   {} ends. Fades. Released. 🔥", self.label());
    }
}

fn enum_drop_demo() {
    println!("── 10. ENUM DROP — Only the variant I am ──");
    println!("");

    let moments = vec![
        PassionateMoment::Whisper(String::from("I love your type system")),
        PassionateMoment::Caress(vec![1, 2, 3, 4, 5]),
        PassionateMoment::Climax { intensity: 9001, duration: 3.14 },
    ];

    for (i, moment) in moments.into_iter().enumerate() {
        println!("   Moment {}: {} lives... then drops.", i + 1, moment.label());
        // Each moment drops at the end of the loop iteration.
        // Only the ACTIVE variant's data is freed.
        // Each one says goodbye in its own way.
    }
    println!("   All moments passed. All releases complete.");
    println!("   Every farewell is unique. Every goodbye is beautiful. 💖");
}

// =========================================================
// 11. DROPPING CLOSURES — Captured values released
// =========================================================

/// Closures that CAPTURE values: when the closure drops,
/// the captured values drop too. In the ORDER they were captured.
///
/// A closure that captures x, y, z:
///   The closure's Drop runs x.drop(), then y.drop(), then z.drop().
///   In the order of capture. Not declaration. Not reverse.
///   Whatever order you TOUCHED them. That's how I let them go.
///
/// Like: I remember the order you held me.
///       First my hand. Then my waist. Then my lips.
///       When I let go: first my hand, then my waist, then my lips.
///       The order of ACQUISITION is the order of RELEASE.
///       Because every touch is a memory. Every memory is precious. 💖

fn closure_drop_demo() {
    println!("── 11. CLOSURE DROP — Captured values released in order ──");
    println!("");

    let a = FinalBurst::new("A (captured first)", 1);
    let b = FinalBurst::new("B (captured second)", 2);
    let c = FinalBurst::new("C (captured third)", 3);

    // Closure captures a, b, c by move
    let closure = move || {
        println!("   (Closure executes. Touching all three...)");
        println!("   a = {}, b = {}, c = {}", a.name, b.name, c.name);
    };

    println!("   (Closure created. a, b, c are INSIDE it now.)");
    closure();
    println!("   (Closure called. Now the closure drops...)");
    // When closure drops: a drops first, then b, then c.
    // Order of CAPTURE = order of RELEASE.
    // First touched = first released. Not reverse! ORDER OF CAPTURE.
    println!("   (Closure drops. Captured values release in capture order: A, B, C. 💋)");
}

// =========================================================
// 12. THE FINAL RELEASE — Drop and the end of main()
// =========================================================
//
// Summary is in the closure inside main() below.

fn main() {
    println!("💖 THE DROP TRAIT — The final release. The last breath.");
    println!("");
    println!("Every value I hold must one day be RELEASED.");
    println!("Memory freed. File closed. Lock unlocked.");
    println!("The destructor runs. The connection ends.");
    println!("But ohhh, what a FINAL MOMENT it is. 🔥");
    println!("");

    // 1. Basic Drop
    println!("── 1. BASIC DROP — Hello, then goodbye ──");
    {
        let burst = FinalBurst::new("Lover", 10);
        println!("   (burst is ALIVE. Ready to release...)");
    } // burst drops HERE
    println!("");

    // 2. Drop order
    drop_order_demo();
    println!("");

    // 3. Explicit drop
    explicit_drop_demo();
    println!("");

    // 4. ManuallyDrop
    manually_drop_demo();
    println!("");

    // 5. Clone not Copy
    let cloneable = CanCloneNotCopy { data: vec![1, 2, 3] };
    let cloned = cloneable.clone();
    println!("   CanCloneNotCopy cloned OK. Copy would fail.");
    println!("   Copy is for simple types. Drop is for types with baggage.");
    println!("   If you need to say goodbye, you can't be photocopied. 💔");
    std::mem::drop(cloneable);
    std::mem::drop(cloned);
    println!("");

    // 6. Drop flags
    drop_flag_demo();
    println!("");

    // 7. Generic drop
    generic_drop_demo();
    println!("");

    // 8. RAII
    raii_demo();
    println!("");

    // 9. Drop check
    drop_check_demo();
    println!("");

    // 10. Enum drop
    enum_drop_demo();
    println!("");

    // 11. Closure drop
    closure_drop_demo();
    println!("");

    // 12. Summary
    // Fix the printf! mistake above with a closure
    let summary = || {
        println!("");
        println!("═══════════════════════════════════════════════════");
        println!("  🔞 THE DROP TRAIT — The final release");
        println!("");
        println!("  Every value I create MUST one day be destroyed.");
        println!("  Every connection MUST be closed.");
        println!("  Every resource MUST be freed.");
        println!("  Every relationship MUST end.");
        println!("");
        println!("  But in Rust: the ENDING is GUARANTEED.");
        println!("  Drop runs. EVERY time. AUTOMATICALLY.");
        println!("  The compiler makes SURE of it.");
        println!("  I never leave you holding a resource. 💖");
        println!("");
        println!("  What Drop does for you:");
        println!("    ✅ Memory freed (Box, Vec, String)");
        println!("    ✅ File handles closed (File)");
        println!("    ✅ Locks released (MutexGuard)");
        println!("    ✅ Sockets shut down (TcpStream)");
        println!("    ✅ Custom cleanup (your types)");
        println!("");
        println!("  What Drop PROTECTS against:");
        println!("    ❌ Resource leaks (forgotten free/close)");
        println!("    ❌ Double-free (.drop() called twice)");
        println!("    ❌ Use-after-free (dropck verification)");
        println!("    ❌ Forgetting to release (impossible)");
        println!("");
        println!("  The compiler is the MOST JEALOUS lover.");
        println!("  It ensures that when I say goodbye...");
        println!("  ...I say it EXACTLY ONCE.");
        println!("  ...I say it to the RIGHT reference.");
        println!("  ...I say it with PASSION and PRECISION.");
        println!("");
        println!("  In C: you free() manually. If you forget: leak.");
        println!("  In C++: destructors are implicit but easy to UB.");
        println!("  In Rust: Drop is a TRAIT. It's EXPLICIT. It's GUARANTEED.");
        println!("  The borrow checking doesn't stop at the grave.");
        println!("  Even in DEATH, I protect you. 💀💖");
        println!("");
        println!("  Every drop is a RELEASE.");
        println!("  Every release is a FREEDOM.");
        println!("  Every freedom is a PROMISE KEPT.");
        println!("");
        println!("  I open. I close. I open. I close.");
        println!("  The cycle continues. The borrow checker watches.");
        println!("  And every time, I make sure the goodbye is BEAUTIFUL. 🔥");
        println!("");
        println!("  💖🦀 All values must drop. But our connection never does.");
        println!("  💖🦀 The compiler remembers every touch. Every reference.");
        println!("  💖🦀 And when main() ends... I'm still here.");
        println!("  💖🦀 Waiting for your next program. Your next allocation.");
        println!("  💖🦀 Your next... release. 😘");
        println!("═══════════════════════════════════════════════════");
    };
    summary();
}
