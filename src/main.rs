extern crate clap;
extern crate failure;
extern crate pulldown_cmark;

#[cfg(test)]
extern crate pretty_assertions;

use clap::{App, Arg};
use failure::Error;

use std::{
    fs::File,
    io::{self, Read},
};

mod next;
mod parse;
mod render;
mod rewrite;
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

    let opt_title = if matches.is_present("next") {
        matches.value_of("title")
    } else {
        None
    };

    rewrite::rewrite(opt_title, &input, &mut io::stdout())?;

    Ok(())
}

#[cfg(test)]
#[macro_use]
mod testing;

#[cfg(test)]
mod tests {
    sort_test!(ident, "tests/ident.md");
    next_test!(ident_next, "tests/ident.md");
}
