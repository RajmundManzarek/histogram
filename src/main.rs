extern crate getopts;
extern crate hdrhistogram;

use getopts::Options;
use std::env;
use std::io::{self, BufRead};
use regex::Regex;
use hdrhistogram::Histogram;

#[derive(Debug)]
struct Params {
    min: u64,
    max: u64,
}

fn help(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
    std::process::exit(1);
}

fn get_args(p: &mut Params) {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optopt("m", "min", "set minimum value. Default is 1.", "<number>");
    opts.optopt("M", "max", "set maximum value. Default is 10000000000.", "<number>");
    opts.optflag("h", "help", "print this help");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            println!("{} use -h,--help to display help screen", f.to_string());
            std::process::exit(1);
        }
    };

    if matches.opt_present("h") {
        help(&program, opts);
    }

    p.min = match matches.opt_str("m") {
        Some(m) => {
            match m.parse::<u64>() {
                Ok(m) => { m }
                Err(f) => {
                    println!("Invalid --min value: {}", f);
                    std::process::exit(1);
                }
            }
        }
        None => { 1 }
    };

    p.max = match matches.opt_str("M") {
        Some(m) => {
            match m.parse::<u64>() {
                Ok(m) => { m }
                Err(error) => {
                    println!("Invalid --max value: {}", error);
                    std::process::exit(1);
                }
            }
        }
        None => { 10000000000 }
    };
}

fn run(p: &Params) {
    let re = Regex::new(r"^\d+$").unwrap();
    let mut histogram = Histogram::<u64>::new_with_bounds(p.min, p.max, 3).unwrap();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();

        if re.is_match(&l) {
            match l.parse::<u64>() {
                Ok(m) => { histogram += m; }
                Err(_e) => {}
            }
        }
    }

    let mut perc = histogram.iter_quantiles(1);

    loop {
        match perc.next() {
            Some(p) => {
                if p.count_since_last_iteration() > 0 {
                    println!("{:.3},{:.6}", p.value_iterated_to() as f32 / 1000.0, p.quantile());
                }
                //println!("{:?}", p);
            }
            None => { break; }
        }
    }
}

fn main() -> Result<(), ()> {
    let mut p = Params {
        min: 0,
        max: 0,
    };

    get_args(&mut p);
    run(&p);

    Ok(())
}
