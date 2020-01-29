use getopts::Options;
use std::env;
use std::fmt;

static HELP_MSG: &str = "Use -h,--help to display help screen.";

#[derive(Debug)]
pub struct Params {
    pub min: u64,
    pub max: u64,
    pub files: Vec<String>,
}

fn help(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    eprintln!("{}\nOption -f,--file can be specified more than once.", opts.usage(&brief));
    std::process::exit(1);
}

impl fmt::Display for Params {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "min:{}, max:{}, files:{}", self.min, self.max, self.files.len())
    }
}

pub fn parse_args(p: &mut Params) {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optopt("m", "min", "set minimum value. Default is 1.", "<number>");
    opts.optopt("M", "max", "set maximum value. Default is 10000000000.", "<number>");
    opts.optmulti("f", "file", "input file.", "<file>");
    opts.optflag("h", "help", "print this help");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            eprintln!("{}. {}", f.to_string(), HELP_MSG);
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
    p.max = parse_number("M", 10_000_000_000);

    if matches.opt_count("f") > 0 {
        for f in matches.opt_strs("f") {
            p.files.push(f);
        }
    }

    if p.files.len() == 0 {
        eprintln!("Option -f,--file not provided. {}", HELP_MSG);
        std::process::exit(1);
    }

    eprintln!("{}", p);

}
