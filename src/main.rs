#![feature(non_ascii_idents)]

// https://softologyblog.wordpress.com/tag/attractors/

use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::{UNIX_EPOCH, SystemTime};

const A:f64 = -0.966918;
const B:f64 = 2.879879;
const C:f64 = 0.765145;
const D:f64 = 0.744728;

#[derive(Debug, Copy, Clone)]
struct Iteration {
    x: f64,
    y: f64,
    t: i32
}

fn sin(arg: f64) -> f64 {
    arg.sin()
}

fn cos(arg: f64) -> f64 {
    arg.cos()
}

fn main() {
    let configuration =  Configuration {a: A, b: B, c: C, d: D, iterations: 300};
    


    // let initial = Iteration{
    //     x: 0.1,
    //     y: 0.1,
    //     t: 0
    // };
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("BROKEN").as_secs();
    let filename = format!("data/pickover.a:{}-b:{}-c:{}-d:{}.{}.dat", A, B, C, D, now);
    fs::create_dir_all("data");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();

    write!(file, "t x y\n");


    let initial = Iteration{
        x: 0.1,
        y: 0.1,
        t: 0
    };

    let calculation = Calculation { configuration: configuration, initial: Iteration{
        x: 0.1,
        y: 0.1,
        t: 0
    }  };
    // run_iterations(initial, file);

    calculation.run_iterations(file);

    let end =  SystemTime::now().duration_since(UNIX_EPOCH).expect("BROKEN").as_secs();

    println!("Done in {} secs", end - now);
}

pub struct Configuration {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    iterations: i32
}

pub struct Calculation {
    configuration: Configuration,
    initial: Iteration
}

impl Calculation {
    fn run_iterations(&self, mut file: std::fs::File) {
        let mut iteration = self.initial;
        let Configuration {a: _, b: _, c: _, d: _, iterations: iterations} = self.configuration;
        
        while iteration.t < iterations {
            self.save_iteration(&iteration, &mut file);
            iteration = self.next_iteration(iteration);
        }
    }

    fn save_iteration(&self, i: &Iteration, file: &mut std::fs::File) {
        write!(file, "{t} {x} {y}\n", t = i.t, x = i.x, y = i.y);
    }

    fn next_iteration(&self, i: Iteration) -> Iteration {
        let Configuration {a: a, b: b, c: c, d: d, iterations: _} = self.configuration;

        let t = i.t + 1;
        let x = sin(b * i.y) + c * sin(b * i.x);
        let y = sin(a * i.x) + d * sin(a * i.y);

        Iteration { x: x, y: y, t: t }
    }
}
