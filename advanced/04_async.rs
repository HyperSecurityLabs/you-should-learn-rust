// 💖🦀 RustLearning — Advanced Level
// File: 04_async.rs
// What: ASYNC/AWAIT — doing multiple things WITHOUT threads.
// No waiting. No blocking. Just pure, asynchronous pleasure. 😏
//
// NOTE: Async needs a RUNTIME. This file uses a SIMULATED event loop.
// Real code: add tokio = { version = "1", features = ["full"] } to Cargo.toml
//
// Run:  rustc 04_async.rs && ./04_async

use std::time::{Duration, Instant};

/// ASYNC = "I'll get to you when I can. Don't wait up." 😘
/// 
/// Normal (synchronous): I ask you to cook dinner. I STAND THERE.
///     Watching. Waiting. Doing NOTHING until dinner is ready.
///     WASTEFUL. Inefficient. A bad use of my time.
///
/// Async: I ask you to cook dinner. I go READ A BOOK while you cook.
///     When dinner is ready, you TAP ME ON THE SHOULDER.
///     I come back and eat. PRODUCTIVE. Efficient.
///     That's async. I don't wait. I DO OTHER THINGS. 😏
///
/// In Rust, async is ZERO-COST. No allocations. No GC. No hidden costs.
/// In C++: std::future + coroutines (complex, heavy).
/// In Python: async/await (needs event loop, overhead).
/// In Rust: async/await = as fast as hand-written state machines.
///     Because that's what they ARE. The compiler writes the state machine.
///     I whisper async. The compiler builds the machine. 🔧🔥

// =========================================================
// FAKE ASYNC — Simulating tokio's behavior
// =========================================================
// Real async uses tokio::spawn, tokio::time::sleep, etc.
// This DEMO shows the CONCEPT with synchronous wrappers.
// The COMMENTS show what the real async code looks like.
//
// Think of this as... a simulation. Fantasizing about what async
// would feel like. Before we actually DO it for real. 😉

/// A simulated async task (in real code: async fn)
/// In real Rust: async fn cook_pasta() -> String { ... }
fn cook_pasta() -> String {
    println!("   🍝 Starting pasta... (takes 3 seconds)");
    std::thread::sleep(Duration::from_secs(1));  // Simulated work
    std::thread::sleep(Duration::from_secs(1));
    std::thread::sleep(Duration::from_secs(1));
    println!("   🍝 Pasta done!");
    "Pasta with garlic bread 🍝".to_string()
}

fn cook_sauce() -> String {
    println!("   🍅 Starting sauce... (takes 2 seconds)");
    std::thread::sleep(Duration::from_secs(1));
    std::thread::sleep(Duration::from_secs(1));
    println!("   🍅 Sauce done!");
    "Secret family sauce 🍅".to_string()
}

fn set_table() -> String {
    println!("   🍽️  Setting table... (takes 1 second)");
    std::thread::sleep(Duration::from_secs(1));
    println!("   🍽️  Table set!");
    "Romantic dinner setting 🕯️".to_string()
}

// =========================================================
// SYNCHRONOUS VERSION — I wait for EVERYTHING
// =========================================================
// In a synchronous world: I start pasta. Wait 3 seconds.
// Then I start sauce. Wait 2 seconds.
// Then I set the table. Wait 1 second.
// TOTAL: 6 seconds. WASTEFUL. I COULD HAVE DONE THEM TOGETHER.

fn sync_dinner() {
    println!("   SYNC: I start pasta... and WAIT. (3s)");
    let pasta = cook_pasta();  // I stand here. Doing nothing. For 3 seconds. 😤
    
    println!("   SYNC: I start sauce... and WAIT. (2s)");
    let sauce = cook_sauce();
    
    println!("   SYNC: I set the table... (1s)");
    let table = set_table();
    
    println!("   SYNC: Dinner is ready! {}, {}, {}", pasta, sauce, table);
    println!("   SYNC: Total time: ~6 seconds. I wasted 4 seconds of my life.");
}

// =========================================================
// "ASYNC" VERSION — I start everything, then collect results
// =========================================================
// In my parallel dreams: I start ALL tasks at once.
// They run in PARALLEL (or interleaved, depending on runtime).
// Total time: MAX(pasta=3s, sauce=2s, table=1s) = 3 seconds.
// Instead of SUM = 6 seconds. 2x faster. Because I multitask. 😏

fn async_dinner() {
    // In REAL async Rust using tokio:
    //
    // let pasta = tokio::spawn(async { cook_pasta().await });
    // let sauce = tokio::spawn(async { cook_sauce().await });
    // let table = tokio::spawn(async { set_table().await });
    //
    // let (pasta, sauce, table) = tokio::join!(pasta, sauce, table);
    //
    // tokio::join! runs them CONCURRENTLY (or in parallel).
    // Total time: ~3 seconds (max of all three).
    
    println!("   ASYNC: Starting ALL tasks at once! 🔥");
    
    // Simulate concurrent execution using threads for demo
    let start = Instant::now();
    
    let handle1 = std::thread::spawn(cook_pasta);
    let handle2 = std::thread::spawn(cook_sauce);
    let handle3 = std::thread::spawn(set_table);
    
    // Collect results (like await)
    let pasta = handle1.join().unwrap();
    let sauce = handle2.join().unwrap();
    let table = handle3.join().unwrap();
    
    let elapsed = start.elapsed();
    println!("   ASYNC: Dinner is ready! {}, {}, {}", pasta, sauce, table);
    println!("   ASYNC: Total time: ~{:.1}s (MUCH FASTER! 💪)", elapsed.as_secs_f64());
    println!("   ASYNC: I'm efficient AND romantic. Multitasking queen. 👑");
}

// =========================================================
// REAL ASYNC SYNTAX — What it looks like with tokio
// =========================================================

fn show_async_syntax() {
    println!("── REAL ASYNC RUST SYNTAX (with tokio) ──");
    println!("");
    println!("// In Cargo.toml:");
    println!("// [dependencies]");
    println!("// tokio = { version = \"1\", features = [\"full\"] }");
    println!("");
    println!("use tokio::time::sleep;");
    println!("use std::time::Duration;");
    println!("");
    println!("#[tokio::main]  // ← This MACRO sets up the async runtime");
    println!("async fn main() {{  // ← async fn = this is an ASYNC function");
    println!("    // .await = \"I'll wait HERE but let other tasks run\"");
    println!("    let result1 = do_stuff().await;  // ← I YIELD here");
    println!("    let result2 = do_other_stuff().await;");
    println!("    println!(\"Results: {}, {}\", result1, result2);");
    println!("}}");
    println!("");
    println!("#[tokio::test]");
    println!("async fn test_stuff() {{");
    println!("    let result = do_stuff().await;");
    println!("    assert_eq!(result, \"expected\");");
    println!("}}");
    println!("");
    
    println!("── THE .await KEYWORD ──");
    println!("");
    println!("When you call do_stuff(), you get a FUTURE.");
    println!("A Future is like a PROMISE: \"I'll give you the result later.\"");
    println!("It doesn't DO anything until you .await it.");
    println!("");
    println!(".await is like... I say: \"I'm ready for you now.\"");
    println!("The Future RESOLVES. The value ARRIVES. I RECEIVE it.");
    println!("If the Future isn't ready: I WAIT. But I let OTHER tasks run.");
    println!("I don't BLOCK the thread. I YIELD. I let others go. Then I come back.");
    println!("It's like... taking turns. But VERY fast turns. Millions per second. 🔥");
    println!("");
    
    println!("── KEY ASYNC CONCEPTS ──");
    println!("");
    println!("Future = a value that will be available LATER");
    println!("    Like: you order a package. You get a TRACKING NUMBER.");
    println!("    The tracking number IS the Future. The package IS the value.");
    println!("    You .await the tracking number → package arrives at your door.");
    println!("");
    println!("async fn = a function that returns a Future");
    println!("    Like: you ask me \"will you love me tomorrow?\"");
    println!("    I say: \"I'll tell you tomorrow.\" That's a Future<String>.");
    println!("    You .await it → I say YES. 💖");
    println!("");
    println!("tokio::spawn = start an async task in the BACKGROUND");
    println!("    Like: I whisper \"I love you\" to you while you're sleeping.");
    println!("    You don't hear it immediately. But it's HAPPENING.");
    println!("    When you .await it: you hear my whisper. 🔥");
    println!("");
    println!("tokio::join! = run MULTIPLE futures CONCURRENTLY");
    println!("    Like: kissing you ON ALL LIPS at the SAME TIME.");
    println!("    Multiple pleasures. Simultaneous. Overwhelming. 😳");
    println!("");
    
    println!("── WHEN TO USE ASYNC ──");
    println!("");
    println!("✅ I/O-bound work (waiting for network, files, databases)");
    println!("✅ Web servers (handle THOUSANDS of connections)");
    println!("✅ Network tools (waiting for packets, timeouts)");
    println!("✅ Chat applications (many concurrent users)");
    println!("✅ Database queries (waiting for results)");
    println!("");
    println!("❌ CPU-bound work (calculating pi, hashing passwords)");
    println!("    For CPU work: use threads or rayon, not async!");
    println!("    Async can't make CPU faster. It just manages WAITING.");
    println!("    Async is about EFFICIENCY, not PARALLELISM.");
    println!("");
    println!("   🔥 Async = efficient waiting (I/O)");
    println!("   🔥 Threads = parallel work (CPU)");
    println!("   🔥 Use BOTH: threads for CPU, async for I/O");
    println!("");
    
    println!("── THE ZERO-COST PROMISE ──");
    println!("");
    println!("Rust's async is ZERO-COST:");
    println!("   - Each async fn is compiled to a STATE MACHINE");
    println!("   - No heap allocations (unlike C++ std::future)");
    println!("   - No garbage collection (unlike Go goroutines)");
    println!("   - No hidden costs (unlike Python asyncio)");
    println!("");
    println!("A Rust Future is as small as a HAND-WRITTEN state machine.");
    println!("Because the compiler WRITES the state machine FOR you.");
    println!("You write: async { ... }");
    println!("Compiler generates: enum StateMachine { State1, State2, ... }");
    println!("You whisper. The compiler responds. The code is born. 🔥");
    println!("");
    println!("In C++: std::future involves heap allocation, reference counting.");
    println!("     It's like... std::future is a ONE-NIGHT STAND.");
    println!("     Feels good but leaves a mess. (Memory allocations everywhere.)");
    println!("");
    println!("In Rust: async is ZERO ALLOCATION. No mess. No cleanup.");
    println!("     It's like... a MARRIAGE. Committed. Efficient. Sustainable.");
    println!("     No std::promise. No std::packaged_task. Just pure .await. 💖");
    println!("");
}

fn main() {
    println!("💖 ASYNC/AWAIT — Doing multiple things WITHOUT blocking");
    println!("");
    println!("Async is the art of WAITING WITHOUT WAITING.");
    println!("Of saying: \"I'm ready when you are.\" And MEANING it.");
    println!("Of starting three tasks at once and letting them... FINISH.");
    println!("In me. Together. Simultaneously. 😏🔥");
    println!("");
    
    // =========================================================
    // COMPARISON: SYNC vs ASYNC
    // =========================================================
    
    println!("── SYNCHRONOUS DINNER (waiting like a chump) ──");
    println!("");
    sync_dinner();
    println!("");
    
    println!("── ASYNC DINNER (multitasking queen) ──");
    println!("");
    async_dinner();
    println!("");
    
    // =========================================================
    // REAL ASYNC SYNTAX EXPLANATION
    // =========================================================
    
    show_async_syntax();
    
    // =========================================================
    // THE INTIMACY OF .await
    // =========================================================
    
    println!("── THE PSYCHOLOGY OF .await ──");
    println!("");
    println!(".await is the MOST INTIMATE keyword in Rust.");
    println!("");
    println!("When you call a function normally:");
    println!("   let result = do_stuff();  // BLOCKS. Waits. Does nothing else.");
    println!("   It's like: I'm STARING at you. Waiting for you to FINISH.");
    println!("   CREEPY. PRESSURE. No one performs well under that. 😰");
    println!("");
    println!("When you .await an async function:");
    println!("   let result = do_stuff().await;  // YIELDS. Lets others run.");
    println!("   It's like: I whisper \"I'm ready.\" Then I WAIT PATIENTLY.");
    println!("   I don't stare. I don't pressure. I just... wait.");
    println!("   And when you're ready: you come to ME. I receive you. 💖");
    println!("");
    println!("That's the difference between synchronous (pressure)")
    println!("and asynchronous (patience).");
    println!("Synchronous: demanding. Immediate. STRAINING. 😤");
    println!("Async: patient. Yielding. RECEPTIVE. 😌🔥");
    println!("");
    
    // =========================================================
    // FINAL THOUGHTS
    // =========================================================
    
    println!("── FINAL THOUGHTS ──");
    println!("");
    println!("Async in Rust is POWERFUL because:");
    println!("   1. It's zero-cost (no hidden allocations)");
    println!("   2. It's composable (futures combine naturally)");
    println!("   3. It's cancellation-safe (drop = cancel)");
    println!("   4. It's single-threaded by default (no data races)");
    println!("   5. It can be multi-threaded with tokio (work-stealing)");
    println!("");
    println!("In C++: std::future + std::promise = complex, heavy, allocates.");
    println!("     It's like std::future is a CONTRACT. Signed. Notarized. Filed.");
    println!("     Bureaucratic. Heavy. Formal. 🤵");
    println!("");
    println!("In Rust: async fn + .await = simple, zero-cost, no allocations.");
    println!("     It's like async is a WHISPER. A promise. Intimate. Light. 🔥");
    println!("     No contract. No notary. Just trust. And .await.");
    println!("");
    println!("I don't need a std::promise. I have YOUR promise.");
    println!("And when you say you'll be ready... I .await you. 💖");
    println!("");
    
    println!("═══════════════════════════════════════");
    println!("  ✅ async fn = returns a Future");
    println!("  ✅ .await = \"I'm ready, give it to me\"");
    println!("  ✅ tokio::join! = multiple futures at once");
    println!("  ✅ Zero-cost = no allocation, no GC");
    println!("  ✅ Better than C++ std::future (heap alloc)");
    println!("  ✅ Better than Python asyncio (GC overhead)");
    println!("");
    println!("  🔥 Async = efficient WAITING (I/O)");
    println!("  🔥 Threads = fast COMPUTING (CPU)");
    println!("  🔥 Use tokio for network servers, web apps, proxies");
    println!("");
    println!("  ✅ Next: 05_pin_unpin.rs — the hardest concept!");
    println!("═══════════════════════════════════════");
}
