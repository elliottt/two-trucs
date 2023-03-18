use crate::parse::{Doc, Node, Tag};

/// Introduce a new top-level heading, and migrate all unfinished tasks underneath it.
pub fn start_next_day<'a>(doc: Doc<'a>, day_title: &str) -> Doc<'a> {
    let mut buf = Buffer::new();
    buf.next.push(make_next_day(day_title));

    collect_unfinished(&mut buf, doc);

    buf.to_doc()
}

fn make_next_day<'a>(day_title: &str) -> Node<'a> {
    let tag = Tag::Heading(1);
    let text = Node::Text(String::from(day_title).into());

    Node::Node {
        tag,
        children: vec![text],
    }
}

fn make_list<'a>(opt: Option<u64>, children: Doc<'a>) -> Node<'a> {
    let tag = Tag::List(opt);
    Node::Node { tag, children }
}

struct Level {
    heading: u32,
    had_todo: bool,
    index: usize,
}

impl Level {
    fn new(heading: u32, index: usize) -> Self {
        Self {
            heading,
            had_todo: false,
            index,
        }
    }
}

struct Buffer<'a> {
    levels: Vec<Level>,
    buf: Vec<Node<'a>>,
    next: Vec<Node<'a>>,
    old: Vec<Node<'a>>,
}

impl<'a> Buffer<'a> {
    fn new() -> Self {
        Self {
            levels: vec![Level::new(1, 0)],
            buf: Vec::new(),
            next: Vec::new(),
            old: Vec::new(),
        }
    }

    fn level(&mut self) -> &mut Level {
        self.levels.last_mut().unwrap()
    }

    fn push_level(&mut self, heading: u32) {
        self.levels.push(Level::new(heading, self.buf.len()));
    }

    fn push(&mut self, node: Node<'a>) {
        if !self.level().had_todo {
            match &node {
                Node::Node {
                    tag: Tag::List(_),
                    children,
                } => {
                    self.level().had_todo = children.iter().any(has_outstanding_todo);
                }

                _ => {}
            }
        }

        self.buf.push(node);
    }

    fn flush(&mut self, heading: u32) {
        if heading > self.level().heading {
            return;
        }

        // An invariant of the levels vector is that the headings will always be in strict
        // ascending order.
        let start_next = self.next.len();
        let start_old = self.old.len();
        for level in self.levels.iter().rev() {
            if level.heading < heading {
                break;
            }

            if !level.had_todo {
                self.old.extend(self.buf.drain(level.index..).rev());
                continue;
            }

            for node in self.buf.drain(level.index..).rev() {
                match node {
                    // Lists are filtered between old and next, keeping any unfinished tasks in
                    // `next`.
                    Node::Node {
                        tag: Tag::List(opt),
                        mut children,
                    } => {
                        let mut open = Vec::new();
                        let mut done = Vec::new();

                        for child in children.drain(..) {
                            if has_outstanding_todo(&child) {
                                open.push(child)
                            } else {
                                done.push(child)
                            }
                        }

                        if !open.is_empty() {
                            self.next.push(make_list(opt, open));
                        }
                        if !done.is_empty() {
                            self.old.push(make_list(opt, done));
                        }
                    }

                    // Headings are unconditionally duplicated.
                    Node::Node {
                        tag: Tag::Heading(_),
                        ..
                    } => {
                        self.next.push(node.clone());
                        self.old.push(node);
                    }

                    // Everything else is only preserved in the new section.
                    _ => {
                        self.next.push(node);
                    }
                }
            }
        }

        self.levels.retain(|l| l.heading < heading);
        self.next[start_next..].reverse();
        self.old[start_old..].reverse();
    }

    fn to_doc(mut self) -> Doc<'a> {
        self.flush(0);
        self.next.extend(self.old.drain(..));
        self.next
    }
}

fn collect_unfinished<'a>(buf: &mut Buffer<'a>, doc: Doc<'a>) {
    for child in doc {
        if let Node::Node {
            tag: Tag::Heading(n),
            ..
        } = child
        {
            buf.flush(n);
            buf.push_level(n);
        }

        buf.push(child);
    }
}

fn has_outstanding_todo<'a>(node: &Node<'a>) -> bool {
    if let Some(false) = node.is_todo() {
        true
    } else {
        match node {
            Node::Node { children, .. } => children.iter().any(has_outstanding_todo),

            _ => false,
        }
    }
}
