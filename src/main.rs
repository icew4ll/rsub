// extern crate csv;
extern crate csv;
extern crate subprocess;

use std::error::Error;
use std::io;
use std::process;

use subprocess::{Exec, Redirection};

fn main() {
    // test function
    if let Err(err) = test() {
        println!("{}", err);
        process::exit(1);
    }
}

fn test() -> Result<(), Box<Error>> {
    let mut v = Vec::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        v.push(record);
    }

    for i in v {
        // println!("{:?}", i[0]);
        let t = &i[0];
        let out = Exec::cmd("touch")
            .arg(t)
            .cwd("./test")
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .capture()?
            .stdout_str();
        println!("Made {} {}", &i[0], out);
    }
    Ok(())
}
