extern crate getopts;

use std::env;
use std::error;
use std::fs;
use std::path;

fn genfile(path: &path::Path) -> Result<(), Box<dyn error::Error>> {
    let dir_name = path.parent().unwrap();

    if !dir_name.file_name().unwrap().is_empty() {
        match fs::create_dir_all(dir_name) {
            Ok(()) => {}
            Err(e) => {
                panic!(
                    "Failed to create directories '{}': {}",
                    dir_name.file_name().unwrap().to_str().unwrap(),
                    e
                );
            }
        }
    }

    let result = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path);

    if result.is_err() {
        panic!(
            "Failed to create a file '{}': {}",
            path.to_str().unwrap(),
            result.err().unwrap()
        );
    }

    return Ok(());
}

fn print_help(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [-h] [files...]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "prints this help menu and exits");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string()),
    };

    if matches.opt_present("h") {
        print_help(&program, opts);
        return;
    }

    for arg in matches.free {
        match genfile(path::Path::new(&arg)) {
            Ok(()) => {}
            Err(err) => {
                println!("error: {}", err);
            }
        }
    }
}
