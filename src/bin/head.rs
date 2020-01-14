use std::io;
use std::iter::Iterator;
use std::io::BufReader;
use std::fs::File;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

use surgery::head;


fn main() -> std::io::Result<()> {
    let matches = App::new("head")
                .version(crate_version!())
                .author(crate_authors!("\n"))
                .about("Print the first 10 lines of text and skip the rest")
                .bin_name("head")
                .arg(Arg::with_name("number")
                     .long("number")
                     .short("n")
                     .help("Number of lines to keep")
                     .value_name("N")
                     .takes_value(true)
                     .default_value("10"))
                .arg(Arg::with_name("files")
                     .help("files to behead, use - for stdin")
                     .value_name("File")
                     .multiple(true)
                     .takes_value(true)
                     .default_value("-"))
                .get_matches();

    let number_of_lines_to_skip = value_t!(matches, "number", usize).unwrap_or_else(|e| e.exit());
    let filenames : Vec<&str> = matches.values_of("files").unwrap().collect();

    for filename in filenames {
        if filename == "-" {
            let stdin = io::stdin();
            let mut stdin_handle = stdin.lock();
            if let Err(e) = head(&mut stdin_handle, number_of_lines_to_skip) {
                eprintln!("error reading from stdin: {}", e);
            }
        } else {
            let f = File::open(filename)?;
            let mut reader = BufReader::new(f);
            if let Err(e) = head(&mut reader, number_of_lines_to_skip) {
                eprintln!("error reading from file: {}", e);
            }
        }
    }

    Ok(())
}
