use comrak::nodes::{AstNode, NodeList, NodeValue};

/// Returns the bullet point if the given node represents a list.
pub fn is_list<'a>(node: &'a AstNode<'a>) -> Option<NodeList> {
    if let NodeValue::List(ty) = node.data.borrow().value {
        Some(ty)
    } else {
        None
    }
}

/// Returns true when this node is a task list item that is not complete.
pub fn is_todo<'a>(node: &'a AstNode<'a>) -> bool {
    if let NodeValue::Item(_) = node.data.borrow().value {
        if let Some(par) = node.first_child() {
            for child in par.children() {
                if let NodeValue::TaskItem(b) = child.data.borrow().value {
                    return !b
                }
            }
        }
    }

    false
}
