#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{}
}

use std::process::{Command, Stdio};
use errors::*;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}
#[allow(dead_code)]
fn alternative_main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        use error_chain::ChainedError;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "{}", e.display_chain()).expect(errmsg);
        ::std::process::exit(1);
    }
}
fn run() -> Result<()> {
    use std::fs::File;
    // File::open("tretrete").chain_err(|| "unable to open tretrete file")?;
    let outputs = File::create("out.txt").chain_err(|| "unable to wait for output")?;
    let errors = outputs.try_clone().chain_err(|| "unable to wait for output")?;

    let cmd = Command::new("mkdir")
        .args(&["test"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        //.chain_err necessary for error chains compilation
        .spawn().chain_err(|| "spawn failed")?
        .wait_with_output().chain_err(|| "wait for output failed")?;
    println!("{:?}", cmd);

    let ctouch = Command::new("touch")
        .args(&["test.txt"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        //.chain_err necessary for error chains compilation
        .spawn().chain_err(|| "spawn failed")?
        .wait_with_output().chain_err(|| "wait for output failed")?;
    println!("{:?}", ctouch);


    Ok(())
}
