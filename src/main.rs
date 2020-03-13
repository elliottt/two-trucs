extern crate clap;
extern crate comrak;
extern crate failure;
extern crate pulldown_cmark;

use clap::{App, Arg};
use failure::Error;
use pulldown_cmark::{Options, Parser};

use std::{
    fs::File,
    io::{self, Read},
};

mod next;
mod parse;
mod render;
mod sort;

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

    let mut input = String::new();
    match matches.value_of("input") {
        Some("-") | None => {
            io::stdin().read_to_string(&mut input)?;
        }
        Some(path) => {
            let mut f = File::open(path)?;
            f.read_to_string(&mut input)?;
        }
    }

    let mut doc = parse::DocBuilder::from(Parser::new_ext(&input, Options::all())).build();
    sort::sort_tasks(&mut doc);

    if matches.is_present("next") {
        next::start_next_day(&mut doc, matches.value_of("title").unwrap())
    }

    render::render_document(&doc, &mut io::stdout())?;

    Ok(())
}
