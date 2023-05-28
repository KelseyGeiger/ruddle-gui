use crate::ElementId;

use std::vec::*;
use std::option::Option;

#[derive(Clone, Debug)]
pub struct GuiTreeNode {
    parent: Option<ElementId>,
    children: Vec<ElementId>
}

impl GuiTreeNode {
    pub fn new() -> GuiTreeNode {
        GuiTreeNode {
            parent: None,
            children: Vec::new()
        }
    }

    pub fn with_parent(&mut self, p: ElementId) -> &mut GuiTreeNode {
        self.parent = Some(p);

        self
    }

    pub fn with_child(&mut self, c: ElementId) -> &mut GuiTreeNode {
        self.children.push(c);

        self
    }

    pub fn with_children(&mut self, cn: &[ElementId]) -> &mut GuiTreeNode {
        self.children.extend_from_slice(cn);

        self
    }

    pub fn parent(&self) -> &Option<ElementId> {
        &self.parent
    }

    pub fn children(&self) -> &Vec<ElementId> {
        &self.children
    }

    pub fn remove_child(&mut self, c: ElementId) {
        let mut c_idx = self.children.len();
        for child in 0..self.children.len() {
            if c == self.children[child] {
                c_idx = child;
                break;
            }
        }
        if c_idx != self.children.len() {
            self.children.swap_remove(c_idx);
        }
    }
}
