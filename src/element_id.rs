use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, PartialOrd, Eq, Ord, Ordering};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ElementId {
    pub id : u32,
    pub gen: u32
}

impl ElementId {
    ///Creates a new ElementId with id and gen.
    pub fn new(id: u32, gen: u32) -> ElementId {
        ElementId {
            id: id,
            gen: gen
        }
    }

}

impl Hash for ElementId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let complete: u64 = ((self.id as u64) << 32) | ((self.gen as u64));
        complete.hash(state);
    }
}

impl PartialEq for ElementId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.gen == other.gen
    }
}

impl Eq for ElementId {}

impl PartialOrd for ElementId {
    fn partial_cmp(&self, other: &ElementId) -> Option<Ordering> {
        if self.id < other.id {
            Some(Ordering::Less)
        } else if self.id > other.id {
            Some(Ordering::Greater)
        } else {
            if self.gen < other.gen {
                Some(Ordering::Less)
            } else if self.gen > other.gen {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            }
        }
    }
}

impl Ord for ElementId {
    fn cmp(&self, other: &ElementId) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
