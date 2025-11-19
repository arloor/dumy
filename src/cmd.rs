#![cfg_attr(all(windows, feature = "no-console"), windows_subsystem = "windows")]
mod io;
mod job;
use log::{info, warn};
use std::env::args;
use std::os::windows::process::CommandExt;
use std::process::Command;

use crate::io::{consume, init_log};
use crate::job::JobObject;

fn main() -> std::io::Result<()> {
    init_log();
    let args: Vec<String> = args().skip(1).collect();
    if args.is_empty() {
        warn!("No command provided. Usage: <command> [args...]");
        std::process::exit(1);
    }
    info!("Running command: {:?}", args);

    // Create a job object that will kill all child processes when it's dropped
    let job = JobObject::new()?;

    let (recv, send) = std::io::pipe()?;

    let mut command = Command::new("cmd")
        .creation_flags(0x08000000)
        .arg("/C")
        .args(&args)
        .stdout(send.try_clone()?)
        .stderr(send)
        .spawn()?;

    // Assign the child process to the job object
    let pid = command.id();
    job.assign_process(pid)?;

    consume(recv);

    // It's important that we read from the pipe before the process exits, to avoid
    // filling the OS buffers if the program emits too much output.
    let status = command.wait()?;

    if !status.success() {
        warn!("Command failed",);
        std::process::exit(1);
    } else {
        info!("Command succeeded",);
    }
    Ok(())
}
