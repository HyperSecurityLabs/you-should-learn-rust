// 💖 RustLearning — Offensive: Process
// File: 01_process_list.rs
// What: List running processes (safe version — reads /proc).
// Like `ps aux` but in Rust. EDUCATIONAL — understand your system.
//
// Run:  rustc 01_process_list.rs && ./01_process_list
// No special permissions needed (reads /proc which is world-readable).

/// On Linux, the /proc filesystem contains info about EVERY running process.
/// /proc/[PID]/status — process state
/// /proc/[PID]/cmdline — command that started the process
/// /proc/[PID]/comm — process name
/// /proc/[PID]/maps — memory mappings
/// /proc/[PID]/fd/ — open file descriptors
///
/// In C: you'd use readdir() + open() + read() — manual buffer management.
/// In Rust: std::fs + String — safe, no buffer overflows.

use std::fs;
use std::path::Path;

/// A single process entry
#[derive(Debug)]
struct Process {
    pid: u32,
    name: String,
    state: String,
    memory_kb: u64,
}

/// Read a value from /proc/[pid]/status
fn read_status_field(pid: u32, field: &str) -> Option<String> {
    let path = format!("/proc/{}/status", pid);
    let content = fs::read_to_string(&path).ok()?;
    
    for line in content.lines() {
        if line.starts_with(field) {
            // Format: "Name:\tvalue"
            let value = line.split(':').nth(1)?.trim().to_string();
            return Some(value);
        }
    }
    None
}

/// Read the command line of a process
fn read_cmdline(pid: u32) -> Option<String> {
    let path = format!("/proc/{}/cmdline", pid);
    // cmdline is null-byte separated. Replace \0 with spaces.
    let content = fs::read_to_string(&path).ok()?;
    let cmdline = content.replace('\0', " ");
    if cmdline.trim().is_empty() {
        None
    } else {
        Some(cmdline.trim().to_string())
    }
}

/// List all processes in /proc
fn list_processes() -> Vec<Process> {
    let mut processes = Vec::new();
    
    // Read /proc directory
    let proc_dir = match fs::read_dir("/proc") {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Cannot read /proc: {}", e);
            return processes;
        }
    };
    
    for entry in proc_dir.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy().to_string();
        
        // Entries that are numbers = process IDs
        let pid: u32 = match name_str.parse() {
            Ok(pid) => pid,
            Err(_) => continue,  // Not a PID entry (like "cpuinfo", "meminfo")
        };
        
        // Read process info
        let name = read_status_field(pid, "Name").unwrap_or_default();
        let state = read_status_field(pid, "State").unwrap_or_default();
        let memory = read_status_field(pid, "VmRSS")
            .and_then(|s| s.trim_end_matches(" kB").parse().ok())
            .unwrap_or(0);
        
        processes.push(Process {
            pid,
            name,
            state,
            memory_kb: memory,
        });
    }
    
    // Sort by PID
    processes.sort_by_key(|p| p.pid);
    processes
}

fn main() {
    println!("💖 Process List — See what's running on your system");
    println!("");
    println!("NOTE: This reads /proc, the Linux process filesystem.");
    println!("In C: you'd manage buffers manually. In Rust: strings are safe.");
    println!("");
    
    let processes = list_processes();
    
    println!("Total processes: {}", processes.len());
    println!("");
    println!("{:<8} {:<20} {:<10} {:<10} {}", "PID", "Name", "State", "RSS(KB)", "Details");
    println!("{}", "-".repeat(80));
    
    for p in &processes {
        // Only show first 50 entries for display
        if p.pid > 100 && p.pid < 200 {
            continue;  // Filter kernel threads for cleaner output
        }
        
        let state_icon = match p.state.chars().next() {
            Some('R') => "▶️ Running",
            Some('S') => "💤 Sleeping",
            Some('D') => "⏳ Disk Sleep",
            Some('Z') => "💀 Zombie",
            Some('T') => "⏸️ Stopped",
            Some('X') => "☠️ Dead",
            _ => "❓ Unknown",
        };
        
        println!("{:<8} {:<20} {:<10} {:<10} {}",
                 p.pid, p.name, state_icon, format!("{}K", p.memory_kb), 
                 if p.pid <= 10 { "[kernel]" } else { "" });
    }
    
    println!("");
    
    // =========================================================
    // PROCESS ANALYSIS — What to look for
    // =========================================================
    
    println!("── ANALYSIS: What to look for in process lists ──");
    println!("");
    println!("🔍 Suspicious indicators:");
    println!("   🔴 Processes with random/typo'd names (scvhost.exe vs svchost.exe)");
    println!("   🔴 Processes running from /tmp or /dev/shm");
    println!("   🔴 Processes without a name (hidden)");
    println!("   🔴 Processes with suspicious memory regions (rwx)");
    println!("   🔴 Unknown processes with open network connections");
    println!("   🔴 Processes masquerading as kernel threads (but with PID > 100)");
    println!("");
    println!("🔍 In C, you'd iterate /proc with opendir/readdir:");
    println!("   char path[256];");
    println!("   snprintf(path, sizeof(path), \"/proc/%d/status\", pid);");
    println!("   // One buffer overflow and you're done.");
    println!("");
    println!("🔍 In Rust:");
    println!("   let path = format!(\"/proc/{}/status\", pid);");
    println!("   let content = fs::read_to_string(&path)?;");
    println!("   // No buffer issues. Strings grow as needed. Safe. 💖");
    println!("");
    
    // =========================================================
    // MEMORY MAPS — Advanced process analysis
    // =========================================================
    
    println!("── BONUS: Reading process memory maps ──");
    println!("");
    println!("/proc/[PID]/maps shows memory regions:");
    println!("");
    println!("   55a1e2e00000-55a1e2f00000 r-xp 00000000 08:01 123456 /usr/bin/foo");
    println!("   7f8a1c000000-7f8a1c001000 rwxp 00000000 00:00 0 [heap]");
    println!("   7f8a1c500000-7f8a1c600000 rwxp 00000000 00:00 0 [stack]");
    println!("");
    println!("   Address range: where in memory this region is");
    println!("   Permissions: r=read, w=write, x=execute, p=private, s=shared");
    println!("   Mapping: file backing or anonymous");
    println!("");
    println!("   🔴 rwx = READ + WRITE + EXECUTE = VERY SUSPICIOUS");
    println!("   🔴 Most legitimate code is r-x (no write)");
    println!("   🔴 rwx = shellcode or JIT spray");
    println!("");
    println!("In Rust, reading maps is as safe as reading a text file:");
    println!("   let maps = fs::read_to_string(format!(\"/proc/{}/maps\", pid))?;");
    println!("   // No buffer management. No off-by-one. Safe. 🦀");
    println!("");
    
    println!("═══════════════════════════════════════");
    println!("  ✅ /proc = process information filesystem");
    println!("  ✅ Read PIDs, names, states, memory usage");
    println!("  ✅ Process analysis — find anomalies");
    println!("  ✅ Memory maps — detect rwx regions");
    println!("");
    println!("  🔥 In C: buffer overflows everywhere");
    println!("  🔥 In Rust: fs::read_to_string() = safe I/O");
    println!("");
    println!("  ✅ Next: 02_memory_reader.rs — read process memory");
    println!("═══════════════════════════════════════");
}
