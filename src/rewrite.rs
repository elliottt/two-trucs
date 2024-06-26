use pulldown_cmark::{Options, Parser};
use std::io::Write;

use crate::{next, parse, render, sort};

pub fn rewrite(
    opt_title: Option<String>,
    input: &str,
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    let opts = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;
    let doc = parse::DocBuilder::from(Parser::new_ext(&input, opts)).build();

    let doc = if let Some(title) = opt_title {
        next::start_next_day(doc, &title)
    } else {
        sort::sort_tasks(doc)
    };

    render::render_document(&doc, output)?;

    Ok(())
}
