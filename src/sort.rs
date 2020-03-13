use crate::parse::{Doc, Node, Tag};

/// Bubble all unfinished tasks up to the top of each todo list encountered.
pub fn sort_tasks<'a>(doc: &mut Doc<'a>) {
    for node in doc.iter_mut() {
        if let Node::Node { tag, children } = node {
            if let Tag::List(_) = tag {
                sort_tasks(children);

                children.sort_by(|left,right| {
                    left.is_todo().cmp(&right.is_todo())
                })
            }
        }
    }
}
