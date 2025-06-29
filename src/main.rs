#![cfg_attr(windows_subsystem, windows_subsystem = "windows")]
use encoding_rs::GBK;
use std::env::args;
use std::io::Read;
use std::os::windows::process::CommandExt;
use std::process::Command;

fn gbk_to_utf8(bytes: &[u8]) -> String {
    let (cow, _encoding, _had_errors) = GBK.decode(bytes);
    cow.to_string()
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().skip(1).collect();
    if args.is_empty() {
        eprintln!("No command provided. Usage: <command> [args...]");
        std::process::exit(1);
    }
    println!("Running command: {:?}", args);
    let (mut recv, send) = std::io::pipe()?;

    let mut command = Command::new("cmd")
        .creation_flags(0x08000000)
        .arg("/C")
        .args(&args)
        .stdout(send.try_clone()?)
        .stderr(send)
        .spawn()?;

    // Stream output incrementally
    let mut buffer = [0; 1024];
    loop {
        match recv.read(&mut buffer) {
            Ok(0) => break, // End of stream
            Ok(n) => {
                let chunk = &buffer[..n];
                print!("{}", gbk_to_utf8(chunk));
            }
            Err(e) => return Err(e),
        }
    }

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
