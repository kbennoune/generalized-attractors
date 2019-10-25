#![feature(non_ascii_idents)]

// https://softologyblog.wordpress.com/tag/attractors/
extern crate chrono;
extern crate clap;
use clap::{Arg, App, SubCommand};

use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::{UNIX_EPOCH, SystemTime};

use chrono::offset::Utc;
use chrono::DateTime;

fn sin(arg: f64) -> f64 {
    arg.sin()
}

fn cos(arg: f64) -> f64 {
    arg.cos()
}

fn parse_command_line() -> (Configuration, Iteration) {
    let matches = App::new("Attractor Calculator")
                    .version(env!("CARGO_PKG_VERSION"))
                    .about("Calculates attractors")
                    .arg(Arg::with_name("a")
                                .required(true)
                                .short("a")
                                .long("a_value")
                                .value_name("FLOAT")
                                .help("a parameter")
                                .takes_value(true))
                    .arg(Arg::with_name("b")
                                .required(true)
                                .short("b")
                                .long("b_value")
                                .value_name("FLOAT")
                                .help("b parameter")
                                .takes_value(true))
                    .arg(Arg::with_name("c")
                                .required(true)
                                .short("c")
                                .long("c_value")
                                .value_name("FLOAT")
                                .help("c parameter")
                                .takes_value(true))
                    .arg(Arg::with_name("d")
                                .required(true)
                                .short("d")
                                .long("d_value")
                                .value_name("FLOAT")
                                .help("d parameter")
                                .takes_value(true))
                    .arg(Arg::with_name("x_init")
                                .required(true)
                                .short("x")
                                .long("x_init")
                                .value_name("FLOAT")
                                .help("Initial x value")
                                .takes_value(true))
                    .arg(Arg::with_name("y_init")
                                .required(true)
                                .short("y")
                                .long("y_init")
                                .value_name("FLOAT")
                                .help("Initial y value")
                                .takes_value(true))                                
                    .arg(Arg::with_name("iterations")
                                .required(true)
                                .short("n")
                                .long("iterations")
                                .value_name("INTEGER")
                                .help("number of iterations")
                                .takes_value(true))
                    .get_matches();

    let a: f64 = matches.value_of("a").unwrap().parse().unwrap();
    let b: f64 = matches.value_of("b").unwrap().parse().unwrap();
    let c: f64 = matches.value_of("c").unwrap().parse().unwrap();
    let d: f64 = matches.value_of("d").unwrap().parse().unwrap();
    let x0: f64 = matches.value_of("x_init").unwrap().parse().unwrap();
    let y0: f64 = matches.value_of("y_init").unwrap().parse().unwrap();
    let n: i32 = matches.value_of("iterations").unwrap().parse().unwrap();

    (
        Configuration {a: a, b: b, c: c, d: d, x_init: x0, y_init: y0, iterations: n},
        Iteration { x: x0, y: y0, t: 0 }
    )
}

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

pub struct Configuration {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    x_init: f64,
    y_init: f64,
    iterations: i32
}

pub struct Calculation {
    configuration: Configuration,
    iteration: Iteration,
    file: std::fs::File
}

impl Calculation {
    fn run_iterations(&mut self) {
        let Configuration {a: _, b: _, c: _, d: _, x_init: _, y_init: _, iterations} = self.configuration;
        
        while self.iteration.t < iterations {
            self.save_iteration();
            self.iteration = self.next_iteration();
        }
    }

    fn save_iteration(&self) {
        let Iteration { t, x, y } = self.iteration;
        write!(&self.file, "{t} {x} {y}\n", t = t, x = x, y = y);
    }

    fn next_iteration(&self) -> Iteration {
        let Configuration {a, b, c, d, x_init: _, y_init: _, iterations: _} = self.configuration;
        let Iteration {t: t_last, x: x_last, y: y_last} = self.iteration;

        let t = t_last + 1;
        let x = sin(b * y_last) + c * sin(b * x_last);
        let y = sin(a * x_last) + d * sin(a * y_last);

        Iteration { x: x, y: y, t: t }
    }
}

#[derive(Debug, Copy, Clone)]
struct Iteration {
    x: f64,
    y: f64,
    t: i32
}
