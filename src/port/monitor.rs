use std::{io, time::Duration};
use reqwest;
use std::process::Command;
use tokio::net::TcpStream;
use tokio::time::sleep;

#[derive(Clone)] // Clone trait'ini ekle
pub struct PortMonitor {
    port: u16,
    check_interval: Duration,
}

impl PortMonitor {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            check_interval: Duration::from_secs(5),
        }
    }

    pub fn set_check_interval(&mut self, seconds: u64) {
        self.check_interval = Duration::from_secs(seconds);
    }

    async fn check_port_status(&self) -> bool {
        if let Ok(_) = TcpStream::connect(format!("127.0.0.1:{}", self.port)).await {
            true
        } else {
            false
        }
    }

    async fn check_http_status(&self) -> Result<String, reqwest::Error> {
        let url = format!("http://localhost:{}", self.port);
        let response = reqwest::get(&url).await?;
        Ok(format!("Status: {}", response.status()))
    }

    pub fn kill_port(&self) -> io::Result<()> {
        #[cfg(target_os = "windows")]
        {
            let output = Command::new("cmd")
                .args(&["/C", &format!("netstat -ano | findstr :{}", self.port)])
                .output()?;

            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(pid) = parts.last() {
                    println!("Next.js process (PID: {}) kapatılıyor...", pid);
                    Command::new("taskkill")
                        .args(&["/F", "/PID", pid])
                        .output()?;
                    println!("Port {} kapatıldı!", self.port);
                    return Ok(());
                }
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            let output = Command::new("lsof")
                .args(&["-i", &format!("tcp:{}", self.port), "-t"])
                .output()?;

            let output_str = String::from_utf8_lossy(&output.stdout);
            for pid in output_str.lines() {
                let pid = pid.trim();
                if !pid.is_empty() {
                    println!("Next.js process (PID: {}) kapatılıyor...", pid);
                    Command::new("kill")
                        .args(&["-9", pid])
                        .output()?;
                    println!("Port {} kapatıldı!", self.port);
                    return Ok(());
                }
            }
        }

        println!("Port {} üzerinde çalışan process bulunamadı.", self.port);
        Ok(())
    }

    pub async fn monitor(&self) -> io::Result<()> {
        println!("Port {} izleniyor...", self.port);

        loop {
            let is_active = self.check_port_status().await;

            if is_active {
                match self.check_http_status().await {
                    Ok(status) => println!("Port {} aktif - {}", self.port, status),
                    Err(e) => println!("Port {} aktif fakat HTTP hatası: {}", self.port, e),
                }
            } else {
                println!("Port {} inactive", self.port);
            }

            sleep(self.check_interval).await;
        }
    }
}

#[tokio::main]
pub async fn run_monitor() -> io::Result<()> {
    let mut monitor = PortMonitor::new(3000);
    monitor.set_check_interval(2);
    let monitor_clone = monitor.clone();
    tokio::spawn(async move {
        if let Ok(_) = tokio::signal::ctrl_c().await {
            println!("\nCtrl+C algılandı, port kapatılıyor...");
            if let Err(e) = monitor_clone.kill_port() {
                eprintln!("Port kapatılırken hata: {}", e);
            }
            std::process::exit(0);
        }
    });

    monitor.monitor().await
}