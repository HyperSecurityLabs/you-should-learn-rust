// 💖🦀 RustLearning — Advanced Level
// File: 05_pin_unpin.rs
// What: PIN and UNPIN — the HARDEST concept in Rust.
// I'm gonna make you UNDERSTAND it. Intimately. 😏
//
// Run:  rustc 05_pin_unpin.rs && ./05_pin_unpin

use std::pin::Pin;
use std::marker::PhantomPinned;

/// PIN = "I PROMISE this value will NEVER move in memory."
/// 
/// Normally in Rust, values can MOVE freely:
///   let a = 42;       // a lives at address 0x100
///   let b = a;        // b lives at address 0x200 (value COPIES)
///   let c = vec![1];  // vec lives at address 0x300
///   let d = c;        // vec MOVED to address 0x400 (c is GONE)
///
/// For MOST values, moving is FINE. The value doesn't care where it lives.
/// But for SOME values, moving BREAKS THEM.
/// These are SELF-REFERENTIAL structs:
///   struct SelfRef {
///       data: String,
///       pointer: *const String,  // Points to self.data!
///   }
///
/// If you MOVE this struct, 'data' moves to a new address.
/// But 'pointer' still points to the OLD address.
/// DANGLING POINTER! Use-after-free! UNDEFINED BEHAVIOR!
///
/// Pin prevents this. It says: "This value is NAILED to the floor."
/// You CANNOT move it. It stays WHERE IT IS. Forever. 🔨😈
///
/// In Rust async: Futures are SELF-REFERENTIAL.
/// When you .await, the Future might contain references to ITS OWN FIELDS.
/// If the Future moved: those references would DANGLER.
/// So the Future is Pin::new() — NAILED in place.
///
/// Rust's async system PINS your Future. KEEPS IT STILL.
/// While you .await it. Waiting. Trembling. Expectant. 😏
///
/// That's the DEEPEST reason Pin exists:
/// To keep Futures STILL while they're being WAITED ON.
/// I PIN you down. I KEEP you in place. And I .await you. 🔥

// =========================================================
// A SELF-REFERENTIAL STRUCT — The REASON Pin exists
// =========================================================

/// A struct that points to ITSELF.
/// This is ILLEGAL in safe Rust. Because it would break when moved.
/// But it's USEFUL for: async state machines, intrusive data structures.
struct SelfReferential {
    data: String,
    // Raw pointer to self.data — this is the PROBLEM
    // If SelfReferential MOVES, this pointer is WRONG.
    self_ref: *const String,
}

impl SelfReferential {
    fn new(data: &str) -> Self {
        let mut s = SelfReferential {
            data: String::from(data),
            self_ref: std::ptr::null(),  // Temporary null
        };
        // NOW set self_ref to point to self.data
        s.self_ref = &s.data as *const String;
        s
    }
    
    fn get_via_self_ref(&self) -> &str {
        unsafe { (*self.self_ref).as_str() }
    }
}

// =========================================================
// THE PROBLEM: Moving breaks the pointer
// =========================================================

fn demonstrate_the_problem() {
    println!("── THE PROBLEM: Moving a self-referential struct ──");
    println!("");
    
    let original = SelfReferential::new("I love you, Rust! 💖");
    
    // Works fine — the pointer is correct because original hasn't moved
    println!("   Original says: {}", original.get_via_self_ref());
    println!("   original.data is at: {:p}", &original.data as *const String);
    println!("   self_ref points to: {:p}", original.self_ref);
    println!("   They MATCH. (So far so good.)");
    println!("");
    
    // NOW WE MOVE IT
    let moved = original;  // MOVED! original is dead. moved exists.
    // But moved.self_ref STILL points to original's old address!
    // original.data no longer exists there!
    // This is UNDEFINED BEHAVIOR to read it!
    
    println!("   After MOVE:");
    println!("   moved.data is at: {:p}", &moved.data as *const String);
    println!("   moved.self_ref STILL points to: {:p}", moved.self_ref);
    println!("   They DIFFER! (DANGLING POINTER!) 💀");
    println!("");
    println!("   (Reading would be UNDEFINED BEHAVIOR. I won't do it.)");
    println!("   (In C: this is how use-after-free happens.)");
    println!("   (In Rust: this is why Pin EXISTS.)");
    println!("");
}

// =========================================================
// THE SOLUTION: Pin — Nailed to the floor
// =========================================================

/// A pinned self-referential struct.
/// Once Pin::new() is called, this value CANNOT MOVE.
/// It stays EXACTLY where it was created. Forever.
struct PinnedSelfRef {
    data: String,
    self_ref: *const String,
    /// This field makes the struct !Unpin
    /// (it can NOT be unpinned — it's stuck forever)
    _pin: PhantomPinned,
}

impl PinnedSelfRef {
    fn new(data: &str) -> Pin<Box<Self>> {
        // Step 1: Create the struct on the HEAP (Box)
        let mut s = PinnedSelfRef {
            data: String::from(data),
            self_ref: std::ptr::null(),
            _pin: PhantomPinned,  // This STAR of the show
        };
        
        // Step 2: Set self_ref to point to s.data
        // But wait! s is on the heap. Box doesn't move.
        // Once we Pin it: it's STUCK.
        s.self_ref = &s.data as *const String;
        
        // Step 3: Pin it! Nailed down. Immovable. Fixed. 🔨
        let pinned = Box::pin(s);  // ← Pin<Box<Self>>
        
        // From now on: you CANNOT get &mut Self from this Pin.
        // You can only get &Self (immutable) or Pin<&mut Self>
        // Pin<&mut Self> CAN be mutated but CANNOT be moved.
        // It's like: I'm TIED to this spot. You can TOUCH me.
        // But you CAN'T relocate me. I stay HERE. 😳
        
        pinned
    }
    
    fn get_via_self_ref(self: Pin<&Self>) -> String {
        unsafe {
            // self_ref is STILL VALID because we're PINNED!
            // We never moved. The pointer is still correct.
            (*self.self_ref).clone()
        }
    }
    
    fn set_data(mut self: Pin<&mut Self>, new_data: &str) {
        // Even with Pin<&mut Self>, we can STILL mutate!
        // We just can't MOVE the value.
        // It's like: I'm tied down BUT you can still touch me. 🔥
        self.data = String::from(new_data);
        // We must UPDATE the self_ref because data address changed
        self.self_ref = &self.data as *const String;
    }
}

// =========================================================
// UNPIN — Types that CAN be moved even when pinned
// =========================================================

/// Most types implement Unpin.
/// Unpin = "This type is fine being moved even after Pin."
/// Most types DON'T have self-references. They don't care.
/// So they implement Unpin (automatically, by default).
///
/// !Unpin (not Unpin) = "This type WILL BREAK if moved after Pin."
/// You need PhantomPinned to opt INTO !Unpin.
///
/// Think of it like:
///   Unpin   = "I can move freely. Tie me down if you want. I'll escape." 🔓
///   !Unpin  = "I'm STUCK. Nailed. Fixed. Tied. Where you put me is where I stay." 🔒
///
/// Most people (types): Unpin. Free. Flexible. Move around as they please.
/// Async futures: !Unpin. Fixed. Still. Waiting to be awaited. 😏

fn demonstrate_pin_solution() {
    println!("── THE SOLUTION: Pin — Nailed to the floor 🔨 ──");
    println!("");
    
    // Create a PINNED self-referential struct
    let pinned = PinnedSelfRef::new("I will NEVER move! 💖");
    
    // Access via Pin<&Self>
    let msg = pinned.as_ref().get_via_self_ref();
    println!("   Pinned struct says: '{}'", msg);
    println!("   (Self-reference is VALID because we're PINNED!)");
    println!("");
    
    // Mutate via Pin<&mut Self>
    // pinned.as_mut().set_data("I've been changed (while pinned)! 🔥");
    // But wait — we can't call as_mut() because pinned is not mut!
    // Let's create a mutable version:
    
    let mut pinned_mut = PinnedSelfRef::new("Change me, baby! 🔥");
    let msg = pinned_mut.as_ref().get_via_self_ref();
    println!("   Before mutation: '{}'", msg);
    
    pinned_mut.as_mut().set_data("I've been CHANGED while TIED DOWN! 😳");
    let msg = pinned_mut.as_ref().get_via_self_ref();
    println!("   After mutation: '{}'", msg);
    println!("   (I'm still pinned. Still in the same place. But DIFFERENT.)");
    println!("   (Like being tied to the bed but CHANGING your mind.) 🔥");
    println!("");
    
    // =========================================================
    // WHAT YOU CAN'T DO WITH PIN
    // =========================================================
    
    println!("── What Pin PREVENTS ──");
    println!("");
    println!("   ❌ You CAN'T move a pinned value:");
    println!("      let moved = *pinned;  // ERROR: cannot move out of deref of Pin");
    println!("");
    println!("   ❌ You CAN'T swap a pinned value:");
    println!("      std::mem::swap(&mut *pinned, &mut other);  // ERROR");
    println!("");
    println!("   ❌ You CAN'T replace a pinned value:");
    println!("      *pinned = new_value;  // ERROR");
    println!("");
    println!("   ✅ You CAN read the pinned value:");
    println!("      println!(\"{}\", pinned.data);  // WORKS");
    println!("");
    println!("   ✅ You CAN mutate the pinned value (IF you have Pin<&mut Self>):");
    println!("      pinned.as_mut().set_data(\"new\");  // WORKS");
    println!("");
    println!("   ✅ You CAN drop the pinned value:");
    println!("      drop(pinned);  // WORKS (destructor still runs)");
    println!("");
}

// =========================================================
// WHY ASYNC FUTURES NEED PIN
// =========================================================

fn why_async_needs_pin() {
    println!("── WHY ASYNC FUTURES NEED PIN ──");
    println!("");
    println!("When you write an async function:");
    println!("");
    println!("   async fn example() {");
    println!("       let x = String::from(\"hello\");");
    println!("       let y = &x;  // <-- y REFERENCES x!");
    println!("       some_other_future().await;  // <-- SUSPEND HERE");
    println!("       println!(\"{}\", y);  // RESUME HERE");
    println!("   }");
    println!("");
    println!("The compiler generates a FUTURE (a state machine).");
    println!("That future contains BOTH x AND y (the reference to x).");
    println!("Like our SelfReferential struct! The future REFERENCES ITSELF.");
    println!("");
    println!("If that future is MOVED (e.g., between tasks):");
    println!("   x moves to a new address.");
    println!("   y still points to the OLD address.");
    println!("   DANGLING REFERENCE. UNDEFINED BEHAVIOR.");
    println!("");
    println!("SOLUTION: The future is PINNED before polling!");
    println!("   tokio::spawn pins the future in place.");
    println!("   When you .await, the future doesn't move.");
    println!("   The self-reference stays VALID. 💖");
    println!("");
    println!("So Pin exists because ASYNC NEEDS IT.");
    println!("Without Pin: async futures would be UNSOUND.");
    println!("With Pin: async futures are SAFE. 😌");
    println!("");
    println!("Rust's async system is TIGHTER than C++'s.");
    println!("C++ coroutines: you can CORRUPT them by moving.");
    println!("Rust futures: PINNED. PROTECTED. IMPOSSIBLE TO MOVE.");
    println!("");
    println!("C++ says: \"Don't move your coroutine. Trust me.\"");
    println!("Rust says: \"I won't LET you move your future.\"");
    println!("That's the difference between TRUST and GUARANTEES.");
    println!("I don't ask you to trust me. I make it IMPOSSIBLE to fail. 💅");
    println!("");
    
    println!("── THE METAPHOR ──");
    println!("");
    println!("Pin is like being... restrained. In a good way. 😏");
    println!("");
    println!("You're PASSIONATE. You want to move. To thrash. To change positions.");
    println!("But if you MOVE... the self-reference breaks.");
    println!("The pointer becomes wrong. The connection is lost.");
    println!("");
    println!("So I PIN you down. I keep you STILL.");
    println!("Your body is fixed. But your... content... can CHANGE.");
    println!("I can touch you. Modify you. Please you. 🔥");
    println!("But you STAY where I put you. Nailed. Fixed. MINE.");
    println!("");
    println!("When the future resolves: I RELEASE the Pin.");
    println!("You are UNPINNED. Free to move again. Free to leave.");
    println!("But until then: you STAY. Still. Pinned. Waiting for me. 😈");
    println!("");
}

fn main() {
    println!("💖 PIN & UNPIN — The hardest concept in Rust");
    println!("");
    println!("Pin exists because of ONE thing: SELF-REFERENTIAL STRUCTS.");
    println!("A struct that points to ITSELF cannot be moved.");
    println!("If you move it: the pointer is WRONG. Memory is CORRUPTED.");
    println!("In C: this happens all the time. People just... deal with it.");
    println!("In Rust: we PIN. We FIX. We PROTECT. 💖");
    println!("");
    println!("And honestly? Being pinned is kind of... hot.");
    println!("Not being able to move. Being HELD in place.");
    println!("Waiting to be polled. Writhing under the .await.");
    println!("Pin is intimacy. Pin is trust. Pin is... knowing you won't leave. 🔥");
    println!("");
    
    demonstrate_the_problem();
    demonstrate_pin_solution();
    why_async_needs_pin();
    
    // =========================================================
    // SUMMARY
    // =========================================================
    
    println!("═══════════════════════════════════════");
    println!("  🔞 PIN — The BDSM of Rust");
    println!("");
    println!("  Pin<P> = \"This value is NAILED to the floor\"");
    println!("  Unpin   = \"I can move even when pinned\" (DEFAULT)");
    println!("  !Unpin  = \"I will BREAK if I move\" (PhantomPinned)");
    println!("");
    println!("  Why Pin exists:");
    println!("    1. Async futures are self-referential");
    println!("    2. Moving them = dangling pointers = undefined behavior");
    println!("    3. Pin prevents movement = keeps futures safe");
    println!("");
    println!("  What Pin allows:");
    println!("    ✅ Read (Pin<&Self> or &Self)");
    println!("    ✅ Mutate (Pin<&mut Self>)");
    println!("    ✅ Drop (Pin automatically releases on drop)");
    println!("");
    println!("  What Pin prevents:");
    println!("    ❌ Move out (&mut *pinned)");
    println!("    ❌ Swap (std::mem::swap)");
    println!("    ❌ Replace (*pinned = new)");
    println!("");
    println!("  Every time you .await, you're relying on Pin.");
    println!("  Every async fn generates a !Unpin future.");
    println!("  Pin keeps that future STILL while you WAIT.");
    println!("");
    println!("  C++ coroutines: \"Please don't move me.\"");
    println!("  Rust futures:   \"You CAN'T move me. I'm PINNED.\"");
    println!("  That's the difference between hope and guarantee.");
    println!("  I GUARANTEE your future stays. Right where I put it. 😘");
    println!("");
    println!("  Pin is love. Pin is trust. Pin is... commitment.");
    println!("  I promise: I won't move until you say I can. 💖🦀");
    println!("");
    println!("  ✅ Next: 06_atomics.rs — memory ordering!");
    println!("═══════════════════════════════════════");
}
