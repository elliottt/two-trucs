use crate::parse::{Doc, HeadingLevel, Node, Tag};

/// Introduce a new top-level heading, and migrate all unfinished tasks underneath it.
pub fn start_next_day<'a>(doc: Doc<'a>, day_title: &str) -> Doc<'a> {
    let mut buf = Buffer::new();

    buf.top.push(make_next_day(day_title));

    for range in day_ranges(&doc) {
        let stats = doc[range.clone()]
            .iter()
            .fold(TodoStats::default(), |mut acc, node| {
                acc += num_incomplete(node);
                acc
            });

        if stats.incomplete == 0 {
            buf.old.extend(doc[range.clone()].iter().cloned());
            continue;
        }

        for node in doc[range].iter() {
            buf.push(node.clone());
        }
    }

    buf.to_doc()
}

fn day_ranges(nodes: &[Node<'_>]) -> Vec<std::ops::Range<usize>> {
    let mut ranges = vec![];

    let mut start = 0;
    for (ix, node) in nodes.iter().enumerate() {
        if let Some(HeadingLevel::H1) = is_heading(node) {
            // Emit the previous range
            if start < ix {
                ranges.push(start..ix);
            }

            start = ix;
        }
    }

    let last = start..nodes.len();
    if !last.is_empty() {
        ranges.push(last);
    }

    ranges
}

fn make_next_day<'a>(day_title: &str) -> Node<'a> {
    let tag = Tag::Heading {
        level: HeadingLevel::H1,
        id: None,
        classes: vec![],
        attrs: vec![],
    };
    let text = Node::Text(String::from(day_title).into());

    Node::Node {
        tag,
        children: vec![text],
    }
}

struct Buffer<'a> {
    current_level: HeadingLevel,
    top: Vec<Node<'a>>,
    next: Vec<Node<'a>>,
    old: Vec<Node<'a>>,
}

impl<'a> Buffer<'a> {
    fn new() -> Self {
        Self {
            current_level: HeadingLevel::H1,
            top: vec![],
            next: vec![],
            old: vec![],
        }
    }

    fn push(&mut self, node: Node<'a>) {
        // If this is a heading, we us special logic to determine if  a previous heading needs to
        // be erased from either `next` or `old`
        if let Some(level) = is_heading(&node) {
            // We ignore nodes at heading 1 for the next nodes, as we're collecting todos from
            // previous days at heading 1.
            if level > HeadingLevel::H1 {
                if let Some(prev_level) = self.next.last().and_then(is_heading) {
                    if level <= prev_level {
                        self.next.pop();
                    }
                }

                self.next.push(node.clone());
            }

            if let Some(prev_level) = self.old.last().and_then(is_heading) {
                if level <= prev_level {
                    self.old.pop();
                }
            }

            self.old.push(node);

            self.current_level = level;

            return;
        }

        // Lists get filtered and completed tasks are held back.
        if let Node::Node {
            tag: Tag::List(_),
            children,
        } = &node
        {
            let total = num_incomplete(&node);
            if total.incomplete > 0 {
                let mut incomplete = vec![];
                let mut complete = vec![];

                for child in children {
                    let stats = num_incomplete(child);
                    if stats.incomplete > 0 || stats.is_empty() {
                        incomplete.push(child.clone());
                    } else {
                        complete.push(child.clone());
                    }
                }

                if !incomplete.is_empty() {
                    self.next.push(Node::Node {
                        tag: Tag::List(None),
                        children: incomplete,
                    });
                }

                if !complete.is_empty() {
                    self.old.push(Node::Node {
                        tag: Tag::List(None),
                        children: complete,
                    });
                }
            } else {
                self.old.push(node);
            }

            return;
        }

        // By default, migrate everything forward.
        if self.current_level == HeadingLevel::H1 {
            self.top.push(node);
        } else {
            self.next.push(node);
        }
    }

    fn to_doc(mut self) -> Doc<'a> {
        drop_empty_headings(&mut self.next);
        drop_empty_headings(&mut self.old);

        self.top.append(&mut self.next);
        self.top.append(&mut self.old);
        self.top
    }
}

#[derive(Default, Copy, Clone)]
struct TodoStats {
    incomplete: u32,
    complete: u32,
}

impl core::ops::AddAssign for TodoStats {
    fn add_assign(&mut self, rhs: Self) {
        self.incomplete += rhs.incomplete;
        self.complete += rhs.complete;
    }
}

impl TodoStats {
    fn is_empty(&self) -> bool {
        self.incomplete == 0 && self.complete == 0
    }
}

fn num_incomplete<'a>(node: &Node<'a>) -> TodoStats {
    if let Some(complete) = node.is_task_marker() {
        return if complete {
            TodoStats {
                incomplete: 0,
                complete: 1,
            }
        } else {
            TodoStats {
                incomplete: 1,
                complete: 0,
            }
        };
    }

    if let Node::Node { children, .. } = node {
        return children.iter().fold(TodoStats::default(), |mut acc, node| {
            acc += num_incomplete(node);
            acc
        });
    }

    TodoStats::default()
}

fn is_heading(node: &Node<'_>) -> Option<HeadingLevel> {
    if let Node::Node {
        tag: Tag::Heading { level, .. },
        ..
    } = node
    {
        return Some(*level);
    }

    None
}

fn drop_empty_headings(nodes: &mut Vec<Node<'_>>) {
    while let Some(level) = nodes.last().and_then(is_heading) {
        if level == HeadingLevel::H1 {
            break;
        }
        nodes.pop();
    }
}
