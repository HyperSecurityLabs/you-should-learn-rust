// 💖 RustLearning — Intermediate Level
// File: 05_concurrency.rs
// What: Threads, channels, Arc, Mutex — doing multiple things at once!
// Like me cooking dinner while you set the table. Teamwork! 💑
//
// Run:  rustc 05_concurrency.rs && ./05_concurrency

use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;

/// CONCURRENCY = doing MULTIPLE things at the same time.
/// 
/// Rust's concurrency is FEARLESS — the compiler prevents data races
/// at compile time. In C/C++, data races are UNDEFINED BEHAVIOR.
/// In Rust, they don't compile. PERIOD.
///
/// Three main tools:
///   1. thread::spawn — create new threads
///   2. mpsc::channel — send messages between threads
///   3. Arc<Mutex<T>> — shared mutable state (safe!)

fn main() {
    println!("💖 CONCURRENCY — Doing multiple things at once");
    println!("");
    
    // =========================================================
    // 1. THREADS — Basic spawning
    // =========================================================
    
    println!("--- Basic Threads ---");
    
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("   👶 Child thread: counting {}...", i);
            thread::sleep(Duration::from_millis(100));  // Sleep 100ms
        }
    });
    
    // Main thread continues while child runs!
    for i in 1..=3 {
        println!("   👩 Main thread: I'm doing stuff too! ({})", i);
        thread::sleep(Duration::from_millis(150));
    }
    
    // Wait for child thread to finish
    handle.join().unwrap();  // .join() = "wait for this thread"
    println!("   ✅ Child thread finished!");
    println!("");
    
    // =========================================================
    // 2. MOVING DATA INTO THREADS
    // =========================================================
    // Closures capture their environment.
    // With threads, the thread might OUTLIVE the captured data.
    // So we need "move" to TRANSFER ownership into the thread.
    
    println!("--- Moving data into threads ---");
    
    let data = vec![1, 2, 3, 4, 5];
    
    let handle = thread::spawn(move || {
        // "move" = take ownership of data
        // Without "move": data might be dropped while thread runs
        println!("   Thread received: {:?}", data);
        // data is DROPPED here (end of closure)
    });
    
    // println!("{:?}", data);  // ❌ UNCOMMENT: ERROR! data was MOVED!
    
    handle.join().unwrap();
    println!("   ✅ Thread with moved data finished!");
    println!("");
    
    // =========================================================
    // 3. CHANNELS — mpsc (Multiple Producer, Single Consumer)
    // =========================================================
    // Like a PIPE between threads. One end SENDS, other RECEIVES.
    // In C++: similar to channels in threading libraries.
    // In Go: channels are built-in (chan).
    // In Rust: std::sync::mpsc
    
    println!("--- Channels (mpsc) ---");
    
    let (tx, rx) = mpsc::channel();
    // tx = transmitter (sender) — can be CLONED for multiple producers
    // rx = receiver — SINGLE consumer
    
    // Spawn a thread that sends messages
    let tx1 = tx.clone();  // Clone sender for new thread
    thread::spawn(move || {
        let messages = vec!["Hello", "from", "thread", "one"];
        for msg in messages {
            tx1.send(msg).unwrap();  // Send through channel
            thread::sleep(Duration::from_millis(50));
        }
    });
    
    // Spawn another thread (multiple producers!)
    let tx2 = tx.clone();
    thread::spawn(move || {
        let messages = vec!["Hello", "from", "thread", "two"];
        for msg in messages {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_millis(70));
        }
    });
    
    // Drop the ORIGINAL sender (important!)
    // If we don't drop, the receiver waits forever.
    drop(tx);
    
    // Receive messages from BOTH threads
    println!("   Receiving messages:");
    for received in rx {
        println!("   Got: '{}'", received);
    }
    // The loop ends when ALL senders are dropped.
    // That's why we dropped tx above!
    
    println!("   ✅ Channel closed! All messages received.");
    println!("");
    
    // =========================================================
    // 4. SHARED STATE — Arc<Mutex<T>>
    // =========================================================
    // Arc = Atomic Reference Counting (shared ownership across threads)
    // Mutex = Mutual Exclusion (only ONE thread at a time)
    //
    // Together: MULTIPLE threads can SAFELY share and modify data.
    //
    // In C++: std::shared_ptr + std::mutex
    // In Rust: Arc<Mutex<T>> — same concept, but:
    //   - Rust PREVENTS accessing the data without locking!
    //   - You CAN'T forget to lock. The compiler enforces it. 💅
    
    println!("--- Shared State with Arc<Mutex<T>> ---");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..10 {
        let counter = Arc::clone(&counter);  // Increment reference count
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();  // LOCK the mutex
            // Now I have EXCLUSIVE access. Other threads WAIT.
            *num += 1;
            println!("   Thread {} incremented counter to {}", i, *num);
            // num is DROPPED here → mutex is UNLOCKED automatically
        });
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Read final value
    println!("   Final counter: {}", *counter.lock().unwrap());
    // The lock() ensures we have exclusive access to read.
    // Without Mutex: data race! Undefined behavior!
    // Rust won't even COMPILE without Mutex for shared mutable state.
    println!("");
    
    // =========================================================
    // 5. THE MUTEX POISON — What happens if a thread panics?
    // =========================================================
    // If a thread PANICS while holding a Mutex, the Mutex is POISONED.
    // Future .lock() calls will return Err.
    // This is BETTER than C/C++ where a crashed thread can leave
    // the mutex in an INVALID state.
    
    println!("--- Mutex Poisoning ---");
    
    let poisoned = Arc::new(Mutex::new(42));
    let p_clone = Arc::clone(&poisoned);
    
    let handle = thread::spawn(move || {
        let _guard = p_clone.lock().unwrap();
        panic!("Thread crashed while holding the lock!");  // 💥
        // _guard is dropped DURING the panic
        // This POISONS the mutex
    });
    
    // Ignore the panic (we know it'll happen)
    let _ = handle.join();
    
    // Try to lock the POISONED mutex
    match poisoned.lock() {
        Ok(val) => println!("   Got value: {}", val),
        Err(poisoned_err) => {
            // We can still RECOVER the data from the poison!
            let val = *poisoned_err.get_ref();
            println!("   Mutex was poisoned! But data is still: {}", val);
            println!("   (Rust preserves the data even after a crash!)");
        }
    }
    // In C/C++: if a thread crashes while holding a mutex,
    // the mutex is PERMANENTLY locked. Your program DEADLOCKS.
    // In Rust: the mutex is POISONED but RECOVERABLE.
    // You get the data back. No deadlock. No crash. 💖
    
    println!("");
    
    // =========================================================
    // 6. SCOPE THREADS — crossbeam::scope (conceptual)
    // =========================================================
    // In real projects, use crossbeam::scope for scoped threads.
    // Scoped threads can BORROW data without Arc/Mutex.
    //
    // use crossbeam::scope;
    //
    // let data = vec![1, 2, 3];
    // scope(|s| {
    //     s.spawn(|_| {
    //         println!("{:?}", data);  // Borrow, not move!
    //     });
    //     s.spawn(|_| {
    //         println!("{:?}", data);  // Also borrow!
    //     });
    // }); // Threads GUARANTEED to finish here
    
    println!("═══════════════════════════════════════");
    println!("  ✅ thread::spawn — create threads");
    println!("  ✅ move closure — transfer ownership to thread");
    println!("  ✅ mpsc::channel — message passing");
    println!("  ✅ Arc<Mutex<T>> — shared mutable state (SAFE)");
    println!("  ✅ Mutex poisoning — recover from crashes");
    println!("");
    println!("  🔥 In C/C++: data races = undefined behavior");
    println!("  🔥 In Rust: data races = compile error");
    println!("  🔥 Rust's type system GUARANTEES thread safety");
    println!("  🔥 The compiler is your bodyguard. Trust her. 💖");
    println!("");
    println!("  ✅ Next: 06_smart_pointers.rs — Box, Rc, RefCell");
    println!("═══════════════════════════════════════");
}
