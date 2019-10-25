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

    let mut calculation = Calculation { configuration: configuration, iteration: initial };
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
    iteration: Iteration
}

impl Calculation {
    fn run_iterations(&mut self, mut file: std::fs::File) {
        let Configuration {a: _, b: _, c: _, d: _, iterations: iterations} = self.configuration;
        
        while self.iteration.t < iterations {
            self.save_iteration(&self.iteration, &mut file);
            self.iteration = self.next_iteration();
        }
    }

    fn save_iteration(&self, i: &Iteration, file: &mut std::fs::File) {
        write!(file, "{t} {x} {y}\n", t = i.t, x = i.x, y = i.y);
    }

    fn next_iteration(&self) -> Iteration {
        let Configuration {a, b, c, d, iterations: _} = self.configuration;
        let Iteration {t: t_last, x: x_last, y: y_last} = self.iteration;

        let t = t_last + 1;
        let x = sin(b * y_last) + c * sin(b * x_last);
        let y = sin(a * x_last) + d * sin(a * y_last);

        Iteration { x: x, y: y, t: t }
    }
}
