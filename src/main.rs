extern crate getopts;
extern crate hdrhistogram;

use getopts::Options;
use std::env;
use std::io::{self, BufRead};
use regex::Regex;
use hdrhistogram::Histogram;
use std::vec::Vec;

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

    let parse_number = |opt: &str, dval: u64| {
        match matches.opt_str(opt) {
            Some(m) => {
                match m.parse::<u64>() {
                    Ok(m) => { m }
                    Err(f) => {
                        println!("Invalid value: {}", f);
                        std::process::exit(1);
                    }
                }
            }
            None => { dval }
        }
    };

    p.min = parse_number("m", 1);
    p.max = parse_number("M", 10000000000);
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

    //let mut perc = histogram.iter_quantiles(1);

    //loop {
    //    match perc.next() {
    //        Some(p) => {
    //            if p.count_since_last_iteration() > 0 {
    //                println!("[{:.10},{:.3}],", p.quantile(), p.value_iterated_to() as f32 / 1000.0);
    //            }
    //            //println!("{:?}", p);
    //        }
    //        None => { break; }
    //    }
    //}

    let p_vector: Vec<f64> = vec![ 5.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 99.0, 100.0 ];

    //let p_array: [f64; 6] = [ 0.05, 0.1, 0.2, 0.4, 0.9, 1.0 ];

    for p in &p_vector {
        println!("[{:.2},{:.3}],", p, histogram.value_at_quantile(*p / 100.0)  as f64 / 1000.0);
    }
    //println!("At 5% {:.3}", histogram.value_at_quantile(0.05) as f32 / 1000.0);
    //println!("At 90% {:.3}", histogram.value_at_quantile(0.9) as f32 / 1000.0);

    println!("Standard deviation {:.3}", histogram.stdev() / 1000.0);
    println!("Mean {:.3}", histogram.mean() / 1000.0);
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
