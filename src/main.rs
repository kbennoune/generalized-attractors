// https://softologyblog.wordpress.com/tag/attractors/
extern crate chrono;
extern crate clap;

use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::{UNIX_EPOCH, SystemTime};

use chrono::offset::Utc;
use chrono::DateTime;

mod attractor;
use attractor::iteration::Iteration;
use attractor::calculation::Calculation;
use attractor::configuration::Configuration;
mod command_line;
use command_line::parse_command_line;

fn main() {          
    let now = SystemTime::now();
    let now_secs = now.duration_since(UNIX_EPOCH).expect("BROKEN").as_secs();
    let (configuration, initial) = parse_command_line();

    let mut file = init_file(&configuration, now);

    let mut calculation = Calculation { configuration: configuration, iteration: initial, file: file };

    calculation.run_iterations();

    let end = SystemTime::now().duration_since(UNIX_EPOCH).expect("BROKEN").as_secs();

    println!("Done in {} secs", end - now_secs);
}

fn init_file(configuration: &Configuration, now: SystemTime) -> std::fs::File {
    let datetime: DateTime<Utc> = now.into();

    let filename = format!("data/pickover.a:{}.b:{}.c:{}.d:{}.n:{}.{}.dat", configuration.a, configuration.b, configuration.c, configuration.d, configuration.iterations, datetime.format("%m-%d-%Y-%T"));
    fs::create_dir_all("data");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();

    write!(file, "t x y\n");
    file
}