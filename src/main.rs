extern crate csv;
extern crate ssh2;

use std::io::prelude::*;
use std::net::TcpStream;
use ssh2::Session;
use std::error::Error;
use std::fs::File;
use std::process;

type Record = (String, String, String);

fn login(ip: String, user: String, pass: String, output: &mut Vec<(String, String, String, String, String, String, String, String)>) {
    let cmds = vec!["uname -n", "hostname -i", "cat /etc/*release", "httpd -v", "php -i", "perl -v | awk '/This/ {print $4}' | sed -e 's/v//'", "mysql -V", "openssl version"];
    let mut data = Vec::new();
    let tcp = TcpStream::connect(ip).unwrap();
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();
    sess.userauth_password(&user, &pass).unwrap();
    assert!(sess.authenticated());

    for i in &cmds {
        // println!("{}", i)
        let mut channel = sess.channel_session().unwrap();
        channel.exec(i).unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        channel.wait_close().expect("Could not ls");
        data.push(s.clone().replace("\n", ""));
    }
    let dname = format!("{}", data[0]);
    let dip = format!("{}", data[1]);
    let ddist = format!("{}", data[2]);
    let dhttpd = format!("{}", data[3]);
    let dphp = format!("{}", data[4]);
    let dperl = format!("{}", data[5]);
    let dsql = format!("{}", data[6]);
    let dssl = format!("{}", data[7]);
    output.push((dname, dip, ddist, dhttpd, dphp, dperl, dsql, dssl));
    // println!("{}", data[0]);
    // DOUBLE
    // let mut channel = sess.channel_session().unwrap();
    // channel.exec("hostname -i").unwrap();
    // let mut sip = String::new();
    // channel.read_to_string(&mut sip).unwrap();
    // channel.wait_close().expect("Could not ls");
    // output.push((sname.clone().replace("\n", ""), sip.clone().replace("\n", "")));
}

fn run(conn: &mut Vec<Record>) -> Result<(), Box<Error>> {
    let file = File::open("conn.csv")?;
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
    for result in rdr.deserialize() {
        let record: Record = result?;
        // println!("{:?}", record);
        conn.push(record);
    }
    Ok(())
}

fn main() {
    // setup vars
    let mut conn = Vec::new();
    let mut output = Vec::new();
    // parse csv
    if let Err(err) = run(&mut conn) {
        println!("{}", err);
        process::exit(1);
    }
    for i in 0..conn.len() {
        let ip = format!("{}:22", &conn[i].0).as_str().to_owned();
        let user = conn[i].1.as_str().to_owned();
        let pass = conn[i].2.as_str().to_owned();
       // println!("{:?}", conn[i])
        login(ip, user, pass, &mut output);
    }
    // println!("{}", ip);
    // println!("{}", user);
    // println!("{}", pass);
    // login(ip, user, pass, &mut output);
    println!("{:?}", output);
}
