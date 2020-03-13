use pulldown_cmark::{CowStr, Event, Parser};

pub use pulldown_cmark::{CodeBlockKind, Tag};

pub type Doc<'a> = Vec<Node<'a>>;

pub struct DocBuilder<'a> {
    doc: Doc<'a>,
    partial: Vec<PartialNode<'a>>,
}

impl<'a> DocBuilder<'a> {
    pub fn new() -> Self {
        DocBuilder {
            doc: Vec::new(),
            partial: Vec::new(),
        }
    }

    pub fn from(parser: Parser<'a>) -> Self {
        let mut builder = Self::new();

        for event in parser {
            match event {
                Event::Start(tag) => builder.start(tag),
                Event::End(_) => builder.end(),
                Event::Text(cow) => builder.push(Node::Text(cow)),
                Event::Code(cow) => builder.push(Node::Code(cow)),
                Event::Html(cow) => builder.push(Node::Html(cow)),
                Event::FootnoteReference(cow) => builder.push(Node::FootnoteReference(cow)),
                Event::HardBreak => builder.push(Node::HardBreak),
                Event::SoftBreak => builder.push(Node::SoftBreak),
                Event::Rule => builder.push(Node::Rule),
                Event::TaskListMarker(b) => builder.push(Node::TaskListMarker(b)),
            }
        }

        builder
    }

    pub fn build(self) -> Doc<'a> {
        self.doc
    }

    pub fn start(&mut self, tag: Tag<'a>) {
        self.partial.push(PartialNode::new(tag));
    }

    pub fn end(&mut self) {
        if let Some(partial) = self.partial.pop() {
            self.push(partial.build());
        }
    }

    pub fn push(&mut self, node: Node<'a>) {
        if let Some(parent) = self.partial.last_mut() {
            parent.push(node);
        } else {
            self.doc.push(node)
        }
    }
}

struct PartialNode<'a> {
    tag: Tag<'a>,
    children: Doc<'a>,
}

impl<'a> PartialNode<'a> {
    fn new(tag: Tag<'a>) -> Self {
        PartialNode {
            tag,
            children: Vec::new(),
        }
    }

    fn push(&mut self, child: Node<'a>) {
        self.children.push(child)
    }

    fn build(self) -> Node<'a> {
        Node::Node {
            tag: self.tag,
            children: self.children,
        }
    }
}

#[derive(Debug,Clone)]
pub enum Node<'a> {
    Node {
        tag: Tag<'a>,
        children: Vec<Node<'a>>,
    },
    Text(CowStr<'a>),
    Code(CowStr<'a>),
    Html(CowStr<'a>),
    FootnoteReference(CowStr<'a>),
    HardBreak,
    SoftBreak,
    Rule,
    TaskListMarker(bool),
}

impl<'a> Node<'a> {
    /// Returns `Some` if this node is a task list item, and a boolean value indicating whether or
    /// not it's checked.
    pub fn is_todo(&self) -> Option<bool> {
        if let Node::Node { tag, children } = self {
            if let Tag::Item = tag {
                return children.first().and_then(|node| node.is_task_marker());
            }
        }

        None
    }

    pub fn is_task_marker(&self) -> Option<bool> {
        if let Node::TaskListMarker(b) = self {
            Some(*b)
        } else {
            None
        }
    }
}
