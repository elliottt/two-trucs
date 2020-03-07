
extern crate clap;
extern crate comrak;
extern crate failure;

use clap::{App, Arg};
use failure::{err_msg, Error};

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
             .help("The TODO file to process")
             .required(true))
        .get_matches();

    Ok(())
}
