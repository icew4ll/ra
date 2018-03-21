extern crate csv;
extern crate ssh2;

use std::io;
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
}

fn run(conn: &mut Vec<Record>) -> Result<(), Box<Error>> {
    let file = File::open("conn.csv")?;
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
    for result in rdr.deserialize() {
        let record: Record = result?;
        conn.push(record);
    }
    Ok(())
}

fn writer() -> Result<(), Box<Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    wtr.write_record(&["Name", "IP", "Distro", "Httpd", "PHP", "Perl", "MySql", "OpenSSL"])?;
    wtr.flush()?;
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
    // loop login on connections
    for i in &conn {
        let ip = format!("{}:22", i.0).as_str().to_owned();
        let user = i.1.as_str().to_owned();
        let pass = i.2.as_str().to_owned();
        login(ip, user, pass, &mut output);
    }
    // run csv writer
    if let Err(err) = writer() {
        println!("{}", err);
        process::exit(1);
    }
    // output to stdout as csv
    for i in &output {
        let name =  i.0.to_owned();
        let ip =  i.1.to_owned();
        let dist =  i.2.to_owned();
        let http =  i.3.to_owned();
        let php =  i.4.to_owned();
        let perl =  i.5.to_owned();
        let sql =  i.6.to_owned();
        let ssl =  i.7.to_owned();
        println!("{},{},{},{},{},{},{},{},", name, ip, dist, http, php, perl, sql, ssl);
    }
}
