use actix_web::{HttpResponse, Responder};
use crate::collectors::{system::SystemCollector, kernel::KernelCollector, network::NetworkCollector};
use crate::models::*;
pub mod system;

pub async fn health_handler() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
    })
}

pub async fn processes_handler() -> impl Responder {
    let collector = crate::collectors::processes::ProcessCollector::new();
    let processes = collector.get_processes();
    HttpResponse::Ok().json(processes)
}


pub async fn metrics_handler() -> impl Responder {
    let mut sys_collector = SystemCollector::new();
    let metrics = SystemMetrics {
        cpu: sys_collector.get_detailed_cpu_info(),
        memory: sys_collector.get_detailed_memory_info(),
        disks: sys_collector.get_detailed_disk_info(),
    };
    HttpResponse::Ok().json(metrics)
}

pub async fn kernel_info_handler() -> impl Responder {
    let kernel_collector = KernelCollector::new();
    let kernel_info = kernel_collector.get_kernel_info().unwrap_or_default();
    HttpResponse::Ok().json(kernel_info)
}

pub async fn network_info_handler() -> impl Responder {
    let network_collector = NetworkCollector::new();
    let network_info = NetworkInfo {
        interfaces: network_collector.get_interfaces().unwrap_or_default(),
        usage: network_collector.get_network_usage().unwrap_or_default(),
    };
    HttpResponse::Ok().json(network_info)
}
