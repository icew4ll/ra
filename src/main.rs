extern crate csv;
// extern crate ssh2;

// use std::net::TcpStream;
// use ssh2::Session;
use std::error::Error;
use std::fs::File;
use std::process;

type Record = (String, String, String);

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
    let mut conn = Vec::new();
    // let mut out = Vec::new();
    if let Err(err) = run(&mut conn) {
        println!("{}", err);
        process::exit(1);
    }

    println!("{:?}", conn);
    println!("{:?}", conn[0].1);
}
