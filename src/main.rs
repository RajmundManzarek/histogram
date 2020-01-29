extern crate getopts;
extern crate hdrhistogram;

//pub mod params;
//pub mod file;

//use core::*;

pub mod core;

fn run(p: &core::params::Params) {
    for f in &p.files {
        match core::file::process_file(f, p) {
            Ok(_m) => { }
            Err(e) => {
                eprintln!("Unable to process input file {}. {:?}", f, e);
                std::process::exit(1);
            }
        }
    }
//    let re = Regex::new(r"^\d+$").unwrap();
//    let mut histogram = Histogram::<u64>::new_with_bounds(p.min, p.max, 3).unwrap();
//
//    let stdin = io::stdin();
//    for line in stdin.lock().lines() {
//        let l = line.unwrap();
//
//        if re.is_match(&l) {
//            match l.parse::<u64>() {
//                Ok(m) => { histogram += m; }
//                Err(_e) => {}
//            }
//        }
//    }
//
//    let p_vector: Vec<f64> = vec![ 5.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 99.0, 100.0 ];
//
//    print!("{{");
//
//    let mut first = true;
//
//    for p in &p_vector {
//        if first {
//            first = false;
//            print!("\"data\":[");
//        } else {
//            print!("\n,");
//        }
//
//        print!("[{:.2},{:.3}]", p, histogram.value_at_quantile(*p / 100.0) as f64 / 1000.0);
//    }
//
//    println!("]");
//
//    println!(",\"stdDev\": {:.3}", histogram.stdev() / 1000.0);
//    println!(",\"mean\": {:.3}", histogram.mean() / 1000.0);
//    println!(",\"median\": {:.3}", histogram.value_at_percentile(0.5) as f64 / 1000.0);
//    println!(",\"min\": {:.3}", histogram.min() as f64 / 1000.0);
//    println!(",\"max\": {:.3}", histogram.max() as f64 / 1000.0);
//    println!(",\"count\": {}", histogram.len());

//    let mut h = histogram.iter_linear(100);
//    first = true;
//
//    loop {
//        match h.next() {
//            Some(m) => {
//                if m.count_since_last_iteration() > 0 {
//                    if m.value_iterated_to() < 10_000 {
//                        if first {
//                            first = false;
//                            print!(",\"histogramData\":[");
//                        } else {
//                            print!("\n,");
//                        }
//
//                        print!("[{:.3},{}]", m.value_iterated_to() as f64 / 1000.0, m.count_since_last_iteration());
//                    }
//                }
//            }
//            None => { break; }
//        };
//    }

//    let h_vector: Vec<u64> = vec![ 0, 1_000, 2_000, 3_000, 4_000, 5_000, 6_000, 7_000, 8_000, 9_000, 10_000, 11_000, 12_000, 13_000, 14_000, 15_000 ];
//
//    let max = h_vector.len() - 1;
//
//    for h in 0..max {
//        let first = h_vector[h] + 1;
//        let second = h_vector[h + 1];
//        let how_many = histogram.count_between(first, second);
//
//        println!("{} - {} --> {}", first, second, histogram.count_between(first, second));
//        
//
//
//    }

    //println!("{}", h_vector.len());
    //println!("{}", histogram.count_between(0, 2_000));
    //println!("{}", histogram.count_between(2_001, 4_000));
    //println!("]}}");
}

fn main() -> Result<(), ()> {
    let mut p = core::params::Params {
        min: 0,
        max: 0,
        files: Vec::new(),
    };

    core::params::parse_args(&mut p);
    run(&p);

    Ok(())
}
