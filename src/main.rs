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
    let mut base = if cfg!(feature = "cmd") {
        let mut cmd = Command::new("cmd");
        cmd.creation_flags(0x08000000).arg("/C");
        cmd
    } else {
        let mut cmd = Command::new("powershell");
        cmd.creation_flags(0x08000000)
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-Command");
        cmd
    };

    let mut command = base
        .args(&args)
        .stdout(send.try_clone()?)
        .stderr(send)
        .spawn()?;

    // Spawn a thread to read the output from the pipe
    // because in powershell mode, the EOF is not sent, and then the recv.read is blocked.
    if !cfg!(feature = "no-console") {
        let _output_handle = std::thread::spawn(move || {
            let mut buffer = [0; 1024];
            loop {
                match recv.read(&mut buffer) {
                    Ok(0) => {
                        // End of stream, break the loop
                        println!("\nEnd of output stream.");
                        break;
                    } // End of stream
                    Ok(n) => {
                        let chunk = &buffer[..n];
                        print!("{}", gbk_to_utf8(chunk));
                    }
                    Err(_) => break,
                }
            }
        });
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
