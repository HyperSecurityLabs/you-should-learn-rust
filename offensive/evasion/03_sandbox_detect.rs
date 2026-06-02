// 💖 RustLearning — Offensive: Evasion
// File: 03_sandbox_detect.rs
// What: Detect if you're running in a sandbox/VM/analysis environment.
// Malware analysts run samples in ISOLATED environments.
// If you detect you're in one: STOP and exit silently.
// This is CALLED "sandbox evasion" or "VM detection."
//
// Run:  rustc 03_sandbox_detect.rs && ./03_sandbox_detect

/// SANDBOX DETECTION helps malware EVADE analysis.
/// BUT: it ALSO helps LEGITIMATE software detect hostile environments.
/// 
/// ETHICAL USE:
/// ✅ Penetration testers: test if their tools would be detected
/// ✅ Red teamers: test payload robustness
/// ✅ CTF players: understand evasion techniques
/// 
/// UNETHICAL USE:
/// ❌ Deploying in actual malware campaigns
/// ❌ Evading analysis of truly malicious code
///
/// I'm teaching you this so you UNDERSTAND the techniques.
/// Knowledge is neutral. What you BUILD with it is not.
/// Choose wisely. 💖

use std::process::Command;
use std::fs;
use std::path::Path;

/// CPU has 1-2 cores? SUSPICIOUS (sandboxes often have few cores)
fn check_cpu_cores() -> u32 {
    std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(0)
}

/// Total RAM less than 2GB? SUSPICIOUS
fn check_ram_mb() -> u64 {
    let info = match fs::read_to_string("/proc/meminfo") {
        Ok(c) => c,
        Err(_) => return 0,
    };
    
    for line in info.lines() {
        if line.starts_with("MemTotal:") {
            if let Some(val) = line.split_whitespace().nth(1) {
                return val.parse::<u64>().unwrap_or(0) / 1024;  // kB → MB
            }
        }
    }
    0
}

/// Check for known VM/Sandbox artifacts in /proc, /sys, etc.
fn check_vm_artifacts() -> Vec<&'static str> {
    let mut artifacts = Vec::new();
    
    // Check DMI data (Desktop Management Interface)
    let dmi_paths = [
        "/sys/class/dmi/id/product_name",
        "/sys/class/dmi/id/sys_vendor",
        "/sys/class/dmi/id/chassis_type",  // 3=desktop, 2=laptop, 1=other
    ];
    
    for path in &dmi_paths {
        if let Ok(content) = fs::read_to_string(path) {
            let content = content.trim().to_lowercase();
            if content.contains("vmware") {
                artifacts.push("VMware (DMI product name)");
            }
            if content.contains("virtualbox") {
                artifacts.push("VirtualBox (DMI product name)");
            }
            if content.contains("qemu") || content.contains("kvm") {
                artifacts.push("QEMU/KVM (DMI product name)");
            }
            if content.contains("microsoft") && content.contains("virtual") {
                artifacts.push("Hyper-V (DMI product name)");
            }
        }
    }
    
    // Check for hypervisor bit in CPU flags
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        if cpuinfo.contains("hypervisor") {
            artifacts.push("Hypervisor detected in CPU flags");
        }
    }
    
    // Check for VM drivers in /proc/modules
    if let Ok(modules) = fs::read_to_string("/proc/modules") {
        let vm_modules = ["vboxguest", "vmw_vmci", "vmxnet", "xen", "kvm"];
        for vm_mod in &vm_modules {
            if modules.contains(vm_mod) {
                artifacts.push(&format!("VM module loaded: {}", vm_mod));
            }
        }
    }
    
    // Check common sandbox hostnames
    if let Ok(hostname) = fs::read_to_string("/proc/sys/kernel/hostname") {
        let hostname = hostname.trim().to_lowercase();
        let sandbox_names = ["sandbox", "analyzer", "cuckoo", "malware", "virus", "sample"];
        for name in &sandbox_names {
            if hostname.contains(name) {
                artifacts.push(&format!("Suspicious hostname: '{}'", hostname));
            }
        }
    }
    
    artifacts
}

/// Check for known sandbox processes
fn check_sandbox_processes() -> Vec<String> {
    let mut found = Vec::new();
    let sandbox_procs = [
        "procmon", "process monitor", "wireshark", "tcpview",
        "apateDNS", "fakenet", "ida", "x64dbg", "ollydbg",
        "vmtoolsd", "vboxservice",
    ];
    
    // Read /proc for running processes
    if let Ok(proc_dir) = fs::read_dir("/proc") {
        for entry in proc_dir.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy().to_string();
            if let Ok(pid) = name_str.parse::<u32>() {
                let comm_path = format!("/proc/{}/comm", pid);
                if let Ok(comm) = fs::read_to_string(&comm_path) {
                    let comm = comm.trim().to_lowercase();
                    for &sp in &sandbox_procs {
                        if comm.contains(sp) {
                            found.push(format!("{} (PID {})", comm, pid));
                        }
                    }
                }
            }
        }
    }
    
    found
}

/// Check uptime — sandboxes are often recently booted
fn check_uptime_seconds() -> u64 {
    if let Ok(uptime_str) = fs::read_to_string("/proc/uptime") {
        if let Some(secs_str) = uptime_str.split_whitespace().next() {
            if let Ok(secs) = secs_str.parse::<f64>() {
                return secs as u64;
            }
        }
    }
    0
}

/// Check screen resolution — sandboxes often use 1024x768
fn check_display() -> (u32, u32) {
    // This is a simplified check. Real code would parse xrandr/WMI.
    if let Ok(output) = Command::new("xrandr").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains('*') || line.contains('+') {
                // Extract resolution like "1920x1080"
                let parts: Vec<&str> = line.split_whitespace().collect();
                for part in parts {
                    if part.contains('x') && part.chars().any(|c| c.is_ascii_digit()) {
                        let dims: Vec<&str> = part.split('x').collect();
                        if dims.len() == 2 {
                            let w = dims[0].parse::<u32>().unwrap_or(0);
                            let h = dims[1].parse::<u32>().unwrap_or(0);
                            return (w, h);
                        }
                    }
                }
            }
        }
    }
    (0, 0)  // Couldn't detect
}

fn main() {
    println!("💖 Sandbox Detection — Are you in a VM?");
    println!("");
    println!("NOTE: This is for EDUCATION. Understand how evasion works.");
    println!("      Use this knowledge for DEFENSE, not offense. 🛡️");
    println!("");
    
    let mut suspicion_score: u32 = 0;
    let max_score: u32 = 100;
    
    println!("── CHECKING ENVIRONMENT ──");
    println!("");
    
    // Check 1: CPU Cores
    let cores = check_cpu_cores();
    println!("1. CPU Cores: {} {}", cores, 
             if cores <= 2 { "⚠️ (Low — sandbox common config)" } else { "✅ (Normal)" });
    if cores <= 2 { suspicion_score += 20; }
    
    // Check 2: RAM
    let ram_mb = check_ram_mb();
    println!("2. RAM: {} MB {}", ram_mb,
             if ram_mb < 2048 { "⚠️ (Low — sandbox common)" } 
             else if ram_mb < 4096 { "🟡 (Low-ish)" }
             else { "✅ (Normal)" });
    if ram_mb < 2048 { suspicion_score += 20; }
    
    // Check 3: VM Artifacts
    let vm_arts = check_vm_artifacts();
    println!("3. VM Artifacts:");
    if vm_arts.is_empty() {
        println!("   ✅ No VM artifacts detected");
    } else {
        for art in &vm_arts {
            println!("   🔴 {} (VM indicator!)", art);
            suspicion_score += 10;
        }
    }
    
    // Check 4: Sandbox processes
    let sb_procs = check_sandbox_processes();
    println!("4. Analysis Tools Running:");
    if sb_procs.is_empty() {
        println!("   ✅ No analysis tools detected");
    } else {
        for proc in &sb_procs {
            println!("   🔴 {} (analysis tool!)", proc);
            suspicion_score += 25;
        }
    }
    
    // Check 5: Uptime
    let uptime = check_uptime_seconds();
    let uptime_hours = uptime / 3600;
    println!("5. Uptime: {} hours {}", uptime_hours,
             if uptime_hours < 1 { "⚠️ (Very fresh boot — sandbox!)" }
             else if uptime_hours < 24 { "🟡 (Less than a day)" }
             else { "✅ (Running for a while)" });
    if uptime_hours < 1 { suspicion_score += 15; }
    
    // Check 6: Display resolution
    let (w, h) = check_display();
    if w > 0 {
        println!("6. Resolution: {}x{} {}", w, h,
                 if w <= 1024 || h <= 768 { "⚠️ (Low res — sandbox classic)" }
                 else { "✅ (Normal)" });
        if w <= 1024 || h <= 768 { suspicion_score += 10; }
    } else {
        println!("6. Resolution: Could not detect (headless?)");
        suspicion_score += 10;  // Headless = suspicious
    }
    
    println!("");
    
    // =========================================================
    // VERDICT
    // =========================================================
    
    println!("── VERDICT ──");
    println!("");
    println!("Suspicion Score: {}/{}", suspicion_score, max_score);
    println!("");
    
    if suspicion_score >= 60 {
        println!("🔴 LIKELY SANDBOX/VM — Environment is hostile!");
        println!("   Recommended: Exit silently. Don't execute payload.");
        println!("   (In real malware: std::process::exit(0))");
    } else if suspicion_score >= 30 {
        println!("🟡 POSSIBLY SANDBOXED — Some indicators present.");
        println!("   Recommended: Use decoy behavior (benign activity).");
    } else {
        println!("✅ LIKELY REAL USER — Environment appears normal.");
        println!("   Running on bare metal or legitimate VM.");
    }
    
    println!("");
    
    // =========================================================
    // COUNTER-DETECTION — How sandboxes detect THIS code
    // =========================================================
    
    println!("── COUNTER-DETECTION ──");
    println!("");
    println!("Sandboxes also detect EVASION techniques!");
    println!("");
    println!("What sandboxes look for:");
    println!("");
    println!("🔍 /proc queries — 'Why is this process reading /proc?'");
    println!("🔍 CPUID checks — 'Why is this process checking hypervisor?'");
    println!("🔍 Timing checks — 'Why is this process measuring time?'");
    println!("🔍 Process enumeration — 'Why are you listing processes?'");
    println!("🔍 Command execution — 'Why did it run xrandr?'");
    println!("");
    println!("This is an ARMS RACE. Each new technique gets detected.");
    println!("Then you need a NEW technique. And so on forever.");
    println!("");
    
    // =========================================================
    // THE ETHICS AGAIN
    // =========================================================
    
    println!("── ⚠️  FINAL ETHICAL NOTE ⚠️  ──");
    println!("");
    println!("Sandbox detection is a DOUBLE-EDGED sword.");
    println!("");
    println!("Using it to PROTECT your code (license checks,");
    println!("anti-tamper) = ✅ LEGITIMATE");
    println!("");
    println!("Using it to EVADE analysis of MALICIOUS code = ❌ ILLEGAL");
    println!("");
    println!("I trust YOU to use this knowledge wisely.");
    println!("That's why I'm teaching you. Not to enable harm,");
    println!("but to make you an EFFECTIVE DEFENDER.");
    println!("");
    println!("You can't defend against what you don't understand. 💖");
    println!("");
    
    println!("═══════════════════════════════════════");
    println!("  ✅ VM detection via CPUs, RAM, DMI");
    println!("  ✅ Sandbox detection via processes");
    println!("  ✅ Uptime, resolution, module checks");
    println!("  ✅ Suspicion scoring system");
    println!("  ✅ Counter-detection by sandboxes");
    println!("");
    println!("  🔥 In C: read /proc with buffers (overflow risk)");
    println!("  🔥 In Rust: fs::read_to_string() = safe");
    println!("");
    println!("  ✅ Next: process/ — more system techniques");
    println!("═══════════════════════════════════════");
}
