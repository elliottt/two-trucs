use comrak::{nodes::AstNode, ComrakOptions};
use failure::Error;

use std::io::Write;

pub fn render_document<'a>(
    doc: &'a AstNode<'a>,
    opts: &ComrakOptions,
    output: &mut dyn Write,
) -> Result<(), Error> {
    comrak::format_commonmark(doc, &opts, output)?;
    Ok(())
}
