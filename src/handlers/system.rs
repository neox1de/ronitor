use actix_web::{HttpResponse, Responder};
use std::process::Command;

pub async fn shutdown_handler() -> impl Responder {
    let _ = Command::new("shutdown")
        .args(&["-h", "now"])
        .spawn();
    HttpResponse::Ok().body("Shutdown command issued")
}

pub async fn reboot_handler() -> impl Responder {
    let _ = Command::new("shutdown")
        .args(&["-r", "now"])
        .spawn();
    HttpResponse::Ok().body("Reboot command issued")
}
