use std::process::Command;
use std::collections::HashMap;
use std::io::Result;
use serde::Serialize;
use regex::Regex;

// Modified: removed speed and duplex fields.
#[derive(Serialize, Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_addresses: Vec<String>,
    pub mac_address: String,
    pub is_up: bool,
    pub mtu: u32,
}

#[derive(Serialize, Debug, Clone)]
pub struct NetworkUsage {
    pub interface: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
}


#[derive(Serialize, Debug, Clone)]
pub struct NetworkInfo {
    pub name: String,
    pub ip_addresses: Vec<String>,
    pub mac_address: String,
    pub usage: Option<NetworkUsage>,
}

pub struct NetworkCollector;

impl NetworkCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn get_interfaces(&self) -> Result<HashMap<String, NetworkInterface>> {
        let output = Command::new("ip")
            .arg("addr")
            .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut interfaces = HashMap::new();
        let mut current_interface: Option<NetworkInterface> = None;

        for line in output_str.lines() {
            if line.starts_with(char::is_numeric) {
                // Process new interface
                if let Some(interface) = &current_interface {
                    interfaces.insert(interface.name.clone(), interface.clone());
                }
                let re = Regex::new(r"^\d+:\s+([^:]+):\s+<(.+)>.*mtu\s+(\d+)").unwrap();
                if let Some(caps) = re.captures(line) {
                    let name = caps[1].trim().to_string();
                    let flags = caps[2].to_string();
                    let mtu = caps[3].parse().unwrap_or(0);
                    let is_up = flags.contains("UP");

                    current_interface = Some(NetworkInterface {
                        name,
                        ip_addresses: Vec::new(),
                        mac_address: String::new(),
                        is_up,
                        mtu,
                    });
                }
            } else if line.contains("inet ") {
                if let Some(ref mut interface) = current_interface {
                    let re = Regex::new(r"inet\s+([0-9.]+)/\d+").unwrap();
                    if let Some(caps) = re.captures(line) {
                        interface.ip_addresses.push(caps[1].to_string());
                    }
                }
            } else if line.contains("link/ether") {
                // Add MAC address
                if let Some(ref mut interface) = current_interface {
                    let re = Regex::new(r"link/ether\s+([0-9a-fA-F:]+)").unwrap();
                    if let Some(caps) = re.captures(line) {
                        interface.mac_address = caps[1].to_string();
                    }
                }
            }
        }

        // Last call for interfaces! Don't leave that lonely network buddy hanging :D
        if let Some(interface) = &current_interface {
            interfaces.insert(interface.name.clone(), interface.clone());
        }

        Ok(interfaces)
    }

    pub fn get_network_usage(&self) -> Result<Vec<NetworkUsage>> {
        let output = Command::new("cat")
            .arg("/proc/net/dev")
            .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut usage = Vec::new();

        for line in output_str.lines().skip(2) { // Skip header lines
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 17 {
                let interface = parts[0].trim_end_matches(':').to_string();
                if interface != "lo" { // Skip loopback
                    usage.push(NetworkUsage {
                        interface,
                        rx_bytes: parts[1].parse().unwrap_or(0),
                        tx_bytes: parts[9].parse().unwrap_or(0),
                        rx_packets: parts[2].parse().unwrap_or(0),
                        tx_packets: parts[10].parse().unwrap_or(0),
                        rx_errors: parts[3].parse().unwrap_or(0),
                        tx_errors: parts[11].parse().unwrap_or(0),
                    });
                }
            }
        }

        Ok(usage)
    }
}
