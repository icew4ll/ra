extern crate ssh2;
use std::io::prelude::*;
use std::fs::File;
use std::net::TcpStream;
use ssh2::Session;
// use regex::Regex;

fn main() {
    //file
    let mut file = File::open("conn.csv").expect("Can't open file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Oops! Can not read the file...");
    println!("File Contents:\n\n{}", contents);

    // ssh
    let mut results = Vec::new();
    login(":22", &mut results);
    login(":22", &mut results);
    login(":22", &mut results);
    login(":22", &mut results);
    println!("{:?}", results)
}

fn login(ip: &'static str, results: &mut Vec<String>) {
    // let mut model = Vec::new();
    let tcp = TcpStream::connect(ip).unwrap();
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();
    sess.userauth_password("root", "pass").unwrap();
    assert!(sess.authenticated());
    // start channel
    let mut channel = sess.channel_session().unwrap();
    channel.exec("uname -n").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    channel.wait_close().expect("Could not ls");
    results.push(s.clone().replace("\n", ""));
    // DOUBLE
    let mut channel = sess.channel_session().unwrap();
    channel.exec("uname -n").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    channel.wait_close().expect("Could not ls");
    results.push(s.clone().replace("\n", ""));

}
