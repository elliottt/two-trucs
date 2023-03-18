use failure::Error;
use std::io::Write;

use crate::parse::{CodeBlockKind, Doc, HeadingLevel, LinkType, Node, Tag};

/// Render a `Doc` to the given output target.
pub fn render_document<'a>(doc: &Doc<'a>, output: &mut dyn Write) -> Result<(), Error> {
    writeln!(output, "")?;
    Renderer::new(output).render_children(doc)?;
    writeln!(output, "")?;
    Ok(())
}

#[derive(Debug, Clone)]
enum SepMode {
    Join,
    NewLine,
    EmptyLine,
}

#[derive(Debug, Clone)]
enum BulletMode {
    Char(char),
    Number(u64),
}

impl BulletMode {
    fn next(&self, opt: &Option<u64>) -> BulletMode {
        if let Some(start) = opt {
            BulletMode::Number(*start)
        } else {
            // TODO: cycle through available bullet types, depending on what self is
            BulletMode::Char('*')
        }
    }

    fn render(&mut self, output: &mut dyn Write) -> Result<(), Error> {
        match self {
            BulletMode::Char(c) => write!(output, "{} ", c)?,

            BulletMode::Number(i) => {
                write!(output, "{}. ", i)?;
                *i += 1;
            }
        }

        Ok(())
    }
}

struct Renderer<'a> {
    sep: SepMode,
    bullet: BulletMode,
    indent: usize,
    leading: Option<char>,
    output: &'a mut dyn Write,
}

impl<'a> Renderer<'a> {
    fn new(output: &'a mut dyn Write) -> Self {
        Renderer {
            sep: SepMode::EmptyLine,
            bullet: BulletMode::Char('*'),
            indent: 0,
            leading: None,
            output,
        }
    }

    /// Emit a newline when nested
    fn nested_nl(&mut self) -> Result<bool, Error> {
        if self.indent > 0 {
            self.nl()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Emit a newline, and handle indentation and possible leading chars.
    fn nl(&mut self) -> Result<(), Error> {
        writeln!(self.output, "")?;

        if let Some(c) = self.leading {
            write!(self.output, "{} ", c)?;
        }

        self.render_indent()?;
        Ok(())
    }

    /// Sometimes text has newlines in it. This translates the newlines to calls to `self.nl`.
    fn nl_str(&mut self, nls: &str) -> Result<(), Error> {
        let mut lines = nls.lines();

        if let Some(line) = lines.next() {
            write!(self.output, "{}", line)?;

            for line in lines {
                self.nl()?;
                write!(self.output, "{}", line)?;
            }
        }

        Ok(())
    }

    fn leading(&mut self) -> Result<(), Error> {
        if let Some(c) = self.leading {
            write!(self.output, "{} ", c)?;
        }
        Ok(())
    }

    fn sep(&mut self) -> Result<(), Error> {
        match self.sep {
            SepMode::Join => Ok(()),
            SepMode::NewLine => self.nl(),
            SepMode::EmptyLine => {
                self.nl()?;
                self.nl()
            }
        }
    }

    fn render_indent(&mut self) -> Result<(), Error> {
        write!(self.output, "{:indent$}", "", indent = self.indent * 2)?;
        Ok(())
    }

    fn set_leading(&mut self, mut leading: Option<char>) -> Option<char> {
        std::mem::swap(&mut self.leading, &mut leading);
        leading
    }

    fn set_sep(&mut self, mut sep: SepMode) -> SepMode {
        std::mem::swap(&mut self.sep, &mut sep);
        sep
    }

    fn set_indent(&mut self, mut indent: usize) -> usize {
        std::mem::swap(&mut self.indent, &mut indent);
        indent
    }

    fn set_bullet(&mut self, mut bullet: BulletMode) -> BulletMode {
        std::mem::swap(&mut self.bullet, &mut bullet);
        bullet
    }

    /// Render a group of children from a single node.
    fn render_children<'b>(&mut self, children: &Doc<'b>) -> Result<(), Error> {
        if let Some(child) = children.first() {
            self.render_node(child)?;

            for child in &children[1..] {
                self.sep()?;
                self.render_node(child)?;
            }
        }

        Ok(())
    }

    /// Render a single node, and all of its children.
    fn render_node<'b>(&mut self, child: &Node<'b>) -> Result<(), Error> {
        match child {
            Node::Node { tag, children } => self.render_nested(tag, children)?,

            Node::Text(cow) => self.nl_str(cow)?,

            Node::Code(cow) => {
                write!(self.output, "`")?;
                self.nl_str(cow)?;
                write!(self.output, "`")?;
            }

            Node::Html(cow) => self.nl_str(cow)?,

            Node::FootnoteReference(cow) => {
                write!(self.output, "[^")?;
                self.nl_str(cow)?;
                write!(self.output, "]")?
            }

            Node::SoftBreak => self.nl()?,

            Node::HardBreak => {
                write!(self.output, "  ")?;
                self.nl()?
            }

            Node::Rule => write!(self.output, "---")?,

            Node::TaskListMarker(finished) => {
                write!(self.output, "[")?;
                if *finished {
                    write!(self.output, "x")?;
                } else {
                    write!(self.output, " ")?;
                }
                write!(self.output, "] ")?;
            }
        }

        Ok(())
    }

    /// Render a nested node. The tag indicates the type of node that contains the children.
    fn render_nested<'b>(&mut self, tag: &Tag<'b>, children: &Doc<'b>) -> Result<(), Error> {
        match tag {
            Tag::Paragraph => {
                let sep = self.set_sep(SepMode::Join);

                self.render_children(children)?;

                self.set_sep(sep);
            }

            Tag::Heading(level, _, _) => {
                let sep = self.set_sep(SepMode::Join);

                let level = match level {
                    HeadingLevel::H1 => "#",
                    HeadingLevel::H2 => "##",
                    HeadingLevel::H3 => "###",
                    HeadingLevel::H4 => "####",
                    HeadingLevel::H5 => "#####",
                    HeadingLevel::H6 => "######",
                };

                write!(self.output, "{} ", level)?;

                self.render_children(children)?;

                self.set_sep(sep);
            }

            Tag::BlockQuote => {
                let leading = self.set_leading(Some('>'));

                if !self.nested_nl()? {
                    self.leading()?
                }

                self.render_children(children)?;
                self.set_leading(leading);
            }

            Tag::CodeBlock(CodeBlockKind::Fenced(lang)) => {
                self.nested_nl()?;

                write!(self.output, "```{}", lang)?;
                self.nl()?;

                self.render_children(children)?;

                self.nl()?;

                write!(self.output, "```")?;
            }

            Tag::CodeBlock(CodeBlockKind::Indented) => {
                let sep = self.set_sep(SepMode::NewLine);

                self.nested_nl()?;

                let indent = self.set_indent(self.indent + 2);

                self.render_indent()?;

                self.render_children(children)?;

                self.set_sep(sep);
                self.set_indent(indent);
            }

            Tag::List(opt) => {
                let sep = self.set_sep(SepMode::NewLine);
                let bullet = self.set_bullet(self.bullet.next(opt));

                self.nested_nl()?;

                self.render_children(children)?;

                self.set_sep(sep);
                self.set_bullet(bullet);
            }

            Tag::Item => {
                let sep = self.set_sep(SepMode::Join);
                let indent = self.set_indent(self.indent + 1);

                self.bullet.render(self.output)?;
                self.render_children(children)?;

                self.set_sep(sep);
                self.set_indent(indent);
            }

            // Tag::FootnoteDefinition
            // Tag::Table
            // Tag::TableHead
            // Tag::TableCell
            Tag::Emphasis => {
                write!(self.output, "*")?;
                self.render_children(children)?;
                write!(self.output, "*")?;
            }

            Tag::Strong => {
                write!(self.output, "**")?;
                self.render_children(children)?;
                write!(self.output, "**")?;
            }

            Tag::Strikethrough => {
                write!(self.output, "~~")?;
                self.render_children(children)?;
                write!(self.output, "~~")?;
            }

            Tag::Link(ty, dest, title) => {
                let sep = self.set_sep(SepMode::Join);

                match ty {
                    LinkType::Inline
                    | LinkType::Reference
                    | LinkType::Collapsed
                    | LinkType::Shortcut => {
                        write!(self.output, "[")?;
                        self.nl_str(title)?;
                        self.render_children(children)?;
                        write!(self.output, "]")?;
                    }

                    _ => {}
                }

                match ty {
                    LinkType::Inline => {
                        write!(self.output, "(")?;
                        self.nl_str(dest)?;
                        write!(self.output, ")")?;
                    }

                    LinkType::Reference => {
                        write!(self.output, "(")?;
                        self.nl_str(dest)?;
                        write!(self.output, ")")?;
                    }

                    LinkType::Collapsed => {
                        write!(self.output, "[]")?;
                    }

                    LinkType::Autolink | LinkType::Email => {
                        write!(self.output, "<")?;
                        self.nl_str(dest)?;
                        write!(self.output, ">")?;
                    }

                    _ => {}
                }

                self.set_sep(sep);
            }

            // Tag::Image
            _ => (),
        }

        Ok(())
    }
}
