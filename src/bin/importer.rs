extern crate clap;
extern crate fritzbox_logs;
extern crate bincode;
#[macro_use]
extern crate log;
extern crate xz2;
extern crate chrono;

use bincode::{serialize_into, deserialize_from, Infinite};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use xz2::read::XzDecoder;
use xz2::write::XzEncoder;
use chrono::Local;

fn build_cli() -> clap::App<'static, 'static> {
    use clap::{App, Arg};
    App::new("Fritz!Box logs import")
        .about(
            "A tool to convert textual fritz!box logs into a structured DB.",
        )
        .arg(
            Arg::with_name("LOGS")
                .long("logs")
                .value_name("path")
                .takes_value(true)
                .required(true)
                .help("Path to the log file"),
        )
        .arg(
            Arg::with_name("DB_PATH")
                .long("db-path")
                .value_name("path")
                .takes_value(true)
                .required(true)
                .help("Path to the database"),
        )
        .arg(
            Arg::with_name("COMPRESSION")
                .long("compression-level")
                .value_name("number")
                .takes_value(true)
                .required(false)
                .default_value("6")
                .help("Compression level (1-9)"),
        )
}

fn load_db(path: &str) -> Result<BTreeMap<i64, BTreeSet<fritzbox_logs::Entry>>, Box<Error>> {
    let mut f = match File::open(path) {
        Ok(f) => XzDecoder::new(f),
        Err(_) => {
            info!["Can't open file {}, initializing a new DB", path];
            return Ok(BTreeMap::new());
        }
    };
    let db = deserialize_from(&mut f, Infinite)?;
    Ok(db)
}

fn save_db(
    db: BTreeMap<i64, BTreeSet<fritzbox_logs::Entry>>,
    path: &str,
    compression_level: u32,
) -> Result<(), Box<Error>> {
    let mut f = XzEncoder::new(File::create(path)?, compression_level);
    serialize_into(&mut f, &db, Infinite)?;
    Ok(())
}

fn run() -> Result<(), Box<Error>> {
    let matches = build_cli().get_matches();
    let logs_path = matches.value_of("LOGS").ok_or("Can't get logs path")?;
    let db_path = matches.value_of("DB_PATH").ok_or("Can't get db path")?;
    let compression_level: u32 = matches
        .value_of("COMPRESSION")
        .ok_or("Can't get compression level")?
        .parse()?;

    let mut db = load_db(db_path)?;

    for log_entry in fritzbox_logs::parse(BufReader::new(File::open(logs_path)?), Local)? {
        let entry_set = db.entry(log_entry.timestamp).or_insert(BTreeSet::new());
        entry_set.insert(log_entry);
    }

    save_db(db, db_path, compression_level)?;
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => println!["Caught an error: {}", e],
    }
}
