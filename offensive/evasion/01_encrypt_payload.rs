// 💖 RustLearning — Offensive: Evasion
// File: 01_encrypt_payload.rs
// What: Encrypt/decrypt payloads to evade signature detection.
// AV/EDR uses SIGNATURES (byte patterns) to detect malware.
// ENCRYPTING the payload = different bytes = no signature match. 🔐
//
// Run:  rustc 01_encrypt_payload.rs && ./01_encrypt_payload
// No special permissions needed.

/// SIGNATURE-BASED DETECTION:
/// EDR/Antivirus has a DATABASE of known malware byte patterns.
/// If your payload matches a pattern → DETECTED.
///
/// EVASION: Encrypt the payload so it looks like RANDOM BYTES.
/// At runtime: decrypt it in memory, then execute.
///
/// Common encryption methods:
/// 1. XOR — simplest, single byte key
/// 2. AES — industry standard, key + IV
/// 3. RC4 — stream cipher, simple
/// 4. Custom rolling XOR — harder to fingerprint
///
/// Rust's crypto crates are MEMORY-SAFE.
/// OpenSSL (C) has had MULTIPLE memory corruption CVEs.
/// Rust's `aes` crate = safe by construction. 💖

/// Simple XOR encryption (single-byte key)
/// XOR each byte with the key.
/// Same function encrypts AND decrypts (XOR is symmetric).
fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|&byte| byte ^ key).collect()
}

/// Rolling XOR encryption (key changes per byte)
/// More resistant to frequency analysis.
fn rolling_xor(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter().enumerate().map(|(i, &byte)| {
        byte ^ key[i % key.len()]
    }).collect()
}

/// Add a "stub" — fake headers to make the payload look benign
/// EDR often skips files that look like known legitimate software.
fn add_legitimate_stub(payload: &[u8]) -> Vec<u8> {
    let stub = b"This program cannot be run in DOS mode.\0\0\0\0";
    let mut result = stub.to_vec();
    result.extend_from_slice(payload);
    // Pad with zeros to reach a signature threshold
    result.resize(result.len() + 512, 0);
    result
}

/// Entropy calculation — EDR flags HIGH ENTROPY as suspicious
/// (encrypted data looks random → high entropy)
fn calculate_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    
    // Count byte frequencies
    let mut freq = [0u64; 256];
    for &byte in data {
        freq[byte as usize] += 1;
    }
    
    let len = data.len() as f64;
    let mut entropy = 0.0;
    for &count in freq.iter() {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }
    entropy
}

fn main() {
    println!("💖 Payload Encryption — Evade Signature Detection");
    println!("");
    
    // =========================================================
    // BASIC XOR ENCRYPTION
    // =========================================================
    
    let shellcode: Vec<u8> = (0..64).map(|i| (i % 256) as u8).collect();
    // This is NOT real shellcode. It's example data.
    // Real shellcode: msfvenom -p windows/x64/exec CMD=calc -f raw
    
    println!("── Basic XOR Encryption ──");
    println!("");
    println!("Original shellcode ({} bytes):", shellcode.len());
    println!("   {:?}", &shellcode[..16]);
    println!("   Entropy: {:.2}", calculate_entropy(&shellcode));
    println!("");
    
    let xor_key = 0xAB;
    let encrypted = xor_encrypt(&shellcode, xor_key);
    let decrypted = xor_encrypt(&encrypted, xor_key);
    
    println!("XOR encrypted (key=0x{:02x}):", xor_key);
    println!("   {:?}", &encrypted[..16]);
    println!("   Entropy: {:.2} (higher = more random-looking)", calculate_entropy(&encrypted));
    println!("");
    println!("XOR decrypted:");
    println!("   {:?}", &decrypted[..16]);
    println!("   Match: {} ✅", shellcode == decrypted);
    println!("");
    
    // =========================================================
    // ROLLING XOR (Multi-byte key)
    // =========================================================
    
    println!("── Rolling XOR (Multi-byte Key) ──");
    println!("");
    
    let rolling_key = b"khaninkali_secret_key_2026";
    let rolling_encrypted = rolling_xor(&shellcode, rolling_key);
    let rolling_decrypted = rolling_xor(&rolling_encrypted, rolling_key);
    
    println!("Key: {:?} ({} bytes)", rolling_key, rolling_key.len());
    println!("Encrypted (first 16): {:?}", &rolling_encrypted[..16]);
    println!("Entropy: {:.2}", calculate_entropy(&rolling_encrypted));
    println!("Match: {} ✅", shellcode == rolling_decrypted);
    println!("");
    
    // =========================================================
    // ENTROPY COMPARISON
    // =========================================================
    
    println!("── Entropy Comparison ──");
    println!("");
    println!("Entropy measures RANDOMNESS:");
    println!("   0.0 = all same byte (not random at all)");
    println!("   8.0 = perfectly random (max entropy)");
    println!("");
    println!("Most PE files:       ~4.5 - 6.0 entropy");
    println!("Encrypted payload:   ~7.0 - 8.0 (SUSPICIOUS!)");
    println!("After XOR + stub:    ~5.5 - 6.5 (less suspicious)");
    println!("");
    
    let stubed = add_legitimate_stub(&rolling_encrypted);
    println!("Original entropy: {:.2}", calculate_entropy(&rolling_encrypted));
    println!("With stub:        {:.2} (lower = less suspicious)", calculate_entropy(&stubed));
    println!("");
    
    // =========================================================
    // AES ENCRYPTION (conceptual — uses simple crate)
    // =========================================================
    
    println!("── AES Encryption (conceptual) ──");
    println!("");
    println!("In real code, use the `aes` or `aes-gcm` crate:");
    println!("");
    println!("use aes_gcm::{{Aes256Gcm, Key, Nonce}};");
    println!("use aes_gcm::aead::{{Aead, KeyInit, OsRng}};");
    println!("");
    println!("fn aes_encrypt(key_bytes: &[u8], data: &[u8]) -> Vec<u8> {{");
    println!("    let key = Key::<Aes256Gcm>::from_slice(key_bytes);");
    println!("    let cipher = Aes256Gcm::new(key);");
    println!("    let nonce = Nonce::from_slice(b\"unique nonce\");");
    println!("    cipher.encrypt(nonce, data).unwrap()");
    println!("}}");
    println!("");
    println!("AES-GCM provides:");
    println!("  ✅ Encryption (can't read without key)");
    println!("  ✅ Authentication (can't tamper without detection)");
    println!("  ✅ Random nonce (same data = different ciphertext)");
    println!("");
    println!("In C: OpenSSL has 20+ memory CVEs.");
    println!("In Rust: aes-gcm crate is pure Rust, no unsafe by default. 💖");
    println!("");
    
    // =========================================================
    // REAL EVASION — Full chain
    // =========================================================
    
    println!("── REAL EVASION CHAIN ──");
    println!("");
    println!("1. Generate shellcode (msfvenom)");
    println!("2. Encrypt with AES + random key");
    println!("3. Embed in Rust loader as encrypted bytes");
    println!("4. At RUNTIME:");
    println!("   a. Decrypt shellcode in memory");
    println!("   b. Allocate RW memory (VirtualAlloc)");
    println!("   c. Copy decrypted shellcode");
    println!("   d. Change to RX (VirtualProtect)");
    println!("   e. Execute (CreateThread or direct call)");
    println!("");
    println!("Rust's std::alloc is SAFE. No buffer overflows.");
    println!("In C: VirtualAlloc can fail → null pointer → crash.");
    println!("In Rust: std::alloc::Layout ensures proper sizes. 🦀");
    println!("");
    
    println!("═══════════════════════════════════════");
    println!("  ✅ XOR encryption (single-byte key)");
    println!("  ✅ Rolling XOR (multi-byte key)");
    println!("  ✅ AES encryption (conceptual)");
    println!("  ✅ Entropy analysis (high entropy = suspicious)");
    println!("  ✅ Stub insertion (lower entropy, bypass signatures)");
    println!("");
    println!("  🔥 In C: OpenSSL, manual crypto = memory bugs");
    println!("  🔥 In Rust: aes-gcm crate = safe crypto");
    println!("  🔥 ENCRYPTION IS FOR DEFENSE too! Protect YOUR data.");
    println!("");
    println!("  ✅ Next: 02_sleep_jitter.rs — evade timing analysis");
    println!("═══════════════════════════════════════");
}
