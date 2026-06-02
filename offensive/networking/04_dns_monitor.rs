// 💖 RustLearning — Offensive: Networking
// File: 04_dns_monitor.rs
// What: DNS query monitor — see what domains are being resolved.
// DEFENSIVE tool for network monitoring and anomaly detection. 📡
//
// NOTE: TEMPLATE — needs pnet crate
// EDUCATIONAL USE ONLY — monitor your OWN network.

/// DNS = Domain Name System
/// "What is the IP of example.com?" → "93.184.216.34"
///
/// DNS queries are OFTEN UNENCRYPTED (even for HTTPS sites).
/// This means a sniffer can see EVERY website you visit.
///
/// DNS monitoring is useful for:
/// ✅ Detecting malware (beaconing to C2 domains)
/// ✅ Monitoring network activity (seeing what devices do)
/// ✅ Detecting DNS tunneling (data exfiltration)
///
/// This is DEFENSIVE. You're monitoring YOUR network.
/// NOT attacking others.

use std::collections::HashMap;

/// Track DNS query stats
struct DnsMonitor {
    queries: HashMap<String, u32>,
    suspicious_domains: Vec<&'static str>,
}

impl DnsMonitor {
    fn new() -> Self {
        DnsMonitor {
            queries: HashMap::new(),
            suspicious_domains: vec![
                "malware.example.com",
                "c2.evilserver.com",
                "phishing.attack.net",
                "pastebin.com",  // Data exfiltration
                "dropbox.com",   // Unauthorized cloud sync
                "torrent.example.org",
            ],
        }
    }
    
    /// Log a DNS query
    fn log_query(&mut self, domain: &str, ip: &str) {
        let count = self.queries.entry(domain.to_string()).or_insert(0);
        *count += 1;
        
        // Check for suspicious domains
        let is_suspicious = self.suspicious_domains.iter()
            .any(|&d| domain.contains(d));
        
        if is_suspicious {
            println!("   🔴 SUSPICIOUS: {} → {} (ALERT! Known bad domain!)", domain, ip);
        } else if *count > 5 {
            println!("   🟡 FREQUENT: {} → {} ({} queries — unusual volume)", domain, ip, count);
        } else {
            println!("   ℹ️  QUERY: {} → {}", domain, ip);
        }
    }
    
    /// Show statistics
    fn show_stats(&self) {
        println!("");
        println!("   DNS Query Statistics:");
        println!("   {:<40} {:<10}", "Domain", "Count");
        println!("   {}", "-".repeat(50));
        
        let mut entries: Vec<_> = self.queries.iter().collect();
        entries.sort_by(|a, b| b.1.cmp(a.1));  // Sort by count, descending
        
        for (domain, count) in entries {
            let flag = if self.suspicious_domains.iter().any(|d| domain.contains(d)) {
                "🔴"
            } else if *count > 5 {
                "🟡"
            } else {
                "✅"
            };
            println!("   {} {:<38} {:<10}", flag, domain, count);
        }
    }
}

fn main() {
    println!("💖 DNS Monitor — See what domains are being resolved");
    println!("");
    println!("NOTE: Template. In real code, capture UDP port 53 packets.");
    println!("In this demo: simulate DNS traffic.");
    println!("");
    println!("── SIMULATING DNS TRAFFIC ──");
    println!("");
    
    let mut monitor = DnsMonitor::new();
    
    // Simulate DNS queries from a network
    let dns_traffic = vec![
        ("google.com", "142.250.80.46"),
        ("youtube.com", "142.250.80.110"),
        ("github.com", "140.82.121.4"),
        ("google.com", "142.250.80.46"),       // Repeated
        ("google.com", "142.250.80.46"),       // Repeated
        ("github.com", "140.82.121.4"),        // Repeated
        ("reddit.com", "151.101.1.140"),
        ("google.com", "142.250.80.46"),       // Repeated
        ("google.com", "142.250.80.46"),       // Frequency anomaly?
        ("google.com", "142.250.80.46"),       // 6 queries to google?
        ("malware.example.com", "198.51.100.50"),  // SUSPICIOUS!
        ("stackoverflow.com", "151.101.129.69"),
        ("c2.evilserver.com", "203.0.113.99"),     // SUSPICIOUS!
        ("pastebin.com", "104.20.208.14"),          // SUSPICIOUS (data exfil)
        ("google.com", "142.250.80.46"),            // 7th time
        ("normal-site.org", "93.184.216.34"),
    ];
    
    for (domain, ip) in dns_traffic {
        monitor.log_query(domain, ip);
    }
    
    monitor.show_stats();
    
    println!("");
    println!("── ANALYSIS ──");
    println!("");
    println!("🔍 What we detected:");
    println!("   1. google.com queried 7 times — possible beaconing");
    println!("   2. malware.example.com — known malicious domain 🔴");
    println!("   3. c2.evilserver.com — C2 server communication 🔴");
    println!("   4. pastebin.com — possible data exfiltration 🔴");
    println!("   5. Normal traffic also visible (github, youtube, etc.)");
    println!("");
    
    // =========================================================
    // DNS TUNNELING — What to look for
    // =========================================================
    
    println!("── DNS TUNNELING DETECTION ──");
    println!("");
    println!("DNS tunneling = encoding data in DNS queries");
    println!("Example: exfiltrated-data.evil.com resolves to...");
    println!("   the encoded data itself, server-side!");
    println!("");
    println!("SIGNS of DNS tunneling:");
    println!("   🔴 Very long subdomains (normal: 20 chars, tunnel: 200+)");
    println!("   🔴 High frequency of queries to ONE domain");
    println!("   🔴 Random-looking subdomain prefixes");
    println!("   🔴 Unusual record types (TXT, NULL)");
    println!("   🔴 Queries to domains that don't host websites");
    println!("");
    
    // =========================================================
    // DEFENSE: DoH (DNS over HTTPS)
    // =========================================================
    
    println!("── DEFENSE: DNS over HTTPS (DoH) ──");
    println!("");
    println!("DoH encrypts DNS queries so sniffers can't see them.");
    println!("Instead of UDP port 53, DNS goes over HTTPS port 443.");
    println!("");
    println!("In Rust, use the `trust-dns` or `hickory-resolver` crate:");
    println!("   use hickory_resolver::config::ResolverConfig;");
    println!("   use hickory_resolver::Resolver;");
    println!("   let resolver = Resolver::new(ResolverConfig::cloudflare_tls()).unwrap();");
    println!("");
    println!("Rust's TLS crates (rustls) are memory-safe.");
    println!("OpenSSL (C) has had 20+ critical CVEs.");
    println!("rustls has had... 0. Because it's Rust. 💖");
    println!("");
    
    println!("═══════════════════════════════════════");
    println!("  ✅ DNS queries reveal visited domains");
    println!("  ✅ Monitor for malware beaconing");
    println!("  ✅ Detect DNS tunneling (long names, frequency)");
    println!("  ✅ DEFENSIVE: alert on known-bad domains");
    println!("");
    println!("  🔥 In C: parse DNS packets manually (easy to mess up)");
    println!("  🔥 In Rust: trust-dns crate = SAFE DNS parsing");
    println!("");
    println!("  ✅ Next: process/ — system-level techniques");
    println!("═══════════════════════════════════════");
}
