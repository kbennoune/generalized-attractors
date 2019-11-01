use std::io::prelude::*;

use crate::attractor::configuration::Configuration;
use crate::attractor::iteration::Iteration;

fn sin(arg: f64) -> f64 {
    arg.sin()
}

fn cos(arg: f64) -> f64 {
    arg.cos()
}

pub struct Calculation {
    pub configuration: Configuration,
    pub iteration: Iteration,
    pub file: std::fs::File
}

impl Calculation {
    pub fn run_iterations(&mut self) {
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