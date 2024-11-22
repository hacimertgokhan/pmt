use sysinfo::{System};
use std::process::Command;

pub(crate)  async fn list_used_ports() {
    println!("Kullanılan portlar eşzamanlı olarak listeleniyor...");
    let used_ports = get_used_ports().await;
    for (port, app) in used_ports {
        println!("Kullanılan Port: {}, Uygulama: {}", port, app);
    }
}

async fn get_used_ports() -> Vec<(u16, String)> {
    let output = Command::new("lsof")
        .arg("-i")
        .output()
        .expect("lsof komutu çalıştırılamadı");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut used_ports = Vec::new();

    for line in output_str.lines() {
        if let Some((port, app)) = extract_port_and_app(line) {
            used_ports.push((port, app));
        }
    }

    used_ports
}

fn extract_port_and_app(line: &str) -> Option<(u16, String)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() > 0 {
        if let Some(port_info) = parts.iter().find(|&&part| part.contains(':')) {
            let port_str = port_info.split(':').last().unwrap();
            if let Ok(port) = port_str.parse::<u16>() {
                let app_name = parts[0].to_string();
                return Some((port, app_name));
            }
        }
    }
    None
}

fn extract_port(line: &str) -> Option<u16> {
    if let Some(pos) = line.find(':') {
        let port_str = &line[pos + 1..pos + 6]; // Port numarasını al
        if let Ok(port) = port_str.parse::<u16>() {
            return Some(port);
        }
    }
    None
}

fn get_app_using_port(port: (u16, String)) -> Option<String> {
    let output = Command::new("lsof")
        .arg("-i")
        .arg(format!("tcp:{:?}", port))
        .output()
        .ok()?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();

    if lines.len() > 1 {
        let parts: Vec<&str> = lines[1].split_whitespace().collect();
        if parts.len() > 0 {
            return Some(parts[0].to_string());
        }
    }
    None
}