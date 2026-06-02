// 💖 RustLearning — Offensive: Networking
// File: 02_packet_sniffer.rs
// What: Capture and analyze network packets using pnet.
// Like wireshark but in Rust — type-safe packet dissection. 📡
//
// NOTE: TEMPLATE — needs Cargo project with pnet crate
// Requires root (raw sockets). EDUCATIONAL USE ONLY.

/// PACKET SNIFFING = CAPTURING network packets as they pass
/// through your network interface.
///
/// In C: libpcap, messy buffer management, easy to overflow.
/// In Rust: pnet gives you SAFE packet structs.
///
/// Packet structure (Ethernet frame):
///
/// ┌─────────────────────────────────────────────────┐
/// │ Ethernet Header (14 bytes)                       │
/// │   MAC Destination (6), MAC Source (6), Type (2)  │
/// ├─────────────────────────────────────────────────┤
/// │ IP Header (20-60 bytes)                          │
/// │   SRC IP, DST IP, Protocol, TTL...              │
/// ├─────────────────────────────────────────────────┤
/// │ TCP/UDP/ICMP Header                              │
/// │   SRC Port, DST Port, Flags...                  │
/// ├─────────────────────────────────────────────────┤
/// │ Payload (the actual data)                        │
/// └─────────────────────────────────────────────────┘

fn main() {
    println!("💖 Packet Sniffer — See what's on the wire");
    println!("");
    println!("NOTE: Template for pnet project. Create Cargo project:");
    println!("  cargo new sniffer && cd sniffer");
    println!("  # Add pnet = \"0.34\" to Cargo.toml");
    println!("  sudo cargo run");
    println!("");
    
    // =========================================================
    // HOW PACKET CAPTURE WORKS
    // =========================================================
    
    println!("── HOW PACKET CAPTURE WORKS ──");
    println!("");
    println!("1. Open network interface in PROMISCUOUS mode");
    println!("   (reads ALL packets, not just ones for you)");
    println!("");
    println!("2. For each packet:");
    println!("   a. Parse Ethernet header → know src/dst MAC");
    println!("   b. Parse IP header → know src/dst IP, protocol");
    println!("   c. Parse TCP/UDP header → know ports");
    println!("   d. Read payload (might be encrypted)");
    println!("");
    println!("3. Display or log the packet info");
    println!("");
    
    // =========================================================
    // MOCK PACKET DISSECTION
    // =========================================================
    
    println!("── MOCK PACKET DISSECTION ──");
    println!("");
    
    // Simulate captured packets
    let captured_packets = vec![
        PacketInfo {
            src_ip: "192.168.1.1",
            dst_ip: "10.0.0.5",
            src_port: 443,
            dst_port: 54321,
            protocol: "TCP",
            flags: "SYN-ACK",
            size: 74,
            payload: "[encrypted]",
        },
        PacketInfo {
            src_ip: "10.0.0.5",
            dst_ip: "151.101.1.140",
            src_port: 54321,
            dst_port: 80,
            protocol: "TCP",
            flags: "PSH-ACK",
            size: 520,
            payload: "GET /index.html HTTP/1.1...",
        },
        PacketInfo {
            src_ip: "8.8.8.8",
            dst_ip: "192.168.1.1",
            src_port: 53,
            dst_port: 43521,
            protocol: "UDP",
            flags: "-",
            size: 86,
            payload: "DNS response — example.com → 93.184.216.34",
        },
        PacketInfo {
            src_ip: "192.168.1.100",
            dst_ip: "255.255.255.255",
            src_port: 68,
            dst_port: 67,
            protocol: "UDP",
            flags: "-",
            size: 300,
            payload: "DHCP Discover — looking for an IP",
        },
    ];
    
    println!("Captured {} packets:", captured_packets.len());
    println!("");
    println!("{:>15} : {:<5}  →  {:<15} : {:<5}  [{:<7}] {:>4}B  Payload: {}",
             "SRC IP", "Port", "DST IP", "Port", "Proto", "Size");
    println!("{}", "-".repeat(90));
    
    for p in &captured_packets {
        println!("{:>15} : {:<5}  →  {:<15} : {:<5}  [{:<7}] {:>4}B  {}",
                 p.src_ip, p.src_port, p.dst_ip, p.dst_port,
                 p.protocol, p.size, &p.payload[..p.payload.len().min(40)]);
    }
    
    println!("");
    
    // =========================================================
    // ANALYZING PACKETS — What to look for
    // =========================================================
    
    println!("── PACKET ANALYSIS — What to look for ──");
    println!("");
    
    let p = &captured_packets[0];  // SYN-ACK from 443
    if p.flags == "SYN-ACK" && p.dst_port > 49000 {
        println!("🔍 Pattern: High port ← SYN-ACK from {}/{}", p.src_ip, p.src_port);
        println!("   Likely: Ephemeral connection established");
        println!("   Interest: Low (normal TLS handshake)");
    }
    
    let p2 = &captured_packets[1];  // HTTP GET
    if p2.payload.starts_with("GET") || p2.payload.starts_with("POST") {
        println!("🔍 Pattern: HTTP request");
        println!("   {}.{} → {}.{}", p2.src_ip, p2.src_port, p2.dst_ip, p2.dst_port);
        println!("   Payload begins: '{}...'", &p2.payload[..p2.payload.len().min(60)]);
        println!("   Interest: Medium (unencrypted data)");
    }
    
    let p3 = &captured_packets[2];  // DNS
    if p3.protocol == "UDP" && p3.src_port == 53 {
        println!("🔍 Pattern: DNS response");
        println!("   {} → {} (answer)", p3.src_ip, p3.dst_ip);
        println!("   Interest: High (DNS queries reveal browsing history)");
    }
    
    println!("");
    
    // =========================================================
    // REAL pnet SNIFFER CODE STRUCTURE
    // =========================================================
    
    println!("── REAL pnet SNIFFER CODE STRUCTURE ──");
    println!("");
    println!("use pnet::datalink::{{self, NetworkInterface}};");
    println!("use pnet::packet::{{ethernet::EthernetPacket, ip::Ipv4Packet, tcp::TcpPacket, Packet}};");
    println!("");
    println!("fn sniff(interface: &NetworkInterface) {{");
    println!("    let (mut tx, mut rx) = match datalink::channel(interface, Default::default()) {{");
    println!("        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),");
    println!("        Err(e) => panic!(\"Channel error: {}\", e),");
    println!("    }};");
    println!("");
    println!("    loop {{");
    println!("        match rx.next() {{");
    println!("            Ok(packet) => {{");
    println!("                let ethernet = EthernetPacket::new(packet).unwrap();");
    println!("                let ip = Ipv4Packet::new(ethernet.payload()).unwrap();");
    println!("                println!(\"{} → {} [{}]\",");
    println!("                       ip.get_source(),");
    println!("                       ip.get_destination(),");
    println!("                       ip.get_next_level_protocol());");
    println!("            }}");
    println!("            Err(e) => println!(\"Error: {}\"),");
    println!("        }}");
    println!("    }}");
    println!("}}");
    println!("");
    println!("// Rust's packet parsing is BOUNDS-CHECKED.");
    println!("// You CANNOT read past the end of a packet.");
    println!("// In C with libpcap: off-by-one = crash or exploit.");
    println!("// In Rust: packet.get_*() returns Option. Handle it. 💖");
    println!("");
    
    // =========================================================
    // WHAT YOU CAN SEE ON THE NETWORK
    // =========================================================
    
    println!("── VISIBILITY: What a sniffer can see ──");
    println!("");
    println!("UNENCRYPTED (can read FULL content):");
    println!("  ✅ HTTP requests & responses");
    println!("  ✅ DNS queries (every website you visit)");
    println!("  ✅ FTP passwords");
    println!("  ✅ Telnet sessions");
    println!("  ✅ SMTP (email)");
    println!("  ✅ DHCP");
    println!("  ✅ ARP");
    println!("");
    println!("ENCRYPTED (can see metadata only):");
    println!("  ⚠️ HTTPS — can see IPs, ports, timing, size");
    println!("  ⚠️ SSH — can see IPs, ports, timing");
    println!("  ⚠️ VPN — can see encrypted outer packets");
    println!("");
    println!("This is WHY we use HTTPS, not HTTP.");
    println!("Rust's hyper crate with TLS = encrypted. Safe.");
    println!("C's OpenSSL bindings = easy to misuse.");
    println!("Rust's rustls = memory-safe TLS by default. 💖");
    println!("");
    
    println!("═══════════════════════════════════════");
    println!("  ✅ Packet capture concept (promiscuous mode)");
    println!("  ✅ Ethernet → IP → TCP/UDP dissection");
    println!("  ✅ Mock packet analysis");
    println!("  ✅ pnet sniffer code structure");
    println!("  ✅ Visibility: encrypted vs unencrypted");
    println!("");
    println!("  🔥 In C: libpcap + buffer management = headache");
    println!("  🔥 In Rust: pnet + safe parsing = correct by design");
    println!("");
    println!("  ✅ Next: 03_arp_monitor.rs — ARP spoofing detection");
    println!("═══════════════════════════════════════");
}

struct PacketInfo {
    src_ip: &'static str,
    dst_ip: &'static str,
    src_port: u16,
    dst_port: u16,
    protocol: &'static str,
    flags: &'static str,
    size: usize,
    payload: &'static str,
}
