extern crate getopts;
extern crate hdrhistogram;

use getopts::Options;
use std::env;
use std::io::{self, BufRead};
use regex::Regex;
use hdrhistogram::Histogram;
use std::vec::Vec;

mod params;

fn help(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    eprint!("{}", opts.usage(&brief));
    std::process::exit(1);
}

fn get_args(p: &mut params::Params) {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optopt("m", "min", "set minimum value. Default is 1.", "<number>");
    opts.optopt("M", "max", "set maximum value. Default is 10000000000.", "<number>");
    opts.optflag("h", "help", "print this help");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            eprintln!("{} use -h,--help to display help screen", f.to_string());
            std::process::exit(1);
        }
    };

    if matches.opt_present("h") {
        help(&program, opts);
    }

    let parse_number = |opt: &str, dval: u64| {
        match matches.opt_str(opt) {
            Some(m) => {
                match m.parse::<u64>() {
                    Ok(m) => { m }
                    Err(f) => {
                        eprintln!("Invalid value: {}", f);
                        std::process::exit(1);
                    }
                }
            }
            None => { dval }
        }
    };

    p.min = parse_number("m", 1);
    p.max = parse_number("M", 10000000000);

    eprintln!("{}", p);
}

fn run(p: &params::Params) {
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

    let p_vector: Vec<f64> = vec![ 5.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 99.0, 100.0 ];

    print!("{{");

    let mut first = true;

    for p in &p_vector {
        if first {
            first = false;
            print!("\"data\":[");
        } else {
            print!("\n,");
        }

        print!("[{:.2},{:.3}]", p, histogram.value_at_quantile(*p / 100.0)  as f64 / 1000.0);
    }

    println!("]");

    println!(",\"stdDev\": {:.3}", histogram.stdev() / 1000.0);
    println!(",\"mean\": {:.3}", histogram.mean() / 1000.0);
    println!(",\"min\": {:.3}", histogram.min() as f64 / 1000.0);
    println!(",\"max\": {:.3}", histogram.max() as f64 / 1000.0);
    println!(",\"count\": {:.3}", histogram.len());
    println!("}}");
}

fn main() -> Result<(), ()> {
    let mut p = params::Params {
        min: 0,
        max: 0,
    };

    get_args(&mut p);
    run(&p);

    Ok(())
}
