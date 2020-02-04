extern crate getopts;
extern crate hdrhistogram;

pub mod core;

fn run(p: &core::params::Params) {
    let mut first = true;

    for f in &p.files {
        if first {
            first = false;
            print!("{{\"title\":\"{}\"", p.title);

            if p.sub_title.len() > 0 {
                print!(",\"subTitle\":\"{}\"", p.sub_title);
            }

            print!(",\"graphMax\":{},\"chArray\":[", p.graph_max);
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

    println!("]}}");
}

fn main() -> Result<(), ()> {
    let p = core::args::parse_args();
    run(&p);

    Ok(())
}
