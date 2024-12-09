use std::io;
use std::process::Command;

pub fn kill_process_by_name_and_port(process_name: &str, port: u16) -> io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        // Windows için netstat ve tasklist kullanarak process ve port eşleşmesi
        let output = Command::new("cmd")
            .args(&["/C", &format!("netstat -ano | findstr :{}", port)])
            .output()?;

        if !output.status.success() {
            println!("Port {} üzerinde çalışan süreç bulunamadı.", port);
            return Ok(());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(pid) = parts.last() {
                // Process adını kontrol et
                let tasklist_output = Command::new("cmd")
                    .args(&["/C", &format!("tasklist /FI \"PID eq {}\" /FO CSV /NH", pid)])
                    .output()?;

                let tasklist_str = String::from_utf8_lossy(&tasklist_output.stdout);
                if tasklist_str.to_lowercase().contains(&process_name.to_lowercase()) {
                    println!("'{}' programı port {} üzerinde çalışıyor (PID: {}), kapatılıyor...",
                             process_name, port, pid);
                    Command::new("taskkill")
                        .args(&["/F", "/PID", pid])
                        .output()?;
                    println!("Program başarıyla kapatıldı.");
                    return Ok(());
                }
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        // Unix sistemleri için lsof ve ps kullanarak process ve port eşleşmesi
        let output = Command::new("lsof")
            .args(&["-i", &format!("tcp:{}", port)])
            .output()?;

        if !output.status.success() {
            println!("Port {} üzerinde çalışan süreç bulunamadı.", port);
            return Ok(());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines().skip(1) { // Header'ı atla
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let command = parts[0];
                let pid = parts[1];

                if command.to_lowercase().contains(&process_name.to_lowercase()) {
                    println!("'{}' programı port {} üzerinde çalışıyor (PID: {}), kapatılıyor...",
                             process_name, port, pid);
                    Command::new("kill")
                        .args(&["-9", pid])
                        .output()?;
                    println!("Program başarıyla kapatıldı.");
                    return Ok(());
                }
            }
        }
    }

    println!("'{}' programı port {} üzerinde çalışırken bulunamadı.", process_name, port);
    Ok(())
}

pub async fn kill_process_by_name_and_port_async(process_name: &str, port: u16) -> io::Result<()> {
    let process_name = process_name.to_string(); // String'e çevir
    tokio::task::spawn_blocking(move || kill_process_by_name_and_port(&process_name, port)).await?
}

pub fn kill_port(port: u16) -> io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        // Windows için netstat kullanarak PID bulma
        let output = Command::new("cmd")
            .args(&["/C", &format!("netstat -ano | findstr :{}", port)])
            .output()?;

        if !output.status.success() {
            println!("Port {} üzerinde çalışan süreç bulunamadı.", port);
            return Ok(());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(pid) = parts.last() {
                println!("Port {} üzerinde çalışan süreç PID: {} kapatılıyor...", port, pid);
                Command::new("taskkill")
                    .args(&["/F", "/PID", pid])
                    .output()?;
                println!("Port {} kapatıldı.", port);
                return Ok(());
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        // lsof kullanarak PID bulma
        let output = Command::new("lsof")
            .args(&["-i", &format!("tcp:{}", port), "-t"])
            .output()?;

        if !output.status.success() {
            println!("Port {} üzerinde çalışan süreç bulunamadı.", port);
            return Ok(());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        for pid in output_str.lines() {
            let pid = pid.trim();
            if !pid.is_empty() {
                println!("Port {} üzerinde çalışan süreç PID: {} kapatılıyor...", port, pid);
                Command::new("kill")
                    .args(&["-9", pid])
                    .output()?;
                println!("Port {} kapatıldı.", port);
                return Ok(());
            }
        }
    }

    Ok(())
}
pub async fn kill_port_async(port: u16) -> io::Result<()> {
    tokio::task::spawn_blocking(move || kill_port(port)).await?
}