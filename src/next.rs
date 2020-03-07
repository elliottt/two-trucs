
use comrak::{Arena,nodes::AstNode};

use crate::utils;

pub fn start_next_day<'a>(arena: &'a Arena<AstNode<'a>>, doc: &'a AstNode<'a>) {
    for node in doc.children() {
        println!("node: {:?}", node);
    }
}
