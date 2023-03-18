extern crate clap;
extern crate failure;
extern crate pulldown_cmark;

use clap::{App, Arg};
use failure::Error;

use std::{
    fs::File,
    io::{self, Read},
};

use two_trucs::rewrite;

fn open_input(input: Option<&str>) -> io::Result<Box<dyn Read>> {
    let handle: Box<dyn Read> = match input {
        Some("-") | None => Box::new(io::stdin()),
        Some(file) => Box::new(File::open(file)?),
    };
    Ok(handle)
}

fn main() -> Result<(), Error> {
    let matches = App::new("updo")
        .version("0.1.0")
        .author("Trevor Elliott")
        .about("Markdown TODO list maintainer")
        .arg(
            Arg::with_name("next")
                .short("n")
                .long("next")
                .help("Start a new day"),
        )
        .arg(
            Arg::with_name("title")
                .short("t")
                .long("title")
                .takes_value(true)
                .default_value("Today")
                .help("Set the title for the new day"),
        )
        .arg(
            Arg::with_name("input")
                .index(1)
                .help("The TODO file to process"),
        )
        .get_matches();

    let mut handle = open_input(matches.value_of("input"))?;

    let input = {
        let mut buf = String::new();
        handle.read_to_string(&mut buf)?;
        buf
    };

    let opt_title = if matches.is_present("next") {
        matches.value_of("title")
    } else {
        None
    };

    rewrite::rewrite(opt_title, &input, &mut io::stdout())?;

    Ok(())
}
