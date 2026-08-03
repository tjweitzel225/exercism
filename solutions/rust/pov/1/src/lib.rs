use std::{collections::BTreeSet, mem};

/// A rooted tree with unordered children.
///
/// Labels are assumed to be unique; two sibling subtrees that are wholly
/// identical collapse into one.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree<T> {
    pub val: T,
    pub children: BTreeSet<Self>,
}

impl<T: Ord> Tree<T> {
    pub fn new(label: T) -> Self {
        Self {
            val: label,
            children: BTreeSet::new(),
        }
    }

    /// Builder-method for constructing a tree with children
    pub fn with_child(mut self, child: Self) -> Self {
        self.children.insert(child);
        self
    }

    fn contains(&self, target: &T) -> bool {
        self.val == *target || self.children.iter().any(|c| c.contains(target))
    }

    pub fn pov_from(&mut self, from: &T) -> bool {
        while self.val != *from {
            let Some(mut child) = self.children.extract_if(.., |c| c.contains(from)).next() else {
                return false;
            };
            mem::swap(self, &mut child);
            self.children.insert(child);
        }
        true
    }

    pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        if !self.pov_from(from) {
            return None;
        }
        let mut node = &*self;
        let mut path = vec![&node.val];
        while node.val != *to {
            node = node.children.iter().find(|c| c.contains(to))?;
            path.push(&node.val);
        }
        Some(path)
    }
}
