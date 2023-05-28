use crate::ElementId;
use crate::GuiTreeNode;
use crate::{Position, Size, Bounds};

use std::vec::*;
use std::collections::{VecDeque};
use std::option::Option;
use std::str::FromStr;

pub struct GuiContext {
    elements: Vec<ElementId>,
    free_id_queue: VecDeque<ElementId>,
    hierarchy: Vec<GuiTreeNode>,
    bounds: Vec<Bounds>
}

impl GuiContext {
    pub fn new() -> GuiContext {
        let elem: Vec<ElementId> = Vec::new();
        let mut q: VecDeque<ElementId> = VecDeque::new();

        q.push_front(ElementId::new(0, 1));

        GuiContext {
            elements: elem,
            free_id_queue: q,
            hierarchy: Vec::new(),
            bounds: Vec::new()
        }
    }

    pub fn add_elem(&mut self, parent: Option<ElementId>, b: Bounds) -> ElementId {
        let id = self.free_id_queue.pop_front().unwrap();
        let mut node = GuiTreeNode::new();
        match parent {
            Some(parent_id) => {
                node.with_parent(parent_id);
                self.hierarchy[parent_id.id as usize].with_child(id);
            },
            None => {}
        };

        if self.elements.len() > id.id as usize {
            self.elements[id.id as usize] = id;
            self.hierarchy[id.id as usize] = node;
            self.bounds[id.id as usize] = b;
        } else {
            self.elements.push(id);
            self.hierarchy.push(node);
            self.bounds.push(b);
        }

        if self.free_id_queue.len() == 0 {
            self.free_id_queue.push_back(ElementId::new(self.elements.len() as u32, 1));
        }

        id
    }

    pub fn delete(&mut self, id: ElementId) -> Result<(), String> {
        if (id.id as usize) < self.elements.len() && id.gen == self.elements[id.id as usize].gen {

            if id.gen == std::u32::MAX {
                self.free_id_queue.push_front(ElementId::new(id.id, 1));
            } else {
                self.free_id_queue.push_front(ElementId::new(id.id, id.gen + 1));
            }

            self.elements[id.id as usize].gen = 0;
            let parent = self.hierarchy[id.id as usize].parent();
            match parent {
                Some(p) => {
                    let mut parent_node = self.hierarchy[p.id as usize].clone();
                    parent_node.remove_child(id);
                },
                None => {}
            };
            self.hierarchy[id.id as usize] = GuiTreeNode::new();
            self.bounds[id.id as usize] = Bounds::new(Position::Absolute((0, 0)), Size::Absolute((0, 0)));

            Ok(())
        } else {
            Err(String::from_str("Not a valid Element ID").unwrap())
        }
    }

    pub fn get_data(&self) -> (&Vec<ElementId>, &Vec<GuiTreeNode>, &Vec<Bounds>) {
        (&self.elements, &self.hierarchy, &self.bounds)
    }
}
