use crate::node::Node;
use easy_node::{Nr, Nw};
use ref_iter::prelude::*;
use std::cell::RefCell;
use std::collections::BTreeSet;

#[derive(Default)]
pub struct Network {
    weak: Nw<Self>,
    nodes: RefCell<BTreeSet<Nr<Node>>>,
}

impl Network {
    pub fn new() -> Nr<Self> {
        Nr::new_cyclic(|weak| {
            let weak = weak.clone();
            Self {
                weak,
                ..Default::default()
            }
        })
    }

    pub fn nodes(&self) -> impl RefIterator<Item = Nr<Node>> + '_ {
        RefIter::new(self.nodes.borrow(), |x| x.iter())
    }

    pub fn add_node(&self) -> Nr<Node> {
        let ret = Node::new(self.weak.clone());
        let mut nodes = self.nodes.borrow_mut();
        nodes.insert(ret.clone());
        ret
    }
}
