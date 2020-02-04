use getopts::Options;
use std::env;

static HELP_MSG: &str = "Use -h,--help to display help screen.";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn help(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    eprintln!("{}\nOption -f,--file can be specified more than once.", opts.usage(&brief));
    std::process::exit(1);
}

pub fn parse_args() -> super::params::Params {
    let mut p = super::params::Params {
        min: 0,
        max: 0,
        title: String::new(),
        sub_title: String::new(),
        graph_max: 100,
        files: Vec::new(),
    };

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optopt("m", "min", "set minimum value (in nanoseconds). Default is 1.", "<number>");
    opts.optopt("M", "max", "set maximum value (in nanoseconds). Default is 10000000000.", "<number>");
    opts.optopt("g", "graph-max", "set maximum value for graph (in microseconds). Default is 100.", "<number>");
    opts.optopt("t", "title", "set graph title.", "<string>");
    opts.optopt("s", "sub_title", "set graph subtitle.", "<string>");
    opts.optmulti("f", "file", "input file.", "<file>");
    opts.optflag("h", "help", "print this help");
    opts.optflag("v", "version", "print version information");

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

    if matches.opt_present("v") {
        eprintln!("v{}", VERSION);
        std::process::exit(1);
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
    p.graph_max = parse_number("g", 100);

    p.title = match matches.opt_str("t") {
        Some(m) => { m }
        None => { "Percentile chart".to_string() }
    };

    p.sub_title = match matches.opt_str("s") {
        Some(m) => { m }
        None => { String::new() }
    };

    if matches.opt_count("f") > 0 {
        for f in matches.opt_strs("f") {
            p.files.push(f);
        }
    }

    if p.files.len() == 0 {
        eprintln!("Option -f,--file not provided. {}", HELP_MSG);
        std::process::exit(1);
    }

    p
}
