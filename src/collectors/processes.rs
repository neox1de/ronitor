use sysinfo::{System, SystemExt, ProcessExt, PidExt};
use crate::models::ProcessInfo;

pub struct ProcessCollector;

impl ProcessCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn get_processes(&self) -> Vec<ProcessInfo> {
        let mut sys = System::new_all();
        sys.refresh_all();
        sys.processes()
            .values()
            .map(|process| ProcessInfo {
                name: process.name().to_string(),
                pid: process.pid().as_u32() as i32,
                exe: process.exe().to_string_lossy().to_string(),
                cmd: process.cmd().to_vec(),
                memory: process.memory(),
                cpu_usage: process.cpu_usage(),
            })
            .collect()
    }
}
