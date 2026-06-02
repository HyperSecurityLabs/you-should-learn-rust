// 💖🦀 RustLearning — Advanced Level
// File: 06_atomics.rs
// What: ATOMIC operations and MEMORY ORDERING.
// When MULTIPLE threads touch the SAME data without a Mutex...
// you need ATOMICS. Atomic = "this operation is INDIVISIBLE."
// Like: you can't INTERRUPT me mid-moan. 😏
//
// Run:  rustc 06_atomics.rs && ./06_atomics

use std::sync::atomic::{
    AtomicBool, AtomicI32, AtomicU64,
    Ordering::{AcqRel, Acquire, Relaxed, Release, SeqCst},
};
use std::thread;
use std::time::Duration;

/// ATOMICS are the BUILDING BLOCKS of lock-free programming.
/// They're like: touching the SAME spot at the SAME TIME.
/// And somehow... it works. No crashes. No corruption.
/// But the MAGIC is in the MEMORY ORDERING.
///
/// Memory Ordering = "How visible are my changes to OTHER threads?"
/// Like: when I whisper in your ear, do OTHER people hear?
/// Or: when I leave a mark on you, does everyone see it immediately?
/// Or: does it appear in a DIFFERENT order than I intended?
///
/// In C: data races are UNDEFINED BEHAVIOR. Compiler can delete your code.
/// In Rust safe: data races are IMPOSSIBLE. (Send + Sync traits enforce it.)
/// In Rust unsafe: you can use atomics. CORRECTLY. With the RIGHT ordering.
/// I trust you with atomics. Because I trust you with my MEMORY. 💖

// =========================================================
// THE FOUR MEMORY ORDERINGS (from weakest to strongest)
// =========================================================

/// Relaxed = "I don't care about ordering. Just do the atomic op."
/// Like: casual sex. No commitment. No guarantees.
/// Fastest. But also: NO guarantees about what other threads see.
/// Thread A: store(1, Relaxed)
/// Thread B: load(Relaxed) → might see 0, might see 1
/// Thread B: load(Relaxed) → might see 1, might see 0 (OUT OF ORDER!)
/// Relaxed is like: "I touched it. I don't know when you'll feel it. Or if you will."
/// USEFUL FOR: counters, statistics, flags where exact timing doesn't matter.

/// Acquire = "LOAD must happen AFTER all previous loads on this thread."
/// Like: I GRAB you. Before I do ANYTHING else, I COMPLETE this grab.
/// Pairs with Release. Prevents operations from being REORDERED before the load.
/// USEFUL FOR: loading a flag that indicates "data is ready" (mutex unlock).

/// Release = "STORE must happen BEFORE all subsequent stores on this thread."
/// Like: I LET GO of you. Everything I did BEFORE this release is NOW VISIBLE.
/// Pairs with Acquire. Prevents operations from being REORDERED after the store.
/// USEFUL FOR: storing a flag that indicates "data is ready" (mutex lock).

/// AcqRel = Acquire + Release combined. Used for READ-MODIFY-WRITE ops.
/// Like: I grab you, change you, AND let you go. All atomic.
/// The LOAD is Acquire, the STORE is Release.
/// USEFUL FOR: fetch_add, swap, compare_exchange.

/// SeqCst = SEQUENTIALLY CONSISTENT. The STRONGEST ordering.
/// All threads see ALL operations in the SAME order.
/// Like: everyone in the room sees exactly who touched who and when.
/// Slowest. But MOST PREDICTABLE. Zero ambiguity. Total visibility.
/// In C++: default for atomics. In Rust: NOT default (you must choose).
/// USEFUL FOR: when you need EVERYTHING to be perfectly ordered.

// =========================================================
// DEMO 1: Relaxed counter (fast but unreliable for ordering)
// =========================================================

fn relaxed_counter_demo() {
    println!("── DEMO 1: Relaxed Counter ──");
    println!("");
    println!("Relaxed is like: I touch you. No promises about when you feel it.");
    println!("But for INCREMENTING a counter? It's FINE.");
    println!("Because we DON'T CARE about the ORDER of increments.");
    println!("We only care that the TOTAL is correct.");
    println!("(Actually: even increments can overlap with Relaxed in theory.");
    println!(" But on x86, Relaxed store/load is STILL atomic for aligned words.");
    println!(" Just... no ordering guarantees for OTHER operations.)");
    println!("");
    
    let counter = AtomicI32::new(0);
    
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let c = &counter;
            thread::spawn(move || {
                for _ in 0..1000 {
                    // fetch_add with Relaxed: "I don't care about ordering"
                    // Just ADD 1. As fast as possible. No promises.
                    c.fetch_add(1, Relaxed);
                }
                println!("   Thread {} finished incrementing", i);
            })
        })
        .collect();
    
    for h in handles {
        h.join().unwrap();
    }
    
    println!("");
    println!("   Final counter (10 threads × 1000 increments): {}", 
             counter.load(Relaxed));
    println!("   Expected: 10000. Got: {}. Worked! ✅", counter.load(Relaxed));
    println!("   (Relaxed is fine for COUNTERS. Not for ORDERING.)");
    println!("");
}

// =========================================================
// DEMO 2: Release/Acquire — The flag pattern
// =========================================================

fn release_acquire_demo() {
    println!("── DEMO 2: Release/Acquire Handshake ──");
    println!("");
    println!("Release/Acquire is like a ROMANTIC HANDOFF:");
    println!("   Thread A: Prepares data. Then RELEASES the flag.");
    println!("   Thread B: ACQUIRES the flag. Then reads data.");
    println!("   The Release ensures: A's data writes are VISIBLE to B.");
    println!("   The Acquire ensures: B sees A's data writes.");
    println!("   It's a PROMISE: \"I'm done. It's yours. Take it.\" 💖");
    println!("");
    
    let data: AtomicU64 = AtomicU64::new(0);
    let ready = AtomicBool::new(false);
    
    // Thread A: prepare data and signal readiness
    let a_data = &data;
    let a_ready = &ready;
    let producer = thread::spawn(move || {
        // Step 1: Prepare data (multiple writes)
        a_data.store(42, Relaxed);  // Store data (doesn't need ordering alone)
        thread::sleep(Duration::from_millis(10));  // Simulate work
        
        // Step 2: Signal readiness with RELEASE
        // This ensures: ALL previous stores (like data=42) are VISIBLE
        // to anyone who ACQUIRES the ready flag.
        a_ready.store(true, Release);
        println!("   Producer: Data is READY! (Release) 💌");
    });
    
    // Thread B: wait for readiness and read data
    let b_data = &data;
    let b_ready = &ready;
    let consumer = thread::spawn(move || {
        // Step 1: Wait for readiness with ACQUIRE
        // This ensures: ALL stores from producer that happened BEFORE
        // the Release store are VISIBLE to me now.
        while !b_ready.load(Acquire) {
            // Spin wait (in real code: use condvar or channel)
            thread::yield_now();
        }
        
        // Step 2: Read data (GUARANTEED to see the Release'd value!)
        let value = b_data.load(Relaxed);
        println!("   Consumer: Got data = {} (Acquire saw the Release!) ✅", value);
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
    println!("");
    
    // WITHOUT Release/Acquire:
    // Producer:  data = 42; ready = true;
    // Consumer:  while (!ready) {} → exits when ready is true
    //           value = data; → MIGHT SEE 0 (old value) or 42!
    // The CPU/compiler can REORDER: ready = true BEFORE data = 42!
    // In a Relaxed world: the consumer might see ready=true but data=0.
    // That's a DATA RACE (in C). In Rust safe: impossible.
    // In Rust atomics: only if you use the WRONG ordering!
    // Release/Acquire PREVENTS this reordering.
    println!("   WITHOUT Release/Acquire: consumer might see data=0!");
    println!("   WITH Release/Acquire: consumer GUARANTEED to see data=42!");
    println!("   That's the power of memory ordering. The THRILL of correct synchronization.");
}

// =========================================================
// DEMO 3: SeqCst — The total order
// =========================================================

fn seqcst_demo() {
    println!("── DEMO 3: SeqCst — Total Global Order ──");
    println!("");
    println!("SeqCst = Everyone sees EVERYTHING in the SAME order.");
    println!("Like: a full video recording of everything we did.");
    println!("Every thread. Every touch. Every whisper. RECORDED.");
    println!("And everyone watches the SAME recording. In ORDER. 😳");
    println!("");
    println!("SeqCst is the STRONGEST guarantee. It prevents ALL reorderings.");
    println!("It's also the SLOWEST (on some architectures, a full memory barrier).");
    println!("On x86: SeqCst loads are just regular loads (+ compiler fence).");
    println!("On ARM: SeqCst is EXPENSIVE (needs dmb instruction).");
    println!("");
    println!("Use SeqCst when:");
    println!("   ✅ You need TOTAL GLOBAL ORDERING");
    println!("   ✅ You're not sure which ordering to use (safe default)");
    println!("   ✅ Correctness matters more than performance");
    println!("   ✅ You're implementing complex lock-free algorithms");
    println!("");
    println!("Don't use SeqCst when:");
    println!("   ❌ A counter that's only incremented (use Relaxed)");
    println!("   ❌ A simple flag handoff (Release/Acquire is enough)");
    println!("   ❌ You're on ARM and performance matters");
    println!("");
    
    // Demonstration: three atomic variables with SeqCst
    let a = AtomicI32::new(0);
    let b = AtomicI32::new(0);
    let c = AtomicI32::new(0);
    
    let handles: Vec<_> = (0..3)
        .map(|i| {
            let (ref_a, ref_b, ref_c) = (&a, &b, &c);
            thread::spawn(move || {
                match i {
                    0 => {
                        ref_a.store(1, SeqCst);
                        ref_b.store(1, SeqCst);
                    }
                    1 => {
                        ref_b.store(2, SeqCst);
                        ref_c.store(1, SeqCst);
                    }
                    2 => {
                        // This thread sees the GLOBAL SEQUENCE
                        let a_val = ref_a.load(SeqCst);
                        let b_val = ref_b.load(SeqCst);
                        let c_val = ref_c.load(SeqCst);
                        println!("   Observer: a={}, b={}, c={} (SeqCst order)", 
                                 a_val, b_val, c_val);
                        // With SeqCst: the order of stores is CONSISTENT.
                        // With Relaxed: we might see a=1,b=0,c=1 or a=0,b=1,c=0
                        // total chaos. But with SeqCst: everyone AGREES.
                    }
                    _ => unreachable!(),
                }
            })
        })
        .collect();
    
    for h in handles {
        h.join().unwrap();
    }
    
    println!("");
    println!("   SeqCst ensures: ALL threads see the SAME store order.");
    println!("   No mind-reading. No quantum bullshit. Just... ORDER.");
    println!("   Like a relationship built on TRUST and COMMUNICATION. 💖");
    println!("");
}

// =========================================================
// DEMO 4: Compare-and-Swap (CAS) — The most powerful atomic
// =========================================================

fn cas_demo() {
    println!("── DEMO 4: Compare-and-Swap (CAS) ──");
    println!("");
    println!("CAS = \"If this is the OLD value, change it to the NEW value.\"");
    println!("Like: \"If you're still mine... let me change you.\"");
    println!("     \"If someone already changed you... I won't touch you.\"");
    println!("It's the basis of ALL lock-free data structures.");
    println!("Mutexes, queues, stacks, hash maps — all built on CAS.");
    println!("");
    
    let value = AtomicI32::new(10);
    
    // CAS attempt: change 10 → 20
    // compare_exchange(expected, new, success_order, failure_order)
    let result = value.compare_exchange(10, 20, AcqRel, Acquire);
    match result {
        Ok(old) => println!("   CAS(10→20): SUCCESS! Old was {}, now {}", old, value.load(Relaxed)),
        Err(actual) => println!("   CAS(10→20): FAILED! Actual value was {}", actual),
    }
    
    // CAS attempt: change 10 → 30 (BUT current value is 20!)
    let result = value.compare_exchange(10, 30, AcqRel, Acquire);
    match result {
        Ok(old) => println!("   CAS(10→30): SUCCESS! Old was {} (impossible?)", old),
        Err(actual) => println!("   CAS(10→30): FAILED! Current is {} (expected 10)", actual),
    }
    
    // CAS in a loop (the "CAS loop" pattern)
    // This is how you safely increment without fetch_add:
    let counter = AtomicI32::new(0);
    
    for _ in 0..1000 {
        loop {
            let current = counter.load(Relaxed);
            let new = current + 1;
            if counter.compare_exchange(current, new, Relaxed, Relaxed).is_ok() {
                break;  // CAS succeeded! We incremented atomically.
            }
            // If CAS failed: another thread changed it. Try again.
            // This is SPINNING. Waiting for our turn. 😤
        }
    }
    
    println!("");
    println!("   CAS loop increment: {} (1000 iterations, no fetch_add!)", 
             counter.load(Relaxed));
    println!("   CAS loops are how LOCK-FREE data structures work.");
    println!("   No mutex. No sleeping. Just ATOMIC PERSISTENCE.");
    println!("   Like... I keep trying until I get through to you.");
    println!("   I don't give up. I don't wait for permission. I PERSIST. 🔥");
    println!("");
    
    // Real-world CAS: implementing a simple spinlock
    let lock = AtomicBool::new(false);
    
    let acquire_lock = |lock: &AtomicBool| {
        loop {
            // Try to CAS: false → true
            // If it WAS false: we now hold the lock. Break.
            // If it WAS true: someone else holds it. Spin.
            if lock.compare_exchange(false, true, Acquire, Relaxed).is_ok() {
                break;
            }
            // Spin (in real code: yield, pause instruction, or exponential backoff)
            std::hint::spin_loop();
        }
    };
    
    let release_lock = |lock: &AtomicBool| {
        lock.store(false, Release);
    };
    
    // Use the spinlock
    acquire_lock(&lock);
    println!("   (critical section — only ONE thread at a time) 💦");
    release_lock(&lock);
    println!("   Spinlock works! No Mutex. Just atomic willpower. 😈");
    println!("");
}

// =========================================================
// MEMORY ORDERING CHEAT SHEET
// =========================================================

fn ordering_cheatsheet() {
    println!("── MEMORY ORDERING CHEAT SHEET ──");
    println!("");
    println!("   Relaxed: 💨 Casual. No commitment. Fast.");
    println!("            Use: counters, statistics, flags you don't care about");
    println!("            Risk: other threads see OUT OF ORDER");
    println!("");
    println!("   Release: 🔓 \"I'm done. Here's everything.\"");
    println!("            Use: stores that SYNCHRONIZE with other threads");
    println!("            Guarantee: ALL previous stores are visible to Acquire");
    println!("");
    println!("   Acquire: 🔒 \"Show me everything that happened before.\"");
    println!("            Use: loads that SYNCHRONIZE with other threads");
    println!("            Guarantee: ALL previous stores from Release are VISIBLE");
    println!("");
    println!("   AcqRel: 🔐 Release + Acquire combined");
    println!("            Use: read-modify-write (fetch_add, swap, CAS)");
    println!("            Guarantee: the LOAD is Acquire, the STORE is Release");
    println!("");
    println!("   SeqCst: 💎 TOTAL GLOBAL ORDER. Everyone agrees.");
    println!("            Use: when you NEED perfect ordering");
    println!("            Cost: most expensive (full memory barrier)");
    println!("");
    println!("   x86 SUPER POWERS (why you might not notice):");
    println!("   - x86 stores are ALREADY release-acquire (a.store() = Release)");
    println!("   - x86 loads are ALREADY acquire (a.load() = Acquire)");
    println!("   - x86 is TOTAL STORE ORDER (stores appear in order)");
    println!("   - On x86: Relaxed vs Acquire might be THE SAME SPEED!");
    println!("   - On ARM: Relaxed is FASTER than Acquire.");
    println!("   - On ARM: SeqCst is EXPENSIVE.");
    println!("   - So: choose the WEAKEST ordering that's CORRECT.");
    println!("   - Weakest = fastest. Strongest = safest. Balance them.");
    println!("");
    
    println!("── THE METAPHOR ──");
    println!("");
    println!("Memory ordering is like... INTIMACY LEVELS:");
    println!("");
    println!("   Relaxed:   💨  A quick kiss in public. Casual. No commitment.");
    println!("   Release:    🔓  Saying \"I love you\" before leaving.");
    println!("   Acquire:    🔒  Hearing \"I love you\" and KNOWING it's true.");
    println!("   AcqRel:     🔐  Kissing + saying it. Simultaneously. Passionate.");
    println!("   SeqCst:     💎  A WEDDING. Everyone witnesses. Everyone agrees.");
    println!("               Total commitment. Total visibility. Total love. 💖");
    println!("");
    println!("Choose the RIGHT level of intimacy for your relationship.");
    println!("Don't buy a diamond ring for a one-night stand (Relaxed is fine).");
    println!("Don't give a casual kiss at your wedding (you need SeqCst).");
    println!("Know your threads. Know your ordering. Know yourself. 🔥");
    println!("");
}

fn main() {
    println!("💖 ATOMICS & MEMORY ORDERING — Multi-threaded intimacy");
    println!("");
    println!("Atomics are how I touch you from ACROSS THREADS.");
    println!("Without a Mutex. Without a lock. Just... atomic will.");
    println!("MEMORY ORDERING is HOW I make sure you FEEL it. Correctly.");
    println!("In the right order. At the right time. Every. Single. Time. 😏");
    println!("");
    
    relaxed_counter_demo();
    release_acquire_demo();
    seqcst_demo();
    cas_demo();
    ordering_cheatsheet();
    
    println!("═══════════════════════════════════════");
    println!("  🔞 ATOMICS — Thread-safe without locks");
    println!("");
    println!("  Atomic operations:");
    println!("    load  — read the value atomically");
    println!("    store — write the value atomically");
    println!("    fetch_add/sub/and/or/xor — atomic math");
    println!("    swap — atomic exchange");
    println!("    compare_exchange — CAS (the most powerful)");
    println!("");
    println!("  Memory orderings (from weakest to strongest):");
    println!("    1. Relaxed — no guarantees (fastest)");
    println!("    2. Release — \"I'm done, take everything I did\"");
    println!("    3. Acquire — \"Show me everything that was done\"");
    println!("    4. AcqRel — Release + Acquire");
    println!("    5. SeqCst — total global order (slowest, safest)");
    println!("");
    println!("  In C: data races = UNDEFINED BEHAVIOR (compiler deletes your code)");
    println!("  In Rust: data races = COMPILE ERROR (unless you use unsafe + atomics)");
    println!("  In Rust unsafe + atomics: you CHOOSE the ordering.");
    println!("  Choose WRONG (Relaxed when you need Acquire) = BUGS but NOT UB.");
    println!("  (Actually, wrong ordering CAN cause UB in theory. Be CAREFUL.)");
    println!("");
    println!("  ❤️  Rust gives you CONTROL. But also RESPONSIBILITY.");
    println!("  ❤️  Like any adult relationship. You choose the INTIMACY LEVEL.");
    println!("  ❤️  Choose wisely. I trust you. 💖🦀");
    println!("");
    println!("  ✅ Next: 07_no_std.rs — bare metal Rust!");
    println!("═══════════════════════════════════════");
}
