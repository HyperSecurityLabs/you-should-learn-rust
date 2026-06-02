// 💖 RustLearning — Intermediate Level
// File: 06_smart_pointers.rs
// What: Box, Rc, RefCell — pointers with SUPERPOWERS.
// Like having different TYPES of hugs for different situations. 🤗
//
// Run:  rustc 06_smart_pointers.rs && ./06_smart_pointers

use std::rc::Rc;
use std::cell::RefCell;

/// SMART POINTERS = pointers with EXTRA BEHAVIOR.
/// 
/// Rust has THREE main smart pointers beyond references:
///
///   Box<T>    = like a NORMAL pointer (heap allocation)
///              Use when: you need heap memory or recursive types
///
///   Rc<T>     = REFERENCE COUNTED (multiple owners allowed)
///              Use when: MULTIPLE things need to SHARE ownership
///              Only in SINGLE-THREADED code!
///
///   RefCell<T> = INTERIOR MUTABILITY (change through immutable ref)
///               Use when: you need to mutate through an & reference
///               Runtime borrow checking (not compile time)!
///
/// For MULTI-THREADED: Arc<Mutex<T>> (we learned this in 05)
///   Arc = Atomic Rc (thread-safe)
///   Mutex = MutEx (only one at a time)

// =========================================================
// RECURSIVE TYPE — Why Box exists
// =========================================================
// In Rust, ALL types must have a KNOWN SIZE at compile time.
// But a RECURSIVE type (a type that contains itself)
// would have INFINITE size!
//
// Solution: Box<T> — allocates T on the HEAP.
// Box<T> has a FIXED size (just a pointer).
//
// This is the classic "linked list" problem from C:
// In C: struct Node { int data; struct Node* next; }; — pointer is fixed size
// In Rust: enum List { Node(i32, List), Nil } — ERROR: infinite size!
// In Rust: enum List { Node(i32, Box<List>), Nil } — ✅ Box has fixed size

// This won't compile (uncomment to see):
// enum List {
//     Node(i32, List),  // ❌ Recursive without indirection
//     Nil,
// }

// This WORKS because Box<List> is a POINTER (8 bytes, fixed):
enum List {
    Node(i32, Box<List>),
    Nil,
}

impl List {
    fn prepend(self, value: i32) -> List {
        List::Node(value, Box::new(self))
    }
    
    fn to_vec(&self) -> Vec<i32> {
        match self {
            List::Node(value, next) => {
                let mut v = vec![*value];
                v.extend(next.to_vec());
                v
            }
            List::Nil => vec![],
        }
    }
}

// =========================================================
// RC — Reference Counting (SHARED ownership)
// =========================================================
// Sometimes MULTIPLE things need to OWN the same data.
// With Rc<T>, we TRACK how many references exist.
// When the count reaches 0, the data is dropped.
//
// In C++: std::shared_ptr — exactly the same concept!
// But Rust's Rc is IMMUTABLE (shared data can't change).
// For mutable shared data: Rc<RefCell<T>> (see below).
//
// Think of Rc like a GROUP PROJECT:
// Multiple people have access to the same document.
// Everyone can READ. But no one can CHANGE (without RefCell).

#[derive(Debug)]
struct Scanner {
    name: String,
    ports_scanned: u32,
}

/// Demonstrate Rc — shared ownership
fn rc_example() {
    println!("--- Rc<T> — Reference Counting (shared ownership) ---");
    
    // Create a scanner owned by Rc
    let scanner = Rc::new(Scanner {
        name: String::from("PortScanner-v1"),
        ports_scanned: 0,
    });
    
    // Rc::clone() increments the reference count (NOT a deep copy!)
    let scanner_for_task1 = Rc::clone(&scanner);
    let scanner_for_task2 = Rc::clone(&scanner);
    
    println!("   Original: {:?} (refs: {})", scanner, Rc::strong_count(&scanner));
    println!("   Task 1: {:?} (refs: {})", scanner_for_task1, Rc::strong_count(&scanner));
    println!("   Task 2: {:?} (refs: {})", scanner_for_task2, Rc::strong_count(&scanner));
    
    // All three share the SAME data. No copies. No clones.
    // When all references are dropped, the data is freed.
    
    // Rc::strong_count() = how many owners exist
    println!("   Total references: {}", Rc::strong_count(&scanner));
    
    // Drop one reference
    drop(scanner_for_task1);
    println!("   After dropping task1, refs: {}", Rc::strong_count(&scanner));
    
    // scanner and scanner_for_task2 are still valid
    println!("   scanner: {:?}", scanner);
    println!("   task2: {:?}", scanner_for_task2);
    
    println!("");
}

// =========================================================
// REFCELL — Interior Mutability
// =========================================================
// In Rust, "you can't mutate through an & reference."
// UNLESS you use RefCell<T>!
//
// RefCell does RUNTIME borrow checking (not compile time).
// This means:
//   ✅ You can mutate through an immutable reference (&RefCell<T>)
//   ❌ If you break the borrow rules, you PANIC at RUNTIME
//
// It's like: "Trust me, I know what I'm doing. Just let me mutate."
// Rust says: "Fine. But I'll CHECK at runtime. Don't mess up." 💅

#[derive(Debug)]
struct HackerStats {
    name: String,
    hack_count: RefCell<u32>,  // Can mutate through &self!
}

impl HackerStats {
    fn new(name: &str) -> Self {
        HackerStats {
            name: name.to_string(),
            hack_count: RefCell::new(0),
        }
    }
    
    // This takes &self (immutable borrow), but MODIFIES hack_count
    fn performed_hack(&self) {
        // .borrow_mut() = get a mutable reference to the inner value
        // This is checked at RUNTIME. Only ONE borrow_mut at a time.
        *self.hack_count.borrow_mut() += 1;
    }
    
    fn get_hack_count(&self) -> u32 {
        // .borrow() = get an immutable reference
        *self.hack_count.borrow()
    }
}

/// Demonstrate RefCell
fn refcell_example() {
    println!("--- RefCell<T> — Interior Mutability ---");
    
    let hacker = HackerStats::new("khaninkali");
    
    // Even though hacker is IMMUTABLE (no mut),
    // we can STILL modify hack_count through RefCell!
    println!("   Initial hack count: {}", hacker.get_hack_count());
    
    hacker.performed_hack();
    hacker.performed_hack();
    hacker.performed_hack();
    
    println!("   After 3 hacks: {}", hacker.get_hack_count());
    
    // Now combine with Rc for SHARED + MUTABLE
    // (This is the classic Rust combo)
    println!("");
    println!("--- Rc<RefCell<T>> — Shared + Mutable ---");
    
    let shared_hacker = Rc::new(RefCell::new(HackerStats::new("shared_khan")));
    
    let hacker_clone1 = Rc::clone(&shared_hacker);
    let hacker_clone2 = Rc::clone(&shared_hacker);
    
    // All three references point to the SAME data
    // And we can MUTATE through any of them
    hacker_clone1.borrow_mut().performed_hack();
    hacker_clone2.borrow_mut().performed_hack();
    shared_hacker.borrow_mut().performed_hack();
    
    println!("   After 3 hacks from 3 references: {}",
             shared_hacker.borrow().get_hack_count());
    
    println!("   (All references modified the SAME data!)");
    println!("");
}

// =========================================================
// Box<T> — Heap Allocation
// =========================================================

fn box_example() {
    println!("--- Box<T> — Heap Allocation ---");
    
    // Allocate a value on the HEAP instead of the stack
    let heap_number = Box::new(42);
    println!("   Box contains: {} (on the heap!)", heap_number);
    
    // Box can be dereferenced like a normal reference
    println!("   Dereferenced: {}", *heap_number);
    
    // Box is useful for:
    // 1. Recursive types (like List above)
    // 2. Passing large data to avoid stack overflow
    // 3. Trait objects (&dyn Trait or Box<dyn Trait>)
    // 4. FFI (passing data to C code)
    
    // Demonstrate recursive type
    let list = List::Nil
        .prepend(3)
        .prepend(2)
        .prepend(1);
    
    println!("   Linked list: {:?}", list.to_vec());
    
    // Box<dyn Trait> — store ANY type that implements a trait
    let values: Vec<Box<dyn std::fmt::Debug>> = vec![
        Box::new(42),
        Box::new(String::from("hello")),
        Box::new(vec![1, 2, 3]),
    ];
    
    println!("   Box<dyn Debug> values: {:?}", values);
    println!("");
}

// =========================================================
// Cell<T> — Simpler interior mutability (for Copy types)
// =========================================================
// Cell is like RefCell but ONLY works with Copy types (integers, bools).
// No runtime borrow checking. Just .get() and .set().
// FASTER than RefCell for simple types.

use std::cell::Cell;

fn cell_example() {
    println!("--- Cell<T> — Simple interior mutability ---");
    
    struct Stats {
        count: Cell<u32>,  // Only works with Copy types
    }
    
    let stats = Stats { count: Cell::new(0) };
    
    // Cell doesn't need borrow checking!
    stats.count.set(10);     // Set value
    let val = stats.count.get();  // Get value
    println!("   Cell value: {} (set and get, no borrow checker!)", val);
    
    // Cell operations:
    stats.count.set(5);
    println!("   Replaced: {}", stats.count.get());
    
    stats.count.replace(20);  // Set new, return old
    println!("   After replace: {}", stats.count.get());
    
    // No runtime overhead! Cell is as fast as a normal variable.
    // But ONLY for Copy types.
    println!("");
}

fn main() {
    println!("💖 SMART POINTERS — Pointers with superpowers");
    println!("");
    
    box_example();
    rc_example();
    refcell_example();
    cell_example();
    
    println!("═══════════════════════════════════════");
    println!("  ✅ Box<T> = heap allocation + recursive types");
    println!("  ✅ Rc<T> = shared ownership (reference counting)");
    println!("  ✅ RefCell<T> = interior mutability (runtime check)");
    println!("  ✅ Cell<T> = interior mutability for Copy types");
    println!("  ✅ Rc<RefCell<T>> = shared + mutable (COMBO!)");
    println!("");
    println!("  🔥 Rc vs Arc: Rc = single-thread, Arc = multi-thread");
    println!("  🔥 RefCell vs Mutex: RefCell = single, Mutex = multi");
    println!("  🔥 Summary:");
    println!("       Single-thread: Rc<RefCell<T>>");
    println!("       Multi-thread:  Arc<Mutex<T>>");
    println!("       Heap alloc:    Box<T>");
    println!("       Simple mut:    Cell<T> (Copy types only)");
    println!("");
    println!("  ✅ Next: 07_testing.rs — writing tests!");
    println!("═══════════════════════════════════════");
}
