// 💖 RustLearning — Intermediate Level
// File: 03_modules.rs
// What: Organizing code into files and modules.
// Like organizing your closet — everything in its place. 👗
//
// This file demonstrates MODULES.
// Run:  rustc 03_modules.rs && ./03_modules

// =========================================================
// MODULE SYSTEM — How Rust organizes code
// =========================================================
//
// Rust uses MODULES to organize code.
// Think of it like FOLDERS for your code:
//
//   lib.rs (crate root)
//   ├── network/
//   │   ├── mod.rs      (network module)
//   │   ├── tcp.rs      (network::tcp submodule)
//   │   └── udp.rs      (network::udp submodule)
//   ├── crypto/
//   │   ├── mod.rs      (crypto module)
//   │   ├── aes.rs      (crypto::aes submodule)
//   │   └── xor.rs      (crypto::xor submodule)
//
// In a SINGLE file, you can define modules inline:
//   mod network { ... }  // This defines a module WITHIN this file

// =========================================================
// INLINE MODULE — network
// =========================================================
mod network {
    // Items inside a module are PRIVATE by default.
    // Use "pub" to make them PUBLIC (accessible outside the module).
    
    pub fn scan(ip: &str, port: u16) -> bool {
        println!("   Scanning {}:{}...", ip, port);
        // Simulate a port scan
        port % 2 == 0  // Even ports are "open" 😉
    }
    
    // Private function — only usable INSIDE this module
    fn port_to_service(port: u16) -> &'static str {
        match port {
            80 => "HTTP",
            443 => "HTTPS",
            22 => "SSH",
            _ => "Unknown",
        }
    }
    
    // Public function that uses a private function
    pub fn describe_port(port: u16) {
        let service = port_to_service(port);  // Private call — OK inside module
        println!("   Port {} → {}", port, service);
    }
    
    // SUBMODULE inside this module
    pub mod http {
        pub fn get(url: &str) -> String {
            format!("GET {} → 200 OK", url)
        }
        
        pub fn post(url: &str, data: &str) -> String {
            format!("POST {} → 201 Created (data: {})", url, data)
        }
    }
}

// =========================================================
// SECOND MODULE — crypto
// =========================================================
mod crypto {
    // "pub use" = re-export. Makes inner items available at crypto::level
    pub use self::aes::encrypt as aes_encrypt;
    
    pub fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
        data.iter().map(|&byte| byte ^ key).collect()
    }
    
    mod aes {
        // Private module, but functions are pub inside it
        pub fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
            // Placeholder for AES encryption
            println!("   AES encryption with {} bytes key", key.len());
            data.to_vec()  // Not real encryption (this is a demo)
        }
        
        fn decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
            println!("   AES decryption");
            data.to_vec()
        }
    }
    
    // "pub use" to re-export AES encrypt
    // pub use self::aes::encrypt as aes_encrypt;
    // (Moved above for clarity)
}

// =========================================================
// THIRD MODULE — utils (with FILE separation concept)
// =========================================================
// If this were a real project, this would be in utils/mod.rs
// But for this single-file example, it's inline.

mod utils {
    // NESTED modules
    pub mod hex {
        pub fn encode(data: &[u8]) -> String {
            data.iter().map(|b| format!("{:02x}", b)).collect()
        }
        
        pub fn decode(hex_str: &str) -> Result<Vec<u8>, String> {
            if hex_str.len() % 2 != 0 {
                return Err("Invalid hex length".to_string());
            }
            (0..hex_str.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex_str[i..i+2], 16)
                    .map_err(|e| format!("Hex decode error: {}", e)))
                .collect()
        }
    }
    
    pub mod base64 {
        // Simplified base64-like encoding
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        
        pub fn encode(data: &[u8]) -> String {
            // This is NOT real base64 (just a demo!)
            data.iter().map(|&b| CHARS[(b % 64) as usize] as char).collect()
        }
    }
}

fn main() {
    println!("💖 MODULES — Organizing code like a boss");
    println!("");
    
    // =========================================================
    // USING MODULES
    // =========================================================
    
    // Access items with :: path separator
    println!("--- network module ---");
    let is_open = network::scan("192.168.1.1", 80);
    println!("   Port 80 open? {}", is_open);
    
    network::describe_port(22);
    network::describe_port(443);
    
    // Submodule
    let response = network::http::get("https://example.com");
    println!("   HTTP response: {}", response);
    
    let response = network::http::post("https://example.com/api", "{\"key\":\"value\"}");
    println!("   HTTP response: {}", response);
    
    println!("");
    println!("--- crypto module ---");
    
    let data = b"Hello, khaninkali!";
    let key = 0xAB;
    let encrypted = crypto::xor_encrypt(data, key);
    let decrypted: Vec<u8> = encrypted.iter().map(|&b| b ^ key).collect();
    
    println!("   Original: {:?}", String::from_utf8_lossy(data));
    println!("   XOR encrypted: {:?}", encrypted);
    println!("   XOR decrypted: {:?}", String::from_utf8_lossy(&decrypted));
    
    // AES via re-export (pub use)
    let aes_key = b"0123456789abcdef";
    let aes_encrypted = crypto::aes_encrypt(data, aes_key);
    println!("   AES encrypted: {:?}", aes_encrypted);
    
    println!("");
    println!("--- utils module ---");
    
    let hex_encoded = utils::hex::encode(data);
    println!("   Hex encoded: {}", hex_encoded);
    
    let hex_decoded = utils::hex::decode(&hex_encoded).unwrap();
    println!("   Hex decoded: {:?}", String::from_utf8_lossy(&hex_decoded));
    
    let b64_encoded = utils::base64::encode(data);
    println!("   Base64-like encoded: {}", b64_encoded);
    
    println!("");
    
    // =========================================================
    // USE KEYWORD — Bringing paths into scope
    // =========================================================
    // "use" makes module paths shorter.
    // Instead of: network::http::get(...)
    // You can:    use network::http::get; then call get(...)
    
    use network::http::get as http_get;
    // Now I can call http_get() instead of network::http::get()
    
    let resp = http_get("https://rust-lang.org");
    println!("Using 'use' keyword: {}", resp);
    
    // You can also use nested paths:
    // use network::{http, scan};
    // use network::http::{get, post};
    
    println!("");
    
    // =========================================================
    // EXTERNAL CRATES — Adding dependencies
    // =========================================================
    // In a real project, you'd add this to Cargo.toml:
    //
    // [dependencies]
    // serde = { version = "1.0", features = ["derive"] }
    // reqwest = "0.11"
    // tokio = { version = "1.0", features = ["full"] }
    //
    // Then in your code:
    // use serde::{Serialize, Deserialize};
    // use reqwest;
    //
    // For this single-file example, we can't show external crates.
    // But the CONCEPT is the same: use crate_name::module::item;
    
    println!("═══════════════════════════════════════");
    println!("  ✅ mod = define a module");
    println!("  ✅ pub = make items accessible outside module");
    println!("  ✅ use = bring paths into scope");
    println!("  ✅ pub use = re-export items");
    println!("  ✅ mod { ... } = inline module");
    println!("  ✅ In real projects: split into FILES");
    println!("     network/mod.rs, crypto/aes.rs, etc.");
    println!("");
    println!("  🔥 In C/C++: #include or modules (C++20)");
    println!("  🔥 In Python: import module");
    println!("  🔥 In Rust: mod + use — cleaner than both");
    println!("");
    println!("  ✅ Next: 04_error_handling.rs — custom errors");
    println!("═══════════════════════════════════════");
}
