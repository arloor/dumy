#![cfg_attr(windows_subsystem, windows_subsystem = "windows")]
mod io;
use std::env::args;
use std::os::windows::process::CommandExt;
use std::process::Command;

use crate::io::{consume, init_log};

fn main() -> std::io::Result<()> {
    init_log();
    let args: Vec<String> = args().skip(1).collect();
    if args.is_empty() {
        eprintln!("No command provided. Usage: <command> [args...]");
        std::process::exit(1);
    }
    println!("Running command: {:?}", args);
    let (recv, send) = std::io::pipe()?;

    let mut command = Command::new("cmd")
        .creation_flags(0x08000000)
        .arg("/C")
        .args(&args)
        .stdout(send.try_clone()?)
        .stderr(send)
        .spawn()?;

    consume(recv);

    // It's important that we read from the pipe before the process exits, to avoid
    // filling the OS buffers if the program emits too much output.
    if !command.wait()?.success() {
        eprintln!("Command failed",);
        std::process::exit(1);
    } else {
        eprintln!("Command succeeded",);
    }
    Ok(())
}
