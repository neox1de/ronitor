use serde::Serialize;
use std::collections::HashMap;
use crate::collectors::{
    system::{CpuDetails, MemoryInfo, DiskDetails},
    network::{NetworkInterface, NetworkUsage},
};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Serialize)]
pub struct SystemMetrics {
    pub cpu: CpuDetails,
    pub memory: MemoryInfo,
    pub disks: DiskDetails,
}

#[derive(Serialize)]
pub struct SystemInfo {
    pub cpu_info: Vec<(String, f32)>,
    pub memory_info: (u64, u64, f32),
    pub disk_info: Vec<(String, u64, u64, f32)>,
}

#[derive(Serialize, Default)]
pub struct KernelInfo {
    pub version: String,
    pub release: String,
    pub architecture: String,
    pub os_name: String,
    pub hostname: String,
    pub uptime: SystemUptime,
    pub last_boot: String,
    pub kernel_parameters: HashMap<String, String>,
    pub loaded_modules: Vec<KernelModule>,
    pub logs: Vec<KernelLog>,
}

#[derive(Serialize, Default)]
pub struct SystemUptime {
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub total_seconds: u64,
}

#[derive(Serialize, Default)]
pub struct KernelModule {
    pub name: String,
    pub size: String,
    pub used_by: Vec<String>,
    pub state: String,
}

#[derive(Serialize, Default)]
pub struct KernelLog {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct NetworkInfo {
    pub interfaces: HashMap<String, NetworkInterface>,
    pub usage: Vec<NetworkUsage>,
}

#[derive(Serialize, Default)]
pub struct ProcessInfo {
    pub name: String,
    pub pid: i32,
    pub exe: String,
    pub cmd: Vec<String>,
    pub memory: u64,
    pub cpu_usage: f32,
}
