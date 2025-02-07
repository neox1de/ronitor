use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use serde::Serialize;

pub struct SystemCollector {
    sys: System,
}

impl SystemCollector {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys }
    }

    pub fn get_detailed_cpu_info(&mut self) -> CpuDetails {
        self.sys.refresh_cpu();
        let cpus = self.sys.cpus();
        
        let cores = cpus.iter()
            .map(|cpu| CoreInfo {
                id: cpu.name().to_string(),
                frequency_mhz: cpu.frequency(),
                usage_percentage: cpu.cpu_usage(),
            })
            .collect();

        CpuDetails {
            processor_name: cpus.first().map_or("Unknown".to_string(), |cpu| cpu.brand().to_string()),
            vendor: cpus.first().map_or("Unknown".to_string(), |cpu| cpu.vendor_id().to_string()),
            cores_count: cpus.len(),
            average_frequency: cpus.iter().map(|cpu| cpu.frequency()).sum::<u64>() / cpus.len() as u64,
            average_usage: cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32,
            cores,
        }
    }

    pub fn get_detailed_memory_info(&mut self) -> MemoryInfo {
        self.sys.refresh_memory();
        MemoryInfo {
            total_memory: self.sys.total_memory(),
            used_memory: self.sys.used_memory(),
            free_memory: self.sys.free_memory(),
            available_memory: self.sys.available_memory(),
            usage_percentage: (self.sys.used_memory() as f32 / self.sys.total_memory() as f32) * 100.0,
        }
    }

    pub fn get_detailed_disk_info(&mut self) -> DiskDetails {
        self.sys.refresh_disks();
        let disks = self.sys.disks();
        
        let total_space: u64 = disks.iter().map(|disk| disk.total_space()).sum();
        let available_space: u64 = disks.iter().map(|disk| disk.available_space()).sum();
        let used_space = total_space - available_space;

        let volumes = disks.iter()
            .map(|disk| VolumeInfo {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                filesystem: String::from_utf8_lossy(disk.file_system()).to_string(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
                used_space: disk.total_space() - disk.available_space(),
                usage_percentage: (disk.total_space() - disk.available_space()) as f32 / disk.total_space() as f32 * 100.0,
                is_removable: disk.is_removable(),
            })
            .collect();

        DiskDetails {
            total_space,
            available_space,
            used_space,
            usage_percentage: (used_space as f32 / total_space as f32) * 100.0,
            volumes_count: disks.len(),
            volumes,
        }
    }
}

#[derive(Serialize)]
pub struct CpuDetails {
    pub processor_name: String,
    pub vendor: String,
    pub cores_count: usize,
    pub average_frequency: u64,
    pub average_usage: f32,
    pub cores: Vec<CoreInfo>,
}

#[derive(Serialize)]
pub struct CoreInfo {
    pub id: String,
    pub frequency_mhz: u64,
    pub usage_percentage: f32,
}

#[derive(Serialize)]
pub struct CpuInfo {
    pub name: String,
    pub model: String,
    pub frequency: u64,
    pub usage: f32,
    pub vendor_id: String,
}

#[derive(Serialize)]
pub struct MemoryInfo {
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
    pub available_memory: u64,
    pub usage_percentage: f32,
}

#[derive(Serialize)]
pub struct DiskDetails {
    pub total_space: u64,
    pub available_space: u64,
    pub used_space: u64,
    pub usage_percentage: f32,
    pub volumes_count: usize,
    pub volumes: Vec<VolumeInfo>,
}

#[derive(Serialize)]
pub struct VolumeInfo {
    pub name: String,
    pub mount_point: String,
    pub filesystem: String,
    pub total_space: u64,
    pub available_space: u64,
    pub used_space: u64,
    pub usage_percentage: f32,
    pub is_removable: bool,
}
