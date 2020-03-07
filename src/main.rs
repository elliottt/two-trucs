
extern crate clap;
extern crate comrak;
extern crate failure;

use comrak::{Arena,ComrakOptions};
use clap::{App, Arg};
use failure::{err_msg, Error};

use std::{
    fs::File,
    io::{self, Read},
};

fn main() -> Result<(), Error> {

    let matches = App::new("updo")
        .version("0.1.0")
        .author("Trevor Elliott")
        .about("Markdown TODO list maintainer")
        .arg(Arg::with_name("next")
             .short("n")
             .long("next")
             .help("Start a new day"))
        .arg(Arg::with_name("input")
             .index(1)
             .help("The TODO file to process"))
        .get_matches();

    let mut buf = String::new();
    let input = matches.value_of("input");
    match input {
        Some("-") | None => {
            io::stdin().read_to_string(&mut buf)?;
        },
        Some(path) => {
            let mut f = File::open(path)?;
            f.read_to_string(&mut buf)?;
        },
    }

    let opts = {
        let mut opts = ComrakOptions::default();
        opts.ext_tasklist = true;
        opts
    };

    let arena = Arena::new();

    let doc = comrak::parse_document(&arena, &buf, &opts);

    if matches.is_present("next") {
        println!("next not implemented");
    } else {
        println!("sorting not implemented");
    }

    Ok(())
}
