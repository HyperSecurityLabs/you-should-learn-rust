// 💖 RustLearning — Intermediate Level
// File: 04_error_handling.rs
// What: Custom error types, ? operator, Result aliases, error chaining.
// Like having a well-organized toolbox for when things go wrong. 🛠️
//
// Run:  rustc 04_error_handling.rs && ./04_error_handling

use std::fmt;

// =========================================================
// CUSTOM ERROR TYPE — Define YOUR OWN errors
// =========================================================
// Instead of returning String everywhere, define proper error types.
// This lets the CALLER know EXACTLY what can go wrong.
//
// In C: errno + global variable. Terrible.
// In C++: exceptions. Unknown what can throw.
// In Rust: Result<Ok, Err> with explicit error types. PERFECT. 💖

/// My custom error type for a port scanner.
/// Each variant describes a DIFFERENT failure mode.
#[derive(Debug)]
enum ScanError {
    Timeout { host: String, port: u16 },           // Named fields
    ConnectionRefused(String),                      // String error
    PermissionDenied,                               // No data, just a flag
    InvalidPort(u16),                               // Just the port number
    DnsLookupFailed(String),                        // Hostname failed
    IoError(std::io::Error),                        // Wrap an IO error
}

// =========================================================
// Implementing Display — So errors can be PRINTED
// =========================================================
// Display is like toString() in Java or __str__ in Python.
// It's how errors look when you print them or use .to_string().

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanError::Timeout { host, port } => {
                write!(f, "Timeout scanning {}:{} (no response)", host, port)
            }
            ScanError::ConnectionRefused(host) => {
                write!(f, "Connection refused by {}", host)
            }
            ScanError::PermissionDenied => {
                write!(f, "Permission denied (need root for raw sockets)")
            }
            ScanError::InvalidPort(port) => {
                write!(f, "Invalid port number: {} (0-65535)", port)
            }
            ScanError::DnsLookupFailed(host) => {
                write!(f, "DNS lookup failed for '{}'", host)
            }
            ScanError::IoError(e) => {
                write!(f, "IO error: {}", e)
            }
        }
    }
}

// Implement std::error::Error — makes it a PROPER error type
// This allows it to be used with ? and other error handling tools.
impl std::error::Error for ScanError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ScanError::IoError(e) => Some(e),
            _ => None,  // Other variants don't have inner errors
        }
    }
}

// =========================================================
// From IMPL — Convert other errors INTO ours
// =========================================================
// This allows the ? operator to AUTOMATICALLY convert errors.
// If a function returns std::io::Error, and we use ?,
// Rust will call ScanError::from(io_error) to convert it.

impl From<std::io::Error> for ScanError {
    fn from(error: std::io::Error) -> Self {
        ScanError::IoError(error)
    }
}

impl From<String> for ScanError {
    fn from(error: String) -> Self {
        ScanError::ConnectionRefused(error)
    }
}

// =========================================================
// RESULT ALIAS — Make your life easier
// =========================================================
// Instead of writing Result<T, ScanError> everywhere,
// create a TYPE ALIAS. Saves typing. Looks cleaner.

type ScanResult<T> = Result<T, ScanError>;

// =========================================================
// FUNCTIONS USING CUSTOM ERROR
// =========================================================

/// Scan a single port. Returns Ok(true) if open, Err if error.
fn scan_port(host: &str, port: u16) -> ScanResult<bool> {
    // Validate port
    if port == 0 || port > 65535 {
        return Err(ScanError::InvalidPort(port));
    }
    
    // Validate host
    if host.is_empty() {
        return Err(ScanError::DnsLookupFailed("Empty hostname".to_string()));
    }
    
    // Simulate different failure modes based on port
    match port {
        22 => Ok(true),           // SSH is always open
        80 => Ok(true),           // HTTP is always open
        443 => Ok(true),          // HTTPS is always open
        444 => Err(ScanError::PermissionDenied),
        445 => Err(ScanError::ConnectionRefused(host.to_string())),
        446 => Err(ScanError::Timeout { host: host.to_string(), port }),
        _ => Ok(false),           // Other ports are closed
    }
}

/// Scan MULTIPLE ports. Returns list of open ports.
/// Uses ? to PROPAGATE errors from scan_port.
fn scan_ports(host: &str, ports: &[u16]) -> ScanResult<Vec<u16>> {
    let mut open_ports = Vec::new();
    
    for &port in ports {
        // If scan_port returns Err, the ? will RETURN it from THIS function
        if scan_port(host, port)? {
            open_ports.push(port);
        }
    }
    
    Ok(open_ports)
}

/// Scan a port and return a DESCRIPTION of the result.
/// Demonstrates error HANDLING (not propagation).
fn describe_port(host: &str, port: u16) -> String {
    match scan_port(host, port) {
        Ok(true) => format!("Port {} is OPEN ✅ (service available)", port),
        Ok(false) => format!("Port {} is CLOSED 🔒 (no response)", port),
        Err(ScanError::Timeout { .. }) => format!("Port {} — TIMEOUT (host may be filtering) ⏰", port),
        Err(ScanError::ConnectionRefused(_)) => format!("Port {} — REFUSED (service not running) 🚫", port),
        Err(ScanError::PermissionDenied) => format!("Port {} — PERMISSION DENIED (need root) 👑", port),
        Err(ScanError::InvalidPort(p)) => format!("Invalid port: {} 🤷", p),
        Err(ScanError::DnsLookupFailed(h)) => format!("DNS lookup failed for '{}' 🌐", h),
        Err(ScanError::IoError(e)) => format!("IO error: {} 💥", e),
    }
}

fn main() {
    println!("💖 CUSTOM ERRORS — Professional error handling");
    println!("");
    
    // =========================================================
    // USING CUSTOM ERROR TYPES
    // =========================================================
    
    println!("--- Scanning ports with custom errors ---");
    
    // scan_port returns ScanResult<bool>
    match scan_port("localhost", 80) {
        Ok(true) => println!("   Port 80: OPEN ✅"),
        Ok(false) => println!("   Port 80: CLOSED 🔒"),
        Err(e) => println!("   Error: {}", e),
    }
    
    match scan_port("localhost", 444) {
        Ok(true) => println!("   Port 444: OPEN"),
        Ok(false) => println!("   Port 444: CLOSED"),
        Err(e) => println!("   Port 444 error: {}", e),
    }
    
    // Invalid port
    match scan_port("localhost", 99999) {
        Ok(_) => println!("   Impossible: port out of range reported open"),
        Err(e) => println!("   Invalid port error: {}", e),
    }
    
    println!("");
    
    // =========================================================
    // PROPAGATING ERRORS WITH ?
    // =========================================================
    // scan_ports calls scan_port with ? for each port.
    // If ANY one fails, the WHOLE function returns the error.
    
    println!("--- Propagating errors with ? ---");
    
    let ports_to_scan = vec![80, 443, 22, 444, 8080];
    
    match scan_ports("localhost", &ports_to_scan) {
        Ok(open) => println!("   Open ports: {:?} ✅", open),
        Err(e) => println!("   Scan failed at: {}", e),
    }
    // It fails at port 444 (PermissionDenied).
    // The ? operator returns the error IMMEDIATELY.
    // Ports after 444 are NOT scanned.
    
    println!("");
    
    // =========================================================
    // ERROR HANDLING — Not propagation
    // =========================================================
    // describe_port HANDLES each error case with a custom message.
    // It doesn't propagate. It converts everything to a String.
    
    println!("--- Error handling (not propagation) ---");
    
    let test_ports = vec![80, 444, 445, 446, 99999];
    for &port in &test_ports {
        println!("   {}", describe_port("localhost", port));
    }
    
    println!("");
    
    // =========================================================
    // WRAPPING EXTERNAL ERRORS
    // =========================================================
    // Sometimes you call a function that returns std::io::Error.
    // You want to WRAP it in YOUR error type.
    // This is what the From impl does.
    
    fn read_config(path: &str) -> ScanResult<String> {
        // This returns std::io::Error normally.
        // But we have From<io::Error> for ScanError.
        // So ? CONVERTS it automatically!
        let contents = std::fs::read_to_string(path)?;
        Ok(contents)
    }
    
    println!("--- Wrapping IO errors ---");
    match read_config("/etc/nonexistent/config.toml") {
        Ok(data) => println!("   Config: {}", data),
        Err(e) => println!("   Error reading config: {}", e),
    }
    
    match read_config("/etc/hostname") {
        Ok(data) => println!("   Hostname config: {}...", &data[..data.len().min(20)]),
        Err(e) => println!("   Error: {}", e),
    }
    
    println!("");
    
    // =========================================================
    // MULTIPLE ERROR TYPES — Using Box<dyn Error>
    // =========================================================
    // When a function can fail in MULTIPLE ways from different libraries,
    // you can use Box<dyn std::error::Error> to wrap ANY error.
    //
    // This is useful for PROTOTYPES. For production, make custom types.
    
    fn fallible_function(should_fail: bool) -> Result<String, Box<dyn std::error::Error>> {
        if should_fail {
            // Return a STRING error (converted to Box<dyn Error>)
            Err("Something went wrong!".into())
        } else {
            Ok("Success!".to_string())
        }
    }
    
    println!("--- Box<dyn Error> for flexibility ---");
    match fallible_function(true) {
        Ok(msg) => println!("   {}", msg),
        Err(e) => println!("   Error: {}", e),
    }
    match fallible_function(false) {
        Ok(msg) => println!("   {}", msg),
        Err(e) => println!("   Error: {}", e),
    }
    
    println!("");
    
    // =========================================================
    // anyhow — Popular error crate (conceptual)
    // =========================================================
    // In real projects, use the `anyhow` crate:
    //
    // use anyhow::{Result, Context};
    //
    // fn do_stuff() -> anyhow::Result<()> {
    //     let data = std::fs::read_to_string("file.txt")
    //         .context("Failed to read file")?;
    //     Ok(())
    // }
    //
    // anyhow::Result = Result<T, anyhow::Error>
    // .context() = add context to the error
    // Great for APPLICATIONS (not libraries).
    //
    // For LIBRARIES, use `thiserror` crate:
    //
    // #[derive(thiserror::Error, Debug)]
    // enum MyError {
    //     #[error("IO error: {0}")]
    //     Io(#[from] std::io::Error),
    //     #[error("Parse error: {0}")]
    //     Parse(#[from] std::num::ParseIntError),
    // }
    
    println!("═══════════════════════════════════════");
    println!("  ✅ Custom error types (enum)");
    println!("  ✅ Display impl for user-friendly messages");
    println!("  ✅ Error trait for integration with ?");
    println!("  ✅ From impl for automatic conversions");
    println!("  ✅ Type alias (ScanResult<T>)");
    println!("  ✅ ? operator propagates errors UP");
    println!("  ✅ Box<dyn Error> for flexible prototyping");
    println!("");
    println!("  🔥 In C: errno + if checks (forgotten 50% of time)");
    println!("  🔥 In Rust: Result + ? (ENFORCED by compiler)");
    println!("  🔥 Safer. Cleaner. More maintainable.");
    println!("");
    println!("  ✅ Next: 05_concurrency.rs — threads and channels");
    println!("═══════════════════════════════════════");
}
