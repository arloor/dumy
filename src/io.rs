use std::io::Read as _;

use encoding_rs::GBK;
use log::info;

pub(crate) fn init_log() {
    let _ = log_x::init_log("log", "dumy");
}

pub(crate) fn gbk_to_utf8(bytes: &[u8]) -> String {
    let (cow, _encoding, _had_errors) = GBK.decode(bytes);
    cow.to_string()
}

pub(crate) fn consume(mut recv: std::io::PipeReader) {
    // Spawn a thread to read the output from the pipe
    // because in powershell mode, the EOF is not sent, and then the recv.read is blocked.

    let _output_handle = std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match recv.read(&mut buffer) {
                Ok(0) => {
                    // End of stream, break the loop
                    info!("End of output stream.");
                    break;
                } // End of stream
                Ok(n) => {
                    let chunk = &buffer[..n];
                    let content = gbk_to_utf8(chunk);
                    info!("\n{content}");
                }
                Err(_) => break,
            }
        }
    });
}
