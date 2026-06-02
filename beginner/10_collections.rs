// 💖 RustLearning — Beginner Level
// File: 10_collections.rs
// What: Advanced data structures — HashMap, HashSet, BTreeMap, VecDeque.
// Like having DIFFERENT tools for different storage needs. 🛠️
//
// Run:  rustc 10_collections.rs && ./10_collections

use std::collections::{HashMap, HashSet, BTreeMap, VecDeque, BinaryHeap};

fn main() {
    println!("💖 COLLECTIONS — Different tools for different jobs");
    println!("");
    
    // =========================================================
    // HashMap — Key-Value Store (unordered)
    // =========================================================
    // Like a DICTIONARY. O(1) average lookup.
    // In C++: std::unordered_map
    // In Python: dict
    // In Rust: HashMap (uses SipHash, DOS-resistant)
    
    println!("--- HashMap<K, V> — Quick lookups ---");
    
    let mut port_services = HashMap::new();
    port_services.insert(80, "HTTP");
    port_services.insert(443, "HTTPS");
    port_services.insert(22, "SSH");
    port_services.insert(3306, "MySQL");
    
    // Lookup
    if let Some(service) = port_services.get(&80) {
        println!("Port 80: {}", service);
    }
    
    // Check existence
    println!("Port 443 exists? {}", port_services.contains_key(&443));
    println!("Port 9999 exists? {}", port_services.contains_key(&9999));
    
    // Iterate
    println!("Known ports:");
    for (port, service) in &port_services {
        println!("   {} → {}", port, service);
    }
    
    // .entry() — insert if missing, update if present
    port_services.entry(8080).or_insert("Proxy");
    port_services.entry(80).or_insert("Already HTTP");  // Won't change (80 exists)
    
    println!("After entry():");
    for (port, service) in &port_services {
        println!("   {} → {}", port, service);
    }
    
    // Remove
    port_services.remove(&3306);
    println!("After removing 3306: {:?}", port_services);
    
    println!("");
    
    // =========================================================
    // HashMap with collect() — From iterator to HashMap
    // =========================================================
    
    let data = vec![("user1", 10), ("user2", 20), ("user3", 30)];
    let scores: HashMap<&str, i32> = data.into_iter().collect();
    println!("HashMap from iterator: {:?}", scores);
    println!("");
    
    // =========================================================
    // HashSet — Unique items (no duplicates)
    // =========================================================
    // Like a SET. Each value appears ONCE.
    // In C++: std::unordered_set
    // In Python: set
    // Perfect for: "have I seen this before?" checks
    
    println!("--- HashSet<T> — Unique values only ---");
    
    let mut unique_ports = HashSet::new();
    unique_ports.insert(80);
    unique_ports.insert(443);
    unique_ports.insert(80);     // Duplicate! Won't be added
    unique_ports.insert(22);
    unique_ports.insert(443);    // Duplicate! Ignored
    
    println!("Unique ports: {:?} (notice 80 and 443 appear ONCE)", unique_ports);
    println!("Contains 80? {}", unique_ports.contains(&80));
    println!("Contains 9999? {}", unique_ports.contains(&9999));
    
    // .len() = number of unique items
    println!("Number of unique ports: {}", unique_ports.len());
    
    // Set operations: union, intersection, difference, symmetric_difference
    let set_a: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set_b: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
    
    println!("");
    println!("Set A: {:?}", set_a);
    println!("Set B: {:?}", set_b);
    
    // Union — everything in A OR B
    println!("Union: {:?}", set_a.union(&set_b).collect::<Vec<_>>());
    
    // Intersection — everything in A AND B
    println!("Intersection: {:?}", set_a.intersection(&set_b).collect::<Vec<_>>());
    
    // Difference — in A but NOT in B
    println!("A - B: {:?}", set_a.difference(&set_b).collect::<Vec<_>>());
    
    println!("");
    
    // =========================================================
    // BTreeMap — Sorted Key-Value Store
    // =========================================================
    // Like HashMap but SORTED by key. O(log n) operations.
    // In C++: std::map (red-black tree)
    // Use when: you need ORDERED iteration
    
    println!("--- BTreeMap<K, V> — Sorted key-value pairs ---");
    
    let mut sorted_ports = BTreeMap::new();
    sorted_ports.insert(443, "HTTPS");
    sorted_ports.insert(80, "HTTP");
    sorted_ports.insert(22, "SSH");
    sorted_ports.insert(8080, "Proxy");
    
    // Iteration is in KEY ORDER (ascending)
    println!("Sorted ports (by key):");
    for (port, service) in &sorted_ports {
        println!("   {} → {}", port, service);
    }
    
    // Range queries — "ports between 80 and 500"
    println!("Ports in range 80..500:");
    for (port, service) in sorted_ports.range(80..500) {
        println!("   {} → {}", port, service);
    }
    
    // First/last entries
    println!("First port: {:?}", sorted_ports.first_key_value());
    println!("Last port: {:?}", sorted_ports.last_key_value());
    
    println!("");
    
    // =========================================================
    // VecDeque — Double-ended queue
    // =========================================================
    // Like Vec but can push/pop from BOTH ends efficiently.
    // In C++: std::deque
    // Perfect for: queue (FIFO) and stack (LIFO) operations
    
    println!("--- VecDeque<T> — Push/pop from both ends ---");
    
    let mut deque: VecDeque<i32> = VecDeque::new();
    
    // Add to both ends
    deque.push_back(10);    // [10]
    deque.push_front(5);    // [5, 10]
    deque.push_back(20);    // [5, 10, 20]
    deque.push_front(1);    // [1, 5, 10, 20]
    
    println!("Deque: {:?}", deque);
    println!("Front: {:?}", deque.front());    // Some(&1)
    println!("Back: {:?}", deque.back());       // Some(&20)
    
    // Pop from both ends
    let front = deque.pop_front();  // Removes 1
    let back = deque.pop_back();    // Removes 20
    println!("Popped front: {:?}, back: {:?}", front, back);
    println!("Deque after pops: {:?}", deque);
    
    // Use as QUEUE (FIFO): push_back + pop_front
    // Use as STACK (LIFO): push_back + pop_back
    println!("");
    
    // =========================================================
    // BinaryHeap — Priority Queue
    // =========================================================
    // Always gives you the LARGEST element (MAX-HEAP).
    // In C++: std::priority_queue
    // Perfect for: "what's the most important thing?"
    
    println!("--- BinaryHeap<T> — Priority queue (max-heap) ---");
    
    let mut heap = BinaryHeap::new();
    heap.push(3);
    heap.push(7);
    heap.push(1);
    heap.push(9);
    heap.push(5);
    
    println!("Heap (unordered): {:?}", heap);
    
    // .pop() returns the LARGEST element
    println!("Popping in priority order:");
    while let Some(value) = heap.pop() {
        println!("   {} (highest remaining)", value);
    }
    
    // For MIN-HEAP: use std::cmp::Reverse
    use std::cmp::Reverse;
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(3));
    min_heap.push(Reverse(7));
    min_heap.push(Reverse(1));
    
    println!("Min-heap (using Reverse):");
    while let Some(Reverse(val)) = min_heap.pop() {
        println!("   {} (smallest remaining)", val);
    }
    
    println!("");
    
    // =========================================================
    // LinkedList — Doubly-linked list (RARELY needed)
    // =========================================================
    // In Rust, LinkedList is almost NEVER the right choice.
    // Vec is faster for almost everything due to cache locality.
    // Use LinkedList only when: you're splicing lists frequently.
    
    // LinkedList exists in std::collections::LinkedList
    // But 99% of the time: use Vec instead.
    
    println!("═══════════════════════════════════════");
    println!("  ✅ HashMap — unordered key-value (fast)");
    println!("  ✅ HashSet — unique values (no dupes)");
    println!("  ✅ BTreeMap — sorted key-value");
    println!("  ✅ VecDeque — push/pop both ends");
    println!("  ✅ BinaryHeap — priority queue");
    println!("");
    println!("  🔥 TIP: Vec is faster than LinkedList in 99% of cases");
    println!("  🔥 TIP: HashMap is faster but BTreeMap has order");
    println!("  🔥 TIP: HashSet is great for 'seen it before' checks");
    println!("");
    println!("  🎉 CONGRATULATIONS! You finished the BEGINNER section!");
    println!("  Next: intermediate/ — lifetimes, closures, concurrency");
    println!("═══════════════════════════════════════");
}
