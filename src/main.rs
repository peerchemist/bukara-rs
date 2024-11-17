use std::{env, error::Error, future::pending, process::{Child, Command, Stdio}, sync::Mutex};
use zbus::{connection, interface};
use notify_rust::Notification;
use lazy_static::lazy_static;
use chrono::{Local, Datelike};
use log::{info, error, warn};

lazy_static! {
    static ref RECORD_PROCESS: Mutex<Option<Child>> = Mutex::new(None);
}

struct Handler;

#[interface(name = "com.github.peerchemist.Bukara1")]
impl Handler {
    fn record(&self) {
        info!("Record method called.");

        // Get $HOME/Downloads directory
        let home_dir = env::var("HOME").expect("$HOME not set");
        let downloads_dir = format!("{}/Downloads", home_dir);

        // Generate filename based on the current timestamp (sans year)
        let now = Local::now();
        let filename = format!("{}/{}-{}-{}-recording.wav", downloads_dir, now.month(), now.day(), now.format("%H-%M-%S"));

        // Start pw-record subprocess
        let mut process_lock = RECORD_PROCESS.lock().unwrap();
        if process_lock.is_none() {
            match Command::new("pw-record")
                .arg(&filename)
                .arg("-P")
                .arg("{ stream.capture.sink=true }")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                Ok(child) => {
                    *process_lock = Some(child);
                    info!("pw-record started with filename: {}", filename);
                    // Show notification after successful start
                    if let Err(e) = Notification::new()
                        .summary("Signal Notification")
                        .body(&format!("Recording started: {}", filename))
                        .icon("dialog-information")
                        .show()
                    {
                        error!("Failed to show notification: {}", e);
                    }
                }
                Err(e) => {
                    error!("Failed to start pw-record: {}", e);
                }
            }
        } else {
            warn!("pw-record is already running.");
        }
    }

    fn stop(&self) {
        info!("Stop method called.");

        // Stop pw-record subprocess
        let mut process_lock = RECORD_PROCESS.lock().unwrap();
        if let Some(mut child) = process_lock.take() {
            match child.kill() {
                Ok(_) => {
                    info!("pw-record stopped.");
                    // Show notification after successful stop
                    if let Err(e) = Notification::new()
                        .summary("Signal Notification")
                        .body("Recording stopped.")
                        .icon("dialog-information")
                        .show()
                    {
                        error!("Failed to show notification: {}", e);
                    }
                }
                Err(e) => error!("Failed to stop pw-record: {}", e),
            }
        } else {
            warn!("pw-record is not running.");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize env_logger for systemd-compatible logging
    env_logger::builder().format_timestamp_secs().init();
    info!("Starting service...");
    let handler = Handler;

    let _conn = connection::Builder::session()?
        .name("com.github.peerchemist.Bukara1")?
        .serve_at("/com/github/peerchemist/Bukara1", handler)?
        .build()
        .await?;

    info!("Service registered as 'com.github.peerchemist.Bukara1' on '/com/github/peerchemist/Bukara1'.");

    // Keep the service alive
    pending::<()>().await;

    Ok(())
}
