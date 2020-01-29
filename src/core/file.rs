use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
//use std::ffi::OsStr;
use regex::Regex;
use hdrhistogram::Histogram;

pub fn process_file(name: &String, p: &super::params::Params) -> std::io::Result<()> {
    eprintln!("Processing file {}", name);

    let re = Regex::new(r"^\d+$").unwrap();
    let mut histogram = Histogram::<u64>::new_with_bounds(p.min, p.max, 3).unwrap();

    let file = File::open(name)?;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();

        if re.is_match(&line) {
            match line.parse::<u64>() {
                Ok(m) => { histogram += m; }
                Err(_e) => {}
            }
        }
    }

    let p_vector: Vec<f64> = vec![ 5.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 99.0, 100.0 ];
    let f_name = Path::new(name).file_name().unwrap().to_str().unwrap();

    print!("[{{\"name\":\"{}\"", f_name);

    let mut first = true;

    for p in &p_vector {
        if first {
            first = false;
            print!(",\"data\":[");
        } else {
            print!("\n,");
        }

        print!("[{:.2},{:.3}]", p, histogram.value_at_quantile(*p / 100.0) as f64 / 1000.0);
    }

    println!("],\"stats\":{{");

    println!("\"stdDev\": {:.3}", histogram.stdev() / 1000.0);
    println!(",\"mean\": {:.3}", histogram.mean() / 1000.0);
    println!(",\"median\": {:.3}", histogram.value_at_percentile(0.5) as f64 / 1000.0);
    println!(",\"min\": {:.3}", histogram.min() as f64 / 1000.0);
    println!(",\"max\": {:.3}", histogram.max() as f64 / 1000.0);
    println!(",\"count\": {}}}", histogram.len());
        
    let h_vector: Vec<u64> = vec![ 0, 1_000, 2_000, 3_000, 4_000, 5_000, 6_000, 7_000, 8_000, 9_000, 10_000, 11_000, 12_000, 13_000, 14_000, 15_000 ];

    let max = h_vector.len() - 1;

    for h in 0..max {
        let first = h_vector[h] + 1;
        let second = h_vector[h + 1];
        //let how_many = histogram.count_between(first, second);

        println!("{} - {} --> {}", first, second, histogram.count_between(first, second));
        


    }

    println!("}}]");

    histogram.clear();
    eprintln!("Done"); 
    Ok(())
}
