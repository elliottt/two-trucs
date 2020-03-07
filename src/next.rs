use comrak::{nodes::AstNode, Arena};
use failure::Error;

use crate::utils;

pub fn start_next_day<'a>(
    arena: &'a Arena<AstNode<'a>>,
    doc: &'a AstNode<'a>,
) -> Result<(), Error> {
    let mut moved: Vec<&'a AstNode<'a>> = Vec::new();

    for node in doc.children() {
        if let Some(ty) = utils::is_list(node) {
            let mut todos = Vec::new();
            for child in node.children() {
                if utils::is_todo(child) {
                    child.detach();
                    todos.push(child)
                }
            }

            if !todos.is_empty() {
                if let Some(pre) = node.previous_sibling() {
                    if let Some(heading) = utils::is_heading(pre) {
                        let bytes = &pre.data.borrow().content;
                        let text = std::str::from_utf8(bytes)?;
                        moved.push(utils::make_heading(arena, heading.level, text));
                    }
                }

                let list = utils::make_bullet_list(arena, ty.bullet_char);
                for todo in todos {
                    list.append(todo);
                }

                moved.push(list)
            }
        }
    }

    if !moved.is_empty() {
        for node in moved.iter().rev() {
            doc.prepend(node);
        }
    }

    doc.prepend(utils::make_heading(arena, 1, "Today"));

    Ok(())
}
