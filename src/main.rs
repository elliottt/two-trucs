use clap::Parser;
use failure::Error;

use std::{
    fs::File,
    io::{self, Read},
};

use two_trucs::rewrite;

fn open_input(input: Option<&String>) -> io::Result<Box<dyn Read>> {
    let handle: Box<dyn Read> = match input.map(String::as_str) {
        Some("-") | None => Box::new(io::stdin()),
        Some(file) => Box::new(File::open(file)?),
    };
    Ok(handle)
}

#[derive(Parser, Debug)]
#[command(
    author = "Trevor Elliott",
    version = "0.1.0",
    about = "Markdown TODO list maintainer"
)]
struct Options {
    #[arg(short = 'n')]
    next: bool,

    #[arg(short = 's')]
    sort: bool,

    #[arg(short = 't', default_value = "Today")]
    title: String,

    input: Option<String>,
}

fn main() -> Result<(), Error> {
    let options = Options::parse();

    let mut handle = open_input(options.input.as_ref())?;

    let input = {
        let mut buf = String::new();
        handle.read_to_string(&mut buf)?;
        buf
    };

    let opt_title = if options.next {
        Some(options.title)
    } else {
        None
    };

    rewrite::rewrite(opt_title, &input, &mut io::stdout())?;

    Ok(())
}
