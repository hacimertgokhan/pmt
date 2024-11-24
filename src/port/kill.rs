use std::io;
use std::process::Command;

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