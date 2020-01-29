extern crate getopts;
extern crate hdrhistogram;

pub mod core;

fn run(p: &core::params::Params) {
    let mut first = true;

    for f in &p.files {
        if first {
            first = false;
        } else {
            print!(",");
        }

        match core::file::process_file(f, p) {
            Ok(_m) => { }
            Err(e) => {
                eprintln!("Unable to process input file {}. {:?}", f, e);
                std::process::exit(1);
            }
        }
    }
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
