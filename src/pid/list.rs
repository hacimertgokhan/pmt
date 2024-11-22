use std::io;
use std::process::Command;
use sysinfo::{System};

pub async fn find_port_by_pid(pid: u32) -> io::Result<String> {
    let output = Command::new("netstat")
        .arg("-ano")
        .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut port_info = String::new();
    for line in output_str.lines() {
        if line.contains(&pid.to_string()) {
            port_info.push_str(&format!("{}\t:>{}\n", pid, line));
        }
    }
    if port_info.is_empty() {
        port_info.push_str(&format!("{}\n", pid));
    }

    Ok(port_info)
}

pub async fn list_used_pids() {
    let system = System::new_all();
    for (pid, process) in system.processes() {
        let process_name = process.name().to_string_lossy();
        match find_port_by_pid(pid.as_u32()).await {
            Ok(port_info) => {
                println!("{:<10}\t{:}\t{}", pid, process_name, port_info.trim());
            }
            Err(e) => {
                eprintln!("Hata: {}", e);
            }
        }
    }
}

pub async fn list_pids_by_process_name(process_name: &str) {
    let system = System::new_all();
    let mut found = false;
    for (pid, process) in system.processes() {
        let name = process.name().to_string_lossy();
        if name.contains(process_name) {
            found = true;
            match find_port_by_pid(pid.as_u32()).await {
                Ok(port_info) => {
                    println!(">: {}", port_info.trim());
                }
                Err(e) => {
                    eprintln!("Hata: {}", e);
                }
            }
        }
    }
    if !found {
        println!("\"{}\" adında bir uygulama bulunamadı.", process_name);
    }
}
