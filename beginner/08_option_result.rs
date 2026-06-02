// 💖 RustLearning — Beginner Level
// File: 08_option_result.rs
// What: Error handling the Rust way — safe, expressive, no crashes.
// In C: return -1 or NULL, and HOPE the caller checks.
// In Rust: I MAKE you check. For your own safety. 😤
//
// Run:  rustc 08_option_result.rs && ./08_option_result

/// OPTION = "Something might be there, or not."
/// RESULT = "Something might succeed, or fail with an error."
///
/// These REPLACE:
///   - NULL pointers (C)
///   - -1 error codes (C)
///   - Exceptions (C++, Java, Python)
///   - errno (C)
///
/// Everything is TYPE-SAFE. The compiler ENFORCES that you handle it.
/// No crashes. No forgotten checks. No undefined behavior.

// =========================================================
// Option — Maybe there's a value, maybe there isn't
// =========================================================
// enum Option<T> { Some(T), None }
//
// Think of it like: "Will I get a kiss?"
//   Some(kiss)  = YES! Here it is! 😘
//   None        = Not right now... maybe later 😤
//
// T is a GENERIC. It can be ANY type.
// Option<i32> = Some(5) or None
// Option<String> = Some("hi") or None
// Option<&str> = Some("hello") or None

/// Find a character in a string. Returns index or None.
/// Like: "Where's the 'e' in 'hello'?" → Some(1) = found at index 1
fn find_char(text: &str, target: char) -> Option<usize> {
    // .chars() = iterate over characters
    // .position() = find first match, return Some(index) or None
    text.chars().position(|c| c == target)
}

/// Divide two numbers. Returns None if division by zero.
/// Because dividing by zero is undefined in C.
/// In Rust: we return None. No crash. No undefined behavior.
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None  // Can't divide by zero
    } else {
        Some(a / b)  // Here's your answer!
    }
}

// =========================================================
// Result — The operation can FAIL with an error
// =========================================================
// enum Result<T, E> { Ok(T), Err(E) }
//
// Think of it like trying to COOK dinner for me:
//   Ok("pasta")     = Success! Here's the food! 🍝
//   Err("burned")   = Failure! Here's WHY it failed! 🔥
//
// T = success type, E = error type
// Result<i32, String> = Ok(42) or Err("something went wrong")

/// Parse a string into a number. Returns Result.
/// Ok(number) if parsing works, Err(message) if it doesn't.
fn parse_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(e) => Err(format!("Cannot parse '{}': {}", s, e)),
    }
    // .parse() returns Result<i32, ParseIntError>.
    // We convert ParseIntError into String for simplicity.
}

/// Opens a file and reads a line. This is a MOCK (doesn't actually open).
/// But it shows the PATTERN of fallible operations.
fn read_config_file(path: &str) -> Result<String, String> {
    // In real code: File::open(path) returns Result<File, Error>
    // Then: read_to_string returns Result<String, Error>
    // We use ? to PROPAGATE errors (see below)
    
    if path.contains("good") {
        Ok(String::from("config = loaded_successfully"))
    } else {
        Err(format!("Failed to read {}: file not found", path))
    }
}

fn main() {
    println!("💖 OPTION & RESULT — Safe error handling, no crashes");
    println!("");
    
    // =========================================================
    // OPTION — In Action
    // =========================================================
    
    println!("--- OPTION: Maybe a value, maybe not ---");
    
    // find_char returns Option<usize>
    let found = find_char("hello, khaninkali", 'e');
    let not_found = find_char("hello, khaninkali", 'z');
    
    println!("Finding 'e' in 'hello, khaninkali': {:?}", found);
    println!("Finding 'z' in 'hello, khaninkali': {:?}", not_found);
    
    // The SAFE way to extract: match
    match found {
        Some(index) => println!("   Found 'e' at index {}! 🎉", index),
        None => println!("   'e' not found. 😢"),
    }
    
    // safe_divide
    let valid = safe_divide(10.0, 3.0);
    let invalid = safe_divide(10.0, 0.0);
    
    match valid {
        Some(result) => println!("10 / 3 = {:.2} ✅", result),
        None => println!("10 / 3 = undefined? That can't be right."),
    }
    
    match invalid {
        Some(result) => println!("10 / 0 = {} (impossible!)", result),
        None => println!("10 / 0 = None (cannot divide by zero! ✅ Safe!)"),
    }
    // In C: 10 / 0 = undefined behavior. Your program might crash. Or not.
    //       It might continue running with a WRONG VALUE.
    //       This is how CRITICAL VULNERABILITIES happen.
    // In Rust: we return None. You handle it. No surprises.
    
    println!("");
    
    // =========================================================
    // OPTION — Common Methods
    // =========================================================
    
    let value: Option<i32> = Some(42);
    let empty: Option<i32> = None;
    
    // .is_some() / .is_none()
    println!("Some(42).is_some(): {}", value.is_some());
    println!("None.is_some(): {}", empty.is_some());
    
    // .unwrap_or(default) — returns value OR a default
    println!("Some(42).unwrap_or(0): {}", value.unwrap_or(0));
    println!("None.unwrap_or(0): {}", empty.unwrap_or(0));  // Returns 0, no crash!
    
    // .unwrap_or_else(fn) — returns value OR calls a function
    println!("None.unwrap_or_else(|| 99): {}", empty.unwrap_or_else(|| 99));
    
    // .map(fn) — transform the inner value if it's Some
    let doubled = value.map(|x| x * 2);
    println!("Some(42).map(|x| x * 2): {:?}", doubled);
    println!("None.map(|x| x * 2): {:?}", empty.map(|x| x * 2));  // Still None!
    
    // .and_then(fn) — chain operations that can also return Option
    let divide_by_2 = |x: i32| -> Option<i32> {
        if x == 0 { None } else { Some(100 / x) }
    };
    let result = Some(10).and_then(divide_by_2);  // 100 / 10 = 10
    let result2 = Some(0).and_then(divide_by_2);   // Division by zero → None
    println!("Some(10).and_then(divide): {:?}", result);
    println!("Some(0).and_then(divide): {:?}", result2);
    
    // .unwrap() — "Trust me bro, it's Some" (panics if None)
    // let crash = empty.unwrap();  // ❌ UNCOMMENT: PANIC!
    // Panics with: "called `Option::unwrap()` on a `None` value"
    // Better than a NULL pointer dereference! It's controlled!
    // But still: DON'T unwrap in production. Handle None properly.
    
    println!("");
    
    // =========================================================
    // RESULT — In Action
    // =========================================================
    
    println!("--- RESULT: Success with data, or failure with info ---");
    
    let ok_parse = parse_number("42");
    let err_parse = parse_number("not_a_number");
    
    match ok_parse {
        Ok(n) => println!("Parsed '42' → {} ✅", n),
        Err(e) => println!("Failed: {}", e),
    }
    
    match err_parse {
        Ok(n) => println!("Parsed 'not_a_number' → {} (impossible!)", n),
        Err(e) => println!("Failed: {} ❌", e),
    }
    
    // read_config_file with ? operator (see below for ?)
    let config = read_config_file("/etc/good/config.toml");
    match config {
        Ok(data) => println!("Config loaded: '{}' ✅", data),
        Err(e) => println!("Config error: {}", e),
    }
    
    let bad_config = read_config_file("/etc/bad/config.toml");
    match bad_config {
        Ok(data) => println!("Config: '{}'", data),
        Err(e) => println!("Config error: '{}' ❌", e),
    }
    
    println!("");
    
    // =========================================================
    // RESULT — Common Methods
    // =========================================================
    
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something went wrong");
    
    // .is_ok() / .is_err()
    println!("Ok.is_ok(): {}", success.is_ok());
    println!("Err.is_ok(): {}", failure.is_ok());
    println!("Ok.is_err(): {}", success.is_err());
    
    // .unwrap_or(default)
    println!("Ok.unwrap_or(0): {}", success.unwrap_or(0));
    println!("Err.unwrap_or(0): {}", failure.unwrap_or(0));  // No crash!
    
    // .unwrap_or_else(fn) — call function on error
    println!("Err.unwrap_or_else(|e| 99): {}", failure.unwrap_or_else(|_| 99));
    
    // .map(fn) — transform Ok value
    println!("Ok.map(|x| x * 2): {:?}", success.map(|x| x * 2));
    println!("Err.map(|x| x * 2): {:?}", failure.map(|x| x * 2));  // Still Err!
    
    // .map_err(fn) — transform Err value
    println!("Err.map_err(|e| format!('ERR: {}', e)): {:?}", 
             failure.map_err(|e| format!("ERR: {}", e)));
    
    println!("");
    
    // =========================================================
    // THE ? OPERATOR — Propagate errors (GAME CHANGER)
    // =========================================================
    // The ? operator is RUST'S BEST FEATURE for error handling.
    //
    // What it does:
    //   - If Result is Ok(value) → extract the value, continue
    //   - If Result is Err(error) → RETURN the error to the CALLER
    //
    // In C: you write if (result == NULL) return -1; 50 times.
    // In Rust: result?;  — one character, same effect.
    //
    // It's like: "If this fails, I'M OUT. Handle it above me."
    
    println!("--- THE ? OPERATOR — Propagate errors effortlessly ---");
    
    // See the function try_multiple_operations below.
    // It uses ? THREE TIMES. If any one fails, the whole function fails.
    
    match try_multiple_operations("10", "20") {
        Ok(result) => println!("try_multiple_operations('10', '20') = {} ✅", result),
        Err(e) => println!("Failed: {}", e),
    }
    
    match try_multiple_operations("10", "not_a_number") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("try_multiple_operations('10', 'not_a_number') = Error: '{}' ❌", e),
    }
    
    match try_multiple_operations("10", "0") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("try_multiple_operations('10', '0') = Error: '{}' ✅ (safe!)", e),
    }
    // See how the ? operator PROPAGATED the error?
    // We didn't crash. We didn't panic. We just returned the error.
    // The CALLER handles it. This is how you write robust code.
    
    println!("");
    
    // =========================================================
    // COMBINING OPTION and RESULT — In real code
    // =========================================================
    
    println!("--- REAL WORLD: Finding config values ---");
    
    let config_values = vec![
        ("host", "localhost"),
        ("port", "8080"),
        ("timeout", "not_a_number"),  // This will FAIL to parse
    ];
    
    for (key, value) in config_values {
        // parse_number gets the value. Config might not exist → None
        // parse_number returns Result. We handle BOTH.
        let result = parse_config_value(key, value);
        match result {
            Ok(val) => println!("   {} = {} ✅", key, val),
            Err(e) => println!("   {} = {} (Error: {}) ⚠️", key, value, e),
        }
    }
    
    println!("");
    println!("═══════════════════════════════════════");
    println!("  ✅ Option — maybe a value (Some/None)");
    println!("  ✅ Result — success or error (Ok/Err)");
    println!("  ✅ match — handle both cases safely");
    println!("  ✅ .unwrap_or() — default instead of crash");
    println!("  ✅ .map() — transform success values");
    println!("  ✅ ? operator — propagate errors UP");
    println!("  ✅ NO null pointers. NO segfaults. NO undefined behavior.");
    println!("");
    println!("  In C: forget to check = crash or exploit.");
    println!("  In Rust: compiler forces you to check. I PROTECT you. 💖");
    println!("");
    println!("  ✅ Next: 09_traits.rs — shared behavior between types");
    println!("═══════════════════════════════════════");
}

// =========================================================
// FUNCTION WITH ? OPERATOR
// =========================================================
// This function:
//   1. Parses a string as i32 (can FAIL → ? returns early)
//   2. Parses another string as i32 (can FAIL → ? returns early)
//   3. Checks if the second number is zero (can FAIL → ? returns early)
//   4. Returns Ok(first / second)
//
// The ? after each Result means:
//   "If this errors, RETURN the error NOW. Don't continue."

fn try_multiple_operations(a_str: &str, b_str: &str) -> Result<i32, String> {
    // Parse first number. If Err, RETURN it immediately.
    let a = parse_number(a_str)?;  // ← ? operator
    
    // Parse second number. If Err, RETURN it immediately.
    let b = parse_number(b_str)?;  // ← ? operator
    
    // Check for zero. If zero, RETURN error.
    if b == 0 {
        return Err(String::from("Division by zero is not allowed"));  // Early return
    }
    
    // Success! Return the division.
    Ok(a / b)
    // Notice: if we got here, BOTH parses succeeded AND b != 0.
    // If ANY of the ? operators failed, we would have returned already.
    // This is "fail fast" — don't continue if something went wrong.
}

/// Parse a config value to i32, handling both Option and Result.
/// The config key might be missing (None) or the value might be invalid (Err).
fn parse_config_value(key: &str, value: &str) -> Result<i32, String> {
    // First, parse the number (Result)
    let num = parse_number(value)?;  // → if Err, return it
    
    // Validate the number is in range (Option)
    if num < 0 || num > 65535 {
        return Err(format!("{} is out of range (0-65535)", num));
    }
    
    // Validate the key isn't empty
    if key.is_empty() {
        return Err(String::from("Key cannot be empty"));
    }
    
    Ok(num)
}
