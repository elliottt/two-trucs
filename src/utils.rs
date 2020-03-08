use comrak::{
    nodes::{Ast, AstNode, NodeHeading, NodeList, NodeValue},
    Arena,
};
use core::cell::RefCell;

/// Construct a heading of the given level.
pub fn make_heading<'a>(arena: &'a Arena<AstNode<'a>>, level: u32, text: &str) -> &'a AstNode<'a> {
    let text = {
        let mut buf = Vec::new();
        buf.extend(text.as_bytes());
        let ast = Ast::new(NodeValue::Text(buf));
        arena.alloc(AstNode::new(RefCell::new(ast)))
    };

    let heading = {
        let mut heading = NodeHeading::default();
        heading.level = level;

        let ast = Ast::new(NodeValue::Heading(heading));
        arena.alloc(AstNode::new(RefCell::new(ast)))
    };

    heading.append(text);

    heading
}

/// Construct a list node.
pub fn make_list<'a>(arena: &'a Arena<AstNode<'a>>, ty: NodeList) -> &'a AstNode<'a> {
    let ast = Ast::new(NodeValue::List(ty));
    arena.alloc(AstNode::new(RefCell::new(ast)))
}

/// Returns true if the node is a heading.
pub fn is_heading<'a>(node: &'a AstNode<'a>) -> Option<NodeHeading> {
    if let NodeValue::Heading(heading) = node.data.borrow().value {
        Some(heading)
    } else {
        None
    }
}

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
                    return !b;
                }
            }
        }
    }

    false
}
