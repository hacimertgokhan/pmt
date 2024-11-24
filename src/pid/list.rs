use std::io;
use std::process::Command;
use sysinfo::{System};

pub async fn find_port_by_pid(pid: u32) -> io::Result<Option<String>> {
    let output = Command::new("netstat")
        .arg("-ano")
        .output()?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut port_info = None;

    for line in output_str.lines() {
        if line.contains(&pid.to_string()) {
            port_info = Some(line.to_string());
            break; // İlk eşleşmeyi bulduktan sonra döngüyü kır
        }
    }

    Ok(port_info)
}

pub async fn list_used_pids() {
    let system = System::new_all();
    let mut seen_pids = std::collections::HashSet::new(); // Tekrar eden PID'leri önlemek için

    for (pid, process) in system.processes() {
        if seen_pids.insert(pid.as_u32()) { // Eğer PID daha önce görülmemişse
            let process_name = process.name().to_string_lossy();
            match find_port_by_pid(pid.as_u32()).await {
                Ok(Some(port_info)) => {
                    println!("PID: {:<10}\tProcess: {:<20}\tPort Info: {}", pid, process_name, port_info);
                }
                Ok(None) => {
                    println!("PID: {:<10}\tProcess: {:<20}\tPort Info: No port found", pid, process_name);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
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
                Ok(Some(port_info)) => {
                    println!("PID: {:<10}\tProcess: {:<20}\tPort Info: {}", pid, name, port_info);
                }
                Ok(None) => {
                    println!("PID: {:<10}\tProcess: {:<20}\tPort Info: No port found", pid, name);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }

    if !found {
        println!("No application found with the name \"{}\".", process_name);
    }
}