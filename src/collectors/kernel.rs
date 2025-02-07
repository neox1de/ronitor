use std::process::Command;
use std::io::Result;
use std::collections::HashMap;
use regex::Regex;
use crate::models::{KernelInfo, SystemUptime, KernelModule, KernelLog};

pub struct KernelCollector;

impl KernelCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn get_kernel_info(&self) -> Result<KernelInfo> {
        Ok(KernelInfo {
            version: self.get_kernel_version()?,
            release: self.get_kernel_release()?,
            architecture: self.get_architecture()?,
            os_name: self.get_os_name()?,
            hostname: self.get_hostname()?,
            uptime: self.get_system_uptime()?,
            last_boot: self.get_last_boot_time()?,
            kernel_parameters: self.get_kernel_parameters()?,
            loaded_modules: self.get_loaded_modules()?,
            logs: self.get_kernel_logs(50)?,
        })
    }

    fn get_kernel_version(&self) -> Result<String> {
        let output = Command::new("uname").arg("-v").output()?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    fn get_kernel_release(&self) -> Result<String> {
        let output = Command::new("uname").arg("-r").output()?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    fn get_architecture(&self) -> Result<String> {
        let output = Command::new("uname").arg("-m").output()?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    fn get_os_name(&self) -> Result<String> {
        let output = Command::new("uname").arg("-o").output()?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    fn get_hostname(&self) -> Result<String> {
        let output = Command::new("uname").arg("-n").output()?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    fn get_system_uptime(&self) -> Result<SystemUptime> {
        let output = Command::new("cat")
            .arg("/proc/uptime")
            .output()?;
        // Parse uptime as float to handle fractional seconds
        let uptime_str = String::from_utf8_lossy(&output.stdout);
        let uptime_seconds: f64 = uptime_str.split_whitespace()
            .next()
            .unwrap_or("0")
            .parse()
            .unwrap_or(0.0);
        let seconds = uptime_seconds.floor() as u64;

        Ok(SystemUptime {
            days: seconds / 86400,
            hours: (seconds % 86400) / 3600,
            minutes: (seconds % 3600) / 60,
            seconds: seconds % 60,
            total_seconds: seconds,
        })
    }

    fn get_last_boot_time(&self) -> Result<String> {
        let output = Command::new("who").arg("-b").output()?;
        Ok(String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace("system boot", "")
            .trim()
            .to_string())
    }

    fn get_kernel_parameters(&self) -> Result<HashMap<String, String>> {
        let output = Command::new("sysctl").arg("-a").output()?;
        let mut params = HashMap::new();
        
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if let Some((key, value)) = line.split_once('=') {
                params.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        
        Ok(params)
    }

    fn get_loaded_modules(&self) -> Result<Vec<KernelModule>> {
        let output = Command::new("lsmod").output()?;
        let mut modules = Vec::new();
        
        for line in String::from_utf8_lossy(&output.stdout).lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                modules.push(KernelModule {
                    name: parts[0].to_string(),
                    size: parts[1].to_string(),
                    used_by: parts[2].split(',').map(String::from).collect(),
                    state: "loaded".to_string(),
                });
            }
        }
        
        Ok(modules)
    }

    pub fn get_kernel_logs(&self, lines: usize) -> Result<Vec<KernelLog>> {
        let output = Command::new("journalctl")
            .arg("--kernel")
            .arg(format!("--lines={}", lines))
            .output()?;

        let mut logs = Vec::new();
        let re = Regex::new(r"^(\w+\s+\d+\s+\d+:\d+:\d+).*?(\w+):\s+(.+)$").unwrap();
        
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if let Some(caps) = re.captures(line) {
                logs.push(KernelLog {
                    timestamp: caps[1].to_string(),
                    level: caps[2].to_string(),
                    message: caps[3].to_string(),
                });
            }
        }
        
        Ok(logs)
    }
}
