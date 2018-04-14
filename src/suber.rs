// extern crate csv;
extern crate subprocess;

use std::error::Error;
use std::process;

use subprocess::{Exec, Redirection};

fn main() {
    // test function
    let t = &["t1", "t2"];
    println!("{:?}", t);

    if let Err(err) = test() {
        println!("{}", err);
        process::exit(1);
    }
}

fn test() -> Result<(), Box<Error>> {
    let out = Exec::cmd("touch")
        .arg("test")
        .cwd("./test")
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()?
        .stdout_str();
    println!("{}", out);
    Ok(())
}
