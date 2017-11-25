extern crate clap;
extern crate fritzbox_logs;
extern crate bincode;
// #[macro_use]
// extern crate log;
extern crate xz2;
extern crate chrono;

use bincode::{deserialize_from, Infinite};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use xz2::read::XzDecoder;
use chrono::{Local, TimeZone, NaiveDateTime};

fn build_cli() -> clap::App<'static, 'static> {
    use clap::{App, Arg};
    App::new("Fritz!Box logs export")
        .about("A tool to convert a structured DB-logs into a textual form")
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
}

fn load_db(path: &str) -> Result<BTreeMap<i64, BTreeSet<fritzbox_logs::Entry>>, Box<Error>> {
    let mut f = XzDecoder::new(File::open(path)?);
    let db = deserialize_from(&mut f, Infinite)?;
    Ok(db)
}

fn save_text_file(
    db: BTreeMap<i64, BTreeSet<fritzbox_logs::Entry>>,
    path: &str,
) -> Result<(), Box<Error>> {
    let mut f = BufWriter::new(File::create(path)?);
    for (_, entries) in db {
        for entry in entries {
            let timestamp =
                Local.from_utc_datetime(&NaiveDateTime::from_timestamp(entry.timestamp, 0u32));
            writeln![
                f,
                "{} {}",
                timestamp.format("%d.%m.%y %H:%M:%S"),
                entry.message,
            ]?;
        }
    }
    Ok(())
}

fn run() -> Result<(), Box<Error>> {
    let matches = build_cli().get_matches();
    let logs_path = matches.value_of("LOGS").ok_or("Can't get logs path")?;
    let db_path = matches.value_of("DB_PATH").ok_or("Can't get db path")?;

    let db = load_db(db_path)?;
    save_text_file(db, logs_path)?;
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => println!["Caught an error: {}", e],
    }
}
