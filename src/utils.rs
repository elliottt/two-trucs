use comrak::nodes::{AstNode, NodeList, NodeValue};

/// Returns the bullet point if the given node represents a list.
pub fn is_list<'a>(node: &'a AstNode<'a>) -> Option<NodeList> {
    match node.data.borrow().value {
        NodeValue::List(ty) => Some(ty),
        _ => None,
    }
}

/// Returns true when this node is a task list item that is not complete.
pub fn is_todo<'a>(node: &'a AstNode<'a>) -> bool {
    match node.data.borrow().value {
        NodeValue::Item(_) => {
            if let Some(par) = node.first_child() {
                for child in par.children() {
                    match child.data.borrow().value {
                        NodeValue::TaskItem(b) => return !b,
                        _ => (),
                    }
                }
            }
        }

        _ => (),
    }

    false
}
