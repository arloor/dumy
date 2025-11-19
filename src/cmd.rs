#![cfg_attr(windows_subsystem, windows_subsystem = "windows")]
mod io;
use log::{info, warn};
use std::env::args;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::{Arc, Mutex};

use crate::io::{consume, init_log};

fn taskkill_pid(pid: u32) {
    let _ = Command::new("taskkill")
        .arg("/PID")
        .arg(pid.to_string())
        .arg("/T")
        .arg("/F")
        .creation_flags(0x08000000)
        .spawn();
}

fn main() -> std::io::Result<()> {
    init_log();
    let args: Vec<String> = args().skip(1).collect();
    if args.is_empty() {
        warn!("No command provided. Usage: <command> [args...]");
        std::process::exit(1);
    }
    info!("Running command: {:?}", args);

    // Shared storage for the child PID so the signal handler can access it.
    let child_pid: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(None));

    // Register Ctrl-C / termination handler.
    {
        let pid_handle = child_pid.clone();
        ctrlc::set_handler(move || {
            if let Some(pid) = *pid_handle.lock().unwrap() {
                info!("Received exit signal; killing child pid {}", pid);
                taskkill_pid(pid);
            }
        })
        .expect("Error setting Ctrl-C handler");
    }

    let (recv, send) = std::io::pipe()?;

    let mut command = Command::new("cmd")
        .creation_flags(0x08000000)
        .arg("/C")
        .args(&args)
        .stdout(send.try_clone()?)
        .stderr(send)
        .spawn()?;

    // Store pid for handler
    {
        let pid = command.id();
        *child_pid.lock().unwrap() = Some(pid);
    }

    consume(recv);

    // It's important that we read from the pipe before the process exits, to avoid
    // filling the OS buffers if the program emits too much output.
    let status = command.wait()?;

    // Clear stored pid — process has exited.
    *child_pid.lock().unwrap() = None;

    if !status.success() {
        warn!("Command failed",);
        std::process::exit(1);
    } else {
        info!("Command succeeded",);
    }
    Ok(())
}
