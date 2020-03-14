use failure::Error;
use pulldown_cmark::{Options, Parser};
use std::io::Write;

use crate::{next, parse, render, sort};

pub fn rewrite(
    opt_title: Option<&str>,
    input: &str,
    output: &mut dyn Write,
) -> Result<(), Error> {
    let mut doc = parse::DocBuilder::from(Parser::new_ext(&input, Options::all())).build();

    if let Some(title) = opt_title {
        next::start_next_day(&mut doc, title)
    } else {
        sort::sort_tasks(&mut doc)
    }

    render::render_document(&doc, output)?;

    Ok(())
}
