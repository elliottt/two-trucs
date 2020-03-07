use comrak::nodes::AstNode;

use crate::utils;

pub fn sort_tasks<'a>(doc: &'a AstNode<'a>) {
    for child in doc.children() {
        if utils::is_list(child).is_some() {
            let (todo, done): (Vec<&'a AstNode<'a>>, Vec<&'a AstNode<'a>>) =
                child.children().partition(|node| {
                    node.detach();
                    utils::is_todo(node)
                });

            for node in todo {
                child.append(node);
            }

            for node in done {
                child.append(node);
            }
        }
    }
}
