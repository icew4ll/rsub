extern crate subprocess;

use subprocess::{Exec, Redirection};

fn main() {
    let out = Exec::cmd("ssh")
        .arg("cloudssh.us-icewall@191.101.227.135")
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture().expect("failed to execute")
        .stdout_str();
    println!("{}", out);
    let out = Exec::cmd("ls")
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture().expect("failed to execute")
        .stdout_str();
    println!("{}", out);
    let out = Exec::cmd("ls")
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture().expect("failed to execute")
        .stdout_str();
    println!("{}", out);
}
