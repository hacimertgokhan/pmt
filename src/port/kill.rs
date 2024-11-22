use std::process::Command;

pub fn kill_process_using_port(port: u16) -> Result<(), String> {
    let output = Command::new("lsof")
        .arg("-t")
        .arg(format!("-i:{}", port))
        .output()
        .map_err(|e| format!("lsof komutu çalıştırılamadı: {}", e))?;
    if output.status.success() {
        let pids = std::str::from_utf8(&output.stdout)
            .map_err(|e| format!("Çıktı dönüştürülemedi: {}", e))?;
        for pid in pids.lines() {
            let pid: i32 = pid.trim().parse().map_err(|e| format!("PID dönüştürülemedi: {}", e))?;
            let _ = Command::new("kill").arg("-9").arg(pid.to_string()).output();
            println!("Port {} kullanan süreç {} kapatıldı.", port, pid);
        }
        Ok(())
    } else {
        Err(format!("Port {} için süreç bulunamadı.", port))
    }
}