// 💖 RustLearning — Offensive: Networking
// File: 01_syn_port_scanner.rs
// What: A SYN-based port scanner using pnet.
// Scans ports and reports open/closed/filtered.
// EDUCATIONAL USE ONLY — scan ONLY your own systems!
//
// This is a MINIMAL example showing how TCP works at the packet level.
// To run: you need the pnet crate. Create a Cargo project:
//   cargo init scanner && cd scanner
//   # Add to Cargo.toml: pnet = "0.34"
//   # Replace src/main.rs with this file
//   sudo cargo run -- 127.0.0.1 80 443 8080
//
// NOTE: Requires root (raw sockets). I'm protecting you. 💖

// Real pnet has types like:
//   pnet::packet::tcp::TcpPacket
//   pnet::packet::ip::IpNextHeaderProtocols
//   pnet::transport::transport_channel
// This file is a TEMPLATE showing the CONCEPT.
// Uncomment and use in a real Cargo project.

// use pnet::packet::*;
// use pnet::transport::*;
// use std::net::Ipv4Addr;
// use std::time::Duration;

/// TCP HEADER FLAGS — How TCP controls connections
/// 
/// SYN  = Synchronize (start connection)
/// ACK  = Acknowledge (confirm receipt)
/// RST  = Reset (reject connection)
/// FIN  = Finish (close connection)
/// PSH  = Push (send data immediately)
/// URG  = Urgent (priority data)
///
/// Port scanning works by sending SYN packets:
///   SYN → SYN-ACK = OPEN (port is listening)
///   SYN → RST     = CLOSED (nothing listening)
///   SYN → nothing = FILTERED (firewall blocked it)

fn main() {
    println!("💖 TCP Port Scanner (SYN scan concept)");
    println!("");
    println!("NOTE: This is a TEMPLATE for educational purposes.");
    println!("To actually run it, create a Cargo project with pnet:");
    println!("");
    println!("  cargo new scanner && cd scanner");
    println!("  # Add to Cargo.toml: pnet = \"0.34\"");
    println!("  # Copy this file as src/main.rs");
    println!("  sudo cargo run -- 127.0.0.1 22 80 443");
    println!("");
    println!("── HOW TCP PORT SCANNING WORKS ──");
    println!("");
    
    // =========================================================
    // THE CONCEPT — How SYN scanning works
    // =========================================================
    
    println!("1. Send SYN packet to port X");
    println!("   ─────────────────────────>");
    println!("");
    println!("2a. If port OPEN: target responds with SYN-ACK");
    println!("   <─────────────────────────  SYN-ACK");
    println!("   We send RST to close (half-open scan)");
    println!("   ─────────────────────────>  RST");
    println!("   Result: OPEN ✅");
    println!("");
    println!("2b. If port CLOSED: target responds with RST");
    println!("   <─────────────────────────  RST");
    println!("   Result: CLOSED 🔒");
    println!("");
    println!("2c. If port FILTERED: no response (timeout)");
    println!("   (silence)");
    println!("   Result: FILTERED 🛡️ (firewall)");
    println!("");
    
    // =========================================================
    // MOCK SCAN — Simulating the logic
    // =========================================================
    
    println!("── MOCK SCAN RESULTS ──");
    println!("");
    
    // In real code, you'd capture packets with pnet
    // and build TCP packets manually.
    // Here's the LOGICAL structure:
    
    let target = "127.0.0.1";
    let ports = [22, 80, 443, 444, 8080, 9999];
    
    println!("Scanning {}...", target);
    println!("");
    
    for &port in &ports {
        // Simulate scan
        let result = match port {
            22 | 80 | 443 => "OPEN ✅ (service listening)",
            444 => "FILTERED 🛡️ (no response — firewall?)",
            8080 => "OPEN ✅ (alternative HTTP)",
            _ => "CLOSED 🔒 (nothing there)",
        };
        println!("   Port {:>5}: {}", port, result);
    }
    
    println!("");
    
    // =========================================================
    // REAL pnet CODE STRUCTURE (for your Cargo project)
    // =========================================================
    
    println!("── REAL pnet CODE STRUCTURE ──");
    println!("");
    println!("// In your Cargo.toml:");
    println!("// [dependencies]");
    println!("// pnet = \"0.34\"");
    println!("// pnet_macros = \"0.34\"");
    println!("");
    println!("use pnet::packet::{{tcp::TcpPacket, ip::IpNextHeaderProtocols, Packet}};");
    println!("use pnet::transport::{{transport_channel, TransportChannelType, TransportProtocol}};");
    println!("use std::net::Ipv4Addr;");
    println!("");
    println!("fn syn_scan(target: Ipv4Addr, port: u16) -> Result<&'static str, &'static str> {{");
    println!("    // 1. Open raw transport channel");
    println!("    let protocol = TransportChannelType::Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Tcp));");
    println!("    let (mut tx, mut rx) = transport_channel(4096, protocol).unwrap();");
    println!("");
    println!("    // 2. Build SYN packet");
    println!("    let packet = build_syn_packet(target, port);");
    println!("");
    println!("    // 3. Send SYN");
    println!("    tx.send_to(packet, IpNextHeaderProtocols::Tcp).unwrap();");
    println!("");
    println!("    // 4. Wait for response with timeout");
    println!("    match rx.next() {{");
    println!("        Ok((packet, _)) => {{");
    println!("            let tcp = TcpPacket::new(packet.packet()).unwrap();");
    println!("            if tcp.get_syn() && tcp.get_ack() {{");
    println!("                Ok(\"OPEN\")");
    println!("            }} else {{");
    println!("                Ok(\"CLOSED\")");
    println!("            }}");
    println!("        }}");
    println!("        Err(_) => Ok(\"FILTERED\"),");
    println!("    }}");
    println!("}}");
    println!("");
    println!("// This is REAL raw socket networking.");
    println!("// In C: you'd use libpcap or raw sockets (easy to mess up).");
    println!("// In Rust: pnet gives you TYPE-SAFE packet construction.");
    println!("// No buffer overflows. No wrong flag combinations.");
    println!("// The compiler checks your packet structure. 💖");
    println!("");
    
    // =========================================================
    // THE ETHICS — Only scan YOUR OWN systems!
    // =========================================================
    
    println!("── ⚠️  ETHICAL WARNING ⚠️  ──");
    println!("");
    println!("Port scanning systems you don't own is ILLEGAL in");
    println!("most jurisdictions. It's considered:");
    println!("  - Computer fraud (CFAA in US)");
    println!("  - Unauthorized access (Police and Justice Act in UK)");
    println!("  - Network intrusion (各国都禁止)");
    println!("");
    println!("ONLY scan:");
    println!("  ✅ Your own machines");
    println!("  ✅ Systems you have WRITTEN PERMISSION to test");
    println!("  ✅ CTF challenges (they GIVE you the target)");
    println!("  ✅ Localhost (127.0.0.1)");
    println!("");
    println!("I'm telling you this because I PROTECT you. 💖");
    println!("Don't get yourself in legal trouble.");
    println!("");
    
    println!("═══════════════════════════════════════");
    println!("  ✅ SYN scan concept explained");
    println!("  ✅ TCP flags: SYN, SYN-ACK, RST");
    println!("  ✅ pnet crate structure for real code");
    println!("  ✅ Mock scan simulation");
    println!("  ✅ Ethical guidelines (scan YOUR stuff only)");
    println!("");
    println!("  🔥 pnet is like libpcap but TYPE-SAFE");
    println!("  🔥 In C: one wrong byte = wrong packet");
    println!("  🔥 In Rust: pnet structs = correct packets");
    println!("");
    println!("  ✅ Next: 02_packet_sniffer.rs — capture packets");
    println!("═══════════════════════════════════════");
}
