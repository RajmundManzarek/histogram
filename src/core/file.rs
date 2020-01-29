use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
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

    eprintln!("Done"); 
    Ok(())
}
