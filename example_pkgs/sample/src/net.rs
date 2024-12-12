use crate::node::Node;
use easy_node::prelude::*;
use ref_iter::{RefIter, RefIterator};
use std::cell::RefCell;
use std::collections::BTreeSet;

#[derive(Default)]
pub struct Net {
    weak: Nw<Self>,
    nodes: RefCell<BTreeSet<Nr<Node>>>,
}

impl Net {
    pub fn new() -> Nr<Self> {
        Nr::new_cyclic(|weak| Self {
            weak: weak.clone(),
            ..Default::default()
        })
    }

    pub fn add_node(&self) -> Nr<Node> {
        let ret = Node::new(self.weak.clone());
        let mut nodes = self.nodes.borrow_mut();
        nodes.insert(ret.clone());
        ret
    }

    pub fn nodes(&self) -> impl Iterator<Item = Nr<Node>> + '_ {
        let nodes = RefIter::new(self.nodes.borrow(), |x| x.iter());
        nodes.cloned()
    }
}
