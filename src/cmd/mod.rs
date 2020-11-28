use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct CLI {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: PathBuf,

    #[structopt(long, short = "t")]
    out_type: Option<String>,

    #[structopt(long, short, required_if("out-type", "file"))]
    out_file: Option<String>,
}

pub fn run() {
    let args = CLI::from_args();

    // Open the given file
    let fp = File::open(&args.path).expect("open file failed");

    // create a read buffer for performance
    let mut reader = BufReader::new(fp);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains(&args.pattern) {
                println!("{}", line);
            }
        }
    }

    println!("args={:#?}", args);
}
