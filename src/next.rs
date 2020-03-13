use crate::parse::{Doc, Node, Tag};

/// Introduce a new top-level heading, and migrate all unfinished tasks underneath it.
pub fn start_next_day<'a>(doc: &mut Doc<'a>, day_title: &str) {
    let mut front = Vec::new();

    front.push(make_next_day(day_title));

    collect_unfinished(doc, &mut front);

    front.append(doc);

    std::mem::swap(&mut front, doc)
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

fn collect_unfinished<'a>(doc: &mut Doc<'a>, front: &mut Doc<'a>) {
    let mut last_header = None;

    for child in doc {
        match child {
            Node::Node {
                tag: Tag::Heading(n),
                ..
            } if *n > 1 => {
                last_header = Some(child);
            }

            Node::Node {
                tag: Tag::List(opt),
                children,
            } => {
                let mut todos = Vec::new();
                let mut done = Vec::new();

                for child in children.drain(0..) {
                    if has_outstanding_todo(&child) {
                        todos.push(child);
                    } else {
                        done.push(child);
                    }
                }

                std::mem::swap(&mut done, children);

                if !todos.is_empty() {
                    if let Some(hdr) = last_header.take() {
                        front.push(hdr.clone());
                    }

                    front.push(make_list(opt.clone(), todos));
                }
            }

            _ => {}
        }
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
