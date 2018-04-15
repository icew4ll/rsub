extern crate ssh2;
extern crate colored;
extern crate csv;
extern crate regex;

use regex::Regex;
use std::fs::File;
use std::process;
use std::net::TcpStream;
use ssh2::Session;
use std::io::Read;
use std::error::Error;
use colored::*;

type Record = (String, String, String);

fn csvread(conn: &mut Vec<Record>) -> Result<(), Box<Error>> {
    let file = File::open("conn.csv")?;
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
    for result in rdr.deserialize() {
        let record: Record = result?;
        conn.push(record);
    }
    Ok(())
}

fn ssh(conn: &mut Vec<Record>, mailq: &mut Vec<(String)>) -> Result<(), Box<Error>> {
    let ip = format!("{}:22", &conn[0].0);
    let user = &conn[0].1;
    let pass = &conn[0].2;
    let tcp = TcpStream::connect(ip)?;
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp)?;
    sess.userauth_password(user, pass)?;
    assert!(sess.authenticated());


    let mut channel = sess.channel_session()?;
    channel.exec("mailq | grep Apr | awk '{print $7}' | sort | uniq -c | sort -n")?;
    let mut s = String::new();
    channel.read_to_string(&mut s)?;
    println!("{}", s.red());
    mailq.push(s.clone());
    Ok(())
}

fn main() {
    // init data vector
    let mut conn = Vec::new();
    let mut mailq = Vec::new();

    // csv read function
    if let Err(err) = csvread(&mut conn) {
        println!("{}", err);
        process::exit(1);
    }

    // ssh function
    if let Err(err) = ssh(&mut conn, &mut mailq) {
        println!("{}", err);
            process::exit(1);
    }
    println!("{:?}", mailq);
}
