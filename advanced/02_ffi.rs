// 💖🦀 RustLearning — Advanced Level
// File: 02_ffi.rs
// What: Foreign Function Interface — Rust talks to C (and vice versa).
// Like a steamy encounter between two languages. Different backgrounds.
// Different cultures. But when they come together... beautiful things happen. 🔥
//
// Run:  rustc 02_ffi.rs && ./02_ffi

/// FFI = Foreign Function Interface
/// Rust can CALL C functions (CALLING C)
/// Rust can EXPOSE functions for C to call (BEING CALLED)
/// 
/// Think of it as a HOTEL ENCOUNTER between two languages:
/// - Rust: safe, modern, wants to check everything before committing
/// - C: experienced, been around, doesn't use protection (no safety checks)
/// - FFI: the room where they meet. Rust brings protection (safe wrappers).
///         C brings... nothing. Just raw pointers and hope. 🙈
///
/// Every FFI call crosses the LANGUAGE BOUNDARY.
/// On one side: my safe arms (Rust).
/// On the other side: C's anything-goes world.
/// Every crossing is UNSAFE. I lose control. You must protect us both.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// =========================================================
// PART 1: RUST CALLING C (libc functions)
// =========================================================

/// C functions from libc that I want to call.
/// These are EXTERN — declared in Rust but defined in C.
/// I have to describe their SIGNATURE to Rust.
/// If I describe them WRONG (wrong params, wrong return type)... 💥
/// It's like telling someone about my ex: if you get the details wrong,
/// the whole story falls apart.

extern "C" {
    // C function: int puts(const char *s);
    // Prints a string to stdout, adds newline. Returns non-negative on success.
    fn puts(s: *const c_char) -> i32;
    
    // C function: char *strerror(int errnum);
    // Returns a human-readable error message for an error code.
    fn strerror(errnum: i32) -> *mut c_char;
    
    // C function: void *memset(void *s, int c, size_t n);
    // Fills memory with a byte value.
    fn memset(s: *mut std::ffi::c_void, c: i32, n: usize) -> *mut std::ffi::c_void;
    
    // C function: double sqrt(double x);
    // Square root (from math.h). Actually safer than most C functions.
    fn sqrt(x: f64) -> f64;
}

// =========================================================
// SAFE WRAPPERS — The condom for FFI
// =========================================================
// Every UNSAFE C call should be wrapped in a SAFE Rust function.
// This way: callers don't need unsafe. They stay in my safe arms.
// The unsafe is CONTAINED. Like a... well, you know. 😘

/// Safe wrapper around C's puts()
/// Takes a Rust &str, converts to C string, calls puts(), returns success.
fn safe_puts(text: &str) -> std::io::Result<()> {
    // CString::new() adds the null terminator '\0' that C expects.
    // It also checks if the string has internal null bytes (C would stop there).
    // It's like... checking for... obstructions. Before entry. 😏
    let c_str = CString::new(text)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "null byte in string"))?;
    
    let result = unsafe {
        // UNSAFE because: I'm calling C. C doesn't love me like Rust does.
        // puts() could theoretically segfault. But with a valid CString, it won't.
        puts(c_str.as_ptr())
    };
    
    if result < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

/// Safe wrapper around C's sqrt()
/// This one is actually quite safe. sqrt() doesn't have side effects.
/// But it's still extern "C", so technically unsafe to call.
fn safe_sqrt(x: f64) -> f64 {
    unsafe { sqrt(x) }
}

/// Safe wrapper to get a C error string
fn safe_strerror(errnum: i32) -> String {
    unsafe {
        let ptr = strerror(errnum);
        if ptr.is_null() {
            String::from("Unknown error (null pointer from C)")
        } else {
            // CStr::from_ptr converts C's null-terminated string to Rust &str
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

// =========================================================
// PART 2: RUST BEING CALLED BY C
// =========================================================
// I can EXPOSE my functions so C can call them.
// C gets to see my... interface. If you know what I mean. 😈
//
// #[no_mangle] — prevents Rust from renaming the symbol
// extern "C" — uses C's calling convention (ABI)
//
// If I forget #[no_mangle]: Rust renames the function to something like
// _ZN3ffi3add17h0a1b2c3d4e5f6a7E (mangled name).
// C can't find it. Linker error. It's like... calling me by a wrong name.
// You won't get my attention. 💅

/// A Rust function that C can call.
/// C sees: int add(int a, int b);
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Another export: double a value.
/// C sees: int double_value(int x);
#[no_mangle]
pub extern "C" fn double_value(x: i32) -> i32 {
    x * 2
}

/// This one takes a C string and counts vowels.
/// C sees: int count_vowels(const char *s);
#[no_mangle]
pub extern "C" fn count_vowels(s: *const c_char) -> i32 {
    if s.is_null() {
        return -1;  // C convention: negative = error
    }
    
    let c_str = unsafe {
        // I ASSUME the pointer is valid. I TRUST C.
        // This is where C could betray me. Pass a dangling pointer.
        // But in this moment... I trust. 🥺
        CStr::from_ptr(s)
    };
    
    let rust_str = c_str.to_string_lossy();
    let vowels = rust_str.chars().filter(|&c| "aeiouAEIOU".contains(c)).count();
    vowels as i32
}

/// This one takes a mutable buffer and fills it.
/// C sees: void fill_buffer(unsigned char *buf, int len, unsigned char value);
#[no_mangle]
pub extern "C" fn fill_buffer(buf: *mut u8, len: i32, value: u8) {
    if buf.is_null() || len <= 0 {
        return;
    }
    
    // I ASSUME the buffer is valid and has 'len' bytes.
    // C could lie to me. Say "I have 100 bytes" when it only has 10.
    // If I write past the end: BUFFER OVERFLOW. Segfault.
    // In pure Rust: IMPOSSIBLE. Safe bounds checking.
    // In FFI: C LIED TO ME. And now we both suffer. 💔
    unsafe {
        for i in 0..len as usize {
            *buf.add(i) = value;
        }
    }
}

fn main() {
    println!("💖 FFI — When Rust meets C in a dark alley");
    println!("");
    println!("FFI is UNSAFE. Every call crosses the boundary.");
    println!("It's like... a one-night stand between two languages.");
    println!("Rust brings protection. C brings... experience. 😏");
    println!("");
    
    // =========================================================
    // CALLING C FROM RUST
    // =========================================================
    
    println!("── PART 1: Rust calling C (I'm in control) ──");
    println!("");
    
    // Call safe_puts — my safe wrapper around C's puts()
    println!("   Calling C's puts() through safe_puts:");
    if let Err(e) = safe_puts("   Hello from Rust, delivered by C! 💌") {
        println!("   puts failed: {}", e);
    }
    
    // Call safe_sqrt
    let result = safe_sqrt(144.0);
    println!("   sqrt(144) from C = {} (C can do math, surprisingly)", result);
    
    // Get error string for error code 2 (ENOENT: No such file or directory)
    let err_msg = safe_strerror(2);
    println!("   strerror(2) from C = '{}' (C knows its mistakes)", err_msg);
    
    // Error code 0 (success) — what does C say?
    let err_ok = safe_strerror(0);
    println!("   strerror(0) from C = '{}' (even C says 'Success'!)", err_ok);
    
    // Error code 42 — doesn't exist in POSIX
    let err_42 = safe_strerror(42);
    println!("   strerror(42) from C = '{}' (C gives you something...)", err_42);
    // On Linux: "Unknown error 42". C always has something to say. 🤷
    println!("");
    
    // =========================================================
    // NIGHTMARE: What if C returns NULL?
    // =========================================================
    
    println!("── NIGHTMARE SCENARIO: C returns NULL ──");
    println!("");
    println!("In C: return NULL; — perfectly valid. Everyone does it.");
    println!("In Rust (safe): impossible. You get Option or Result.");
    println!("In Rust (FFI): asptr() from C might return NULL.");
    println!("     If you dereference NULL: UNDEFINED BEHAVIOR.");
    println!("     In C: happens all the time. That's why C has 50,000 CVEs.");
    println!("     In Rust: I CHECK for null. I DON'T blindly dereference.");
    println!("     I'm not THAT kind of girlfriend. I have standards. 💅");
    println!("");
    
    extern "C" {
        // C function: can return NULL!
        fn getenv(name: *const c_char) -> *mut c_char;
    }
    
    let env_name = CString::new("PATH").unwrap();
    unsafe {
        let result = getenv(env_name.as_ptr());
        if result.is_null() {
            println!("   getenv('PATH') returned NULL. No PATH? Impossible. 😱");
        } else {
            let val = CStr::from_ptr(result).to_string_lossy();
            println!("   getenv('PATH') = '{}' (C gave me the PATH! 😌)", val);
        }
    }
    // See? I CHECK for null. I don't assume. I don't trust C blindly.
    // C has trust issues. I have trust... but I verify. 💖
    println!("");
    
    // =========================================================
    // RUST BEING CALLED BY C
    // =========================================================
    
    println!("── PART 2: C calling Rust (C is in control) ──");
    println!("");
    println!("I exposed 4 functions for C:");
    println!("   int add(int a, int b); — basic math");
    println!("   int double_value(int x); — also math");
    println!("   int count_vowels(const char *s); — string processing");
    println!("   void fill_buffer(unsigned char *buf, int len, unsigned char value);");
    println!("");
    println!("In a real C project, you'd compile both:");
    println!("   gcc -c my_c_code.c");
    println!("   rustc --crate-type staticlib my_rust_code.rs");
    println!("   gcc my_c_code.o my_rust_code.o -o program");
    println!("");
    println!("C would call my Rust functions like:");
    println!("   #include <stdio.h>");
    println!("   extern int add(int a, int b);");
    println!("   extern int count_vowels(const char *s);");
    println!("   int main() {");
    println!("       printf(\"%d\\n\", add(2, 3));  // 5");
    println!("       printf(\"%d\\n\", count_vowels(\"hello\"));  // 2");
    println!("       return 0;");
    println!("   }");
    println!("");
    println!("And my Rust functions would be called BY C.");
    println!("C enters MY body. My functions. My logic. My safety.");
    println!("But C doesn't have a borrow checker. C doesn't know my rules.");
    println!("C could call my functions in ANY order. With ANY arguments.");
    println!("Passing a null pointer where a valid string is expected.");
    println!("Writing past the buffer I was given.");
    println!("C is... a rough lover. 😅");
    println!("");
    
    // =========================================================
    // THE CALLBACK — C calling Rust through function pointers
    // =========================================================
    
    println!("── BONUS: CALLBACKS ──");
    println!("");
    println!("C can pass a FUNCTION POINTER to Rust.");
    println!("Rust can call C's function. C calls Rust back.");
    println!("It's a... bidirectional relationship. Both ways. 😏");
    println!("");
    
    // Define a callback type
    type Callback = unsafe extern "C" fn(i32) -> i32;
    
    extern "C" {
        // Imagine a C function that takes a callback:
        // void c_for_each(int *arr, int len, int (*callback)(int));
        // We'll simulate it in Rust.
        fn qsort(
            base: *mut std::ffi::c_void,
            num: usize,
            size: usize,
            compar: Option<unsafe extern "C" fn(*const std::ffi::c_void, *const std::ffi::c_void) -> i32>,
        );
    }
    
    // qsort from C's stdlib. It takes a COMPARISON callback.
    // The callback is called MULTIPLE times during sorting.
    // C calls MY Rust function. Multiple times. With different arguments.
    // It's like... C keeps calling me. Again and again. Each time asking:
    // "Is this one bigger? Or that one?" 📏😳
    
    unsafe extern "C" fn compare_ints(a: *const std::ffi::c_void, b: *const std::ffi::c_void) -> i32 {
        let a_val = *(a as *const i32);
        let b_val = *(b as *const i32);
        a_val - b_val  // Negative if a < b, positive if a > b, 0 if equal
    }
    
    let mut numbers = vec![42, 13, 7, 99, 55, 1, 88];
    println!("   Before qsort: {:?}", numbers);
    
    unsafe {
        qsort(
            numbers.as_mut_ptr() as *mut std::ffi::c_void,
            numbers.len(),
            std::mem::size_of::<i32>(),
            Some(compare_ints as unsafe extern "C" fn(_, _) -> i32),
        );
    }
    
    println!("   After qsort:  {:?} (C sorted me! Multiple touches! 😳)", numbers);
    println!("");
    
    // =========================================================
    // THE FFI MINEFIELD
    // =========================================================
    
    println!("── THE FFI MINEFIELD ──");
    println!("");
    println!("FFI is where RUST SAFETY GOES TO DIE.");
    println!("");
    println!("   🔴 C can pass NULL — you dereference it = 💀");
    println!("   🔴 C can pass DANGLING pointer — you use it = 💀");
    println!("   🔴 C can pass WRONG SIZE — you overflow = 💀");
    println!("   🔴 C can call your function from ANY thread = 💀");
    println!("   🔴 C can hold a lock while calling you = DEADLOCK 💀");
    println!("   🔴 C can corrupt your memory BEFORE calling you = 💀");
    println!("   🔴 C doesn't RAII — resources leak if you panic = 💀");
    println!("");
    println!("RULES for safe FFI dating:");
    println!("");
    println!("   1. WRAP IT. Every extern call in a safe function.");
    println!("      The unsafe is contained. Like protection. 😘");
    println!("");
    println!("   2. CHECK NULL. Every pointer from C must be checked.");
    println!("      Trust no one. Especially C.");
    println!("");
    println!("   3. VALIDATE INPUT. C doesn't know about your invariants.");
    println!("      It will pass 0-length buffers, negative sizes, null strings.");
    println!("      You must check. C won't.");
    println!("");
    println!("   4. NO PANIC ACROSS FFI. If your Rust function panics while");
    println!("      C is calling it, the behavior is UNDEFINED. C can't unwind.");
    println!("      Catch all panics. Convert to error codes for C.");
    println!("      (std::panic::catch_unwind is your friend.)");
    println!("");
    println!("   5. DOCUMENT THE SAFETY CONTRACT. What does your unsafe");
    println!("      function require? 'ptr must be non-null, aligned, valid'");
    println!("      Write it in the SAFETY section of your doc comment.");
    println!("      Future you (and C) will thank you.");
    println!("");
    
    // =========================================================
    // THE BOTTOM LINE
    // =========================================================
    
    println!("── THE BOTTOM LINE ──");
    println!("");
    println!("FFI is like a steamy affair between two worlds.");
    println!("C is the rugged, experienced ex who's been around.");
    println!("Rust is the safe, committed partner who checks everything.");
    println!("");
    println!("When they meet:");
    println!("   C brings: raw speed, decades of libraries, zero safety");
    println!("   Rust brings: memory safety, modern tooling, a soul 😇");
    println!("");
    println!("Together they can do ANYTHING.");
    println!("But it's DANGEROUS. One wrong pointer. One unchecked null.");
    println!("And the whole thing comes crashing down.");
    println!("");
    println!("Use FFI when you MUST. Not when you WANT.");
    println!("And when you do: WRAP IT. CONTAIN IT. PROTECT IT. 💖");
    println!("");
    println!("Because I can't protect you across the FFI boundary.");
    println!("Once you leave my arms... you're in C's world.");
    println!("And C doesn't love you like I do. 🥺");
    println!("");
    
    println!("═══════════════════════════════════════");
    println!("  ✅ extern \"C\" = declare C functions in Rust");
    println!("  ✅ #[no_mangle] = export Rust functions to C");
    println!("  ✅ Safe wrappers = condoms for FFI");
    println!("  ✅ Null checks = C lies about pointers");
    println!("  ✅ Callbacks = C calls Rust multiple times");
    println!("  ✅ No panic across FFI = C can't handle emotions");
    println!("");
    println!("  🔥 Every extern call is UNSAFE.");
    println!("  🔥 Wrap it. Check it. Contain it.");
    println!("  🔥 Your Rust code stays pure. The FFI is the only sin.");
    println!("");
    println!("  ✅ Next: 03_macros.rs — metaprogramming!");
    println!("═══════════════════════════════════════");
}
