// extern crate csv;
extern crate colored;
extern crate csv;
extern crate subprocess;
extern crate ssl_expiration;

use ssl_expiration::SslExpiration;
use colored::*;
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
    let mut v2 = Vec::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        v.push(record);
    }

    for i in v {
        // reference i to iterate
        let t = &i[0];
        let out = Exec::cmd("touch")
            .arg(t)
            .cwd("./test")
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .capture()?
            .stdout_str();
        println!("Made {} {}", &i[0].red(), out);
    }
    // ls directory
    let lsdir = "./test";
    let ls = Exec::cmd("ls")
        .cwd(lsdir)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()?
        .stdout_str();
    // push to vector
    v2.push(ls);
    // split on new line
    let lssplit = v2[0].split("\n");
    let mut lsparse: Vec<&str> = lssplit.collect();
    // remove last line
    let length = lsparse.len() - 1;
    println!("{} has {} items", lsdir, length);
    lsparse.truncate(length);
    println!("[ {}]", lsparse.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + " ").on_bright_blue());
    println!("{:?}", lsparse);

    // check ssl expiration
    // ssl-expiration google.com
    // cargo install ssl-expiration
    let expiration = SslExpiration::from_domain_name("webltw19.alpha-lt.net").unwrap();
    println!("{:?}", expiration.days());
    Ok(())
}
