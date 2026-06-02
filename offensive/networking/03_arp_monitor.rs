// 💖 RustLearning — Offensive: Networking
// File: 03_arp_monitor.rs
// What: ARP spoofing DETECTOR — the DEFENSIVE tool.
// Monitor ARP traffic and alert when someone is lying about MACs.
// PROTECT your network. Don't ATTACK it. 💖
//
// NOTE: TEMPLATE — needs pnet crate in Cargo project
// EDUCATIONAL USE ONLY — defense, not offense.

/// ARP = Address Resolution Protocol
/// "Who has IP 192.168.1.1? Tell MAC 00:11:22:33:44:55."
///
/// ARP SPOOFING (MITM attack):
/// 1. Attacker sends fake ARP: "I am 192.168.1.1 (the router)"
/// 2. Target believes attacker → sends traffic to attacker
/// 3. Attacker reads/modifies traffic, forwards to real router
///
/// This file DETECTS ARP spoofing. It's DEFENSIVE.
/// We monitor ARP traffic and alert on ANOMALIES.

use std::collections::HashMap;

/// Represents a single ARP observation
#[derive(Debug, Clone)]
struct ArpEntry {
    ip: String,
    mac: String,
    count: u32,
}

/// ARP Spoofing Detector
/// Tracks IP→MAC mappings and detects CHANGES.
/// If an IP suddenly has a NEW MAC, that's SUSPICIOUS.
struct ArpMonitor {
    /// Known IP→MAC mappings
    known: HashMap<String, String>,
    /// Alert log
    alerts: Vec<String>,
}

impl ArpMonitor {
    fn new() -> Self {
        ArpMonitor {
            known: HashMap::new(),
            alerts: Vec::new(),
        }
    }
    
    /// Process an ARP packet observation
    /// Returns true if it's a normal update, false if suspicious
    fn observe(&mut self, ip: &str, mac: &str) -> bool {
        // Check if we've seen this IP before
        match self.known.get(ip) {
            None => {
                // First time seeing this IP→MAC mapping
                // This is NORMAL (new device joined network)
                self.known.insert(ip.to_string(), mac.to_string());
                println!("   ℹ️  NEW: {} → {} (new device on network)", ip, mac);
                true
            }
            Some(existing_mac) if existing_mac == mac => {
                // Same mapping as before — NORMAL
                // (device periodically re-announces itself)
                println!("   ✅ OK: {} → {} (confirmed)", ip, mac);
                true
            }
            Some(existing_mac) => {
                // DIFFERENT MAC than before!
                // THIS IS THE ALERT CONDITION!
                // Either: device changed NIC (normal if replaced)
                // Or: ARP SPOOFING ATTACK!
                let alert = format!(
                    "🔴 ALERT: {} changed MAC! Was: {} → Now: {} (ARP spoofing?)",
                    ip, existing_mac, mac
                );
                println!("   {}", alert);
                self.alerts.push(alert);
                
                // Update the mapping (might be legit NIC replacement)
                self.known.insert(ip.to_string(), mac.to_string());
                false
            }
        }
    }
    
    /// Get all alerts
    fn get_alerts(&self) -> &[String] {
        &self.alerts
    }
    
    /// Dump all known mappings
    fn dump_table(&self) {
        println!("");
        println!("   ARP Table ({} entries):", self.known.len());
        println!("   {:<20} {:<20}", "IP", "MAC");
        println!("   {}", "-".repeat(40));
        let mut entries: Vec<_> = self.known.iter().collect();
        entries.sort_by_key(|e| e.0.clone());
        for (ip, mac) in entries {
            println!("   {:<20} {:<20}", ip, mac);
        }
    }
}

fn main() {
    println!("💖 ARP Monitor — Detect ARP spoofing attacks");
    println!("");
    println!("NOTE: Template for pnet project.");
    println!("In real code: capture ARP packets from network interface.");
    println!("In this demo: simulate ARP traffic to show detection.");
    println!("");
    
    // Simulate ARP traffic on a network
    let mut monitor = ArpMonitor::new();
    
    println!("── SIMULATING NORMAL ARP TRAFFIC ──");
    println!("");
    
    // Normal ARP announcements (devices joining network)
    monitor.observe("192.168.1.1", "00:11:22:33:44:01");  // Router
    monitor.observe("192.168.1.100", "AA:BB:CC:DD:EE:01"); // Desktop
    monitor.observe("192.168.1.101", "AA:BB:CC:DD:EE:02"); // Laptop
    monitor.observe("192.168.1.102", "AA:BB:CC:DD:EE:03"); // Phone
    monitor.observe("192.168.1.50", "11:22:33:44:55:66");   // Printer
    
    // Normal re-announcements (same IP, same MAC — no change)
    println!("");
    println!("── RE-ANNOUNCEMENTS (normal) ──");
    println!("");
    monitor.observe("192.168.1.1", "00:11:22:33:44:01");   // Same — OK
    monitor.observe("192.168.1.100", "AA:BB:CC:DD:EE:01"); // Same — OK
    
    // ARP SPOOFING ATTACK!
    println!("");
    println!("── 🚨 ARP SPOOFING ATTACK DETECTED! 🚨 ──");
    println!("");
    
    // Attacker claims to be the router (192.168.1.1)
    // With a DIFFERENT MAC address!
    monitor.observe("192.168.1.1", "DE:AD:BE:EF:CA:FE");
    
    // Attacker claims to be the desktop
    monitor.observe("192.168.1.100", "DE:AD:BE:EF:CA:FE");
    
    // Normal traffic continues while attacker intercepts
    println!("");
    monitor.observe("192.168.1.101", "AA:BB:CC:DD:EE:02"); // Laptop still OK
    monitor.observe("192.168.1.102", "AA:BB:CC:DD:EE:03"); // Phone still OK
    
    // Another spoofed entry
    monitor.observe("192.168.1.50", "DE:AD:BE:EF:CA:FE");  // Printer spoofed too
    
    // Show alerts
    println!("");
    println!("── ALERTS SUMMARY ──");
    println!("");
    
    let alerts = monitor.get_alerts();
    if alerts.is_empty() {
        println!("   ✅ No ARP spoofing detected. Network is clean.");
    } else {
        println!("   🔴 {} ARP spoofing alert(s) detected!", alerts.len());
        for alert in alerts {
            println!("   ⚠️  {}", alert);
        }
        println!("");
        println!("   💡 The attacker (DE:AD:BE:EF:CA:FE) is intercepting");
        println!("   💡 traffic meant for the router and other devices.");
        println!("   💡 This is a CLASSIC MITM attack.");
        println!("   💡 DEFENSE: Use static ARP entries or switch to IPv6.");
    }
    
    // Show final ARP table
    monitor.dump_table();
    
    println!("");
    println!("── REAL pnet ARP CAPTURE CODE ──");
    println!("");
    println!("use pnet::packet::{{arp::ArpPacket, ethernet::EthernetPacket, Packet}};");
    println!("use pnet::datalink::Channel::Ethernet;");
    println!("");
    println!("fn capture_arp(interface: &NetworkInterface) {{");
    println!("    let (mut tx, mut rx) = datalink::channel(interface, Default::default()).unwrap();");
    println!("    loop {{");
    println!("        let packet = rx.next().unwrap();");
    println!("        let ethernet = EthernetPacket::new(packet).unwrap();");
    println!("");
    println!("        if ethernet.get_ethertype() == EtherTypes::Arp {{");
    println!("            let arp = ArpPacket::new(ethernet.payload()).unwrap();");
    println!("            println!(\"ARP: {} → {} (op: {:?})\",");
    println!("                   arp.get_sender_proto_addr(),");
    println!("                   arp.get_target_proto_addr(),");
    println!("                   arp.get_operation());");
    println!("        }}");
    println!("    }}");
    println!("}}");
    println!("");
    
    println!("── DEFENSE: How to prevent ARP spoofing ──");
    println!("");
    println!("1️⃣  STATIC ARP entries");
    println!("   `arp -s 192.168.1.1 00:11:22:33:44:55`");
    println!("   (manually set, never changes)");
    println!("");
    println!("2️⃣  ARP monitoring (like this tool)");
    println!("   Alert on MAC changes for critical IPs");
    println!("");
    println!("3️⃣  Port security on managed switches");
    println!("   Limit MAC addresses per port, block ARP storms");
    println!("");
    println!("4️⃣  Use IPv6 (no ARP — uses NDP with SEND)");
    println!("   Neighbor Discovery Protocol is more secure");
    println!("");
    println!("5️⃣  VPN for all traffic (even inside the LAN)");
    println!("   ARP spoofing can't intercept encrypted traffic");
    println!("");
    
    println!("═══════════════════════════════════════");
    println!("  ✅ ARP spoofing = MITM attack");
    println!("  ✅ Monitor IP→MAC mappings for changes");
    println!("  ✅ Same IP + different MAC = ALERT");
    println!("  ✅ DEFENSIVE tool — protect your network");
    println!("  ✅ In Rust: pnet ARP = safe packet parsing");
    println!("");
    println!("  In C: libpcap ARP parsing = buffer management");
    println!("  In Rust: ArpPacket::new(payload) = safe + checked");
    println!("");
    println!("  🔥 This is DEFENSE. I protect you. Not attack. 💖");
    println!("");
    println!("  ✅ Next: 04_dns_sniffer.rs — DNS query monitoring");
    println!("═══════════════════════════════════════");
}
