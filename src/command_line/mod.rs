use clap::{Arg, App};

use crate::attractor::iteration::Iteration;
use crate::attractor::configuration::Configuration;

pub fn parse_command_line() -> (Configuration, Iteration) {
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