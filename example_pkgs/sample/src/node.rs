use crate::net::Net;
use easy_node::prelude::*;
use ndeq::prelude::*;
use ref_iter::prelude::*;
use std::cell::RefCell;
use std::collections::BTreeMap;

pub struct Node {
    net: Nw<Net>,
    this: Nw<Self>,
    work_idx: RefCell<usize>,
    value: RefCell<f32>,
    edges: RefCell<BTreeMap<Nw<Self>, f32>>,
}

impl Node {
    pub fn new(net: Nw<Net>) -> Nr<Self> {
        Nr::new_cyclic(|this| Self {
            net,
            this: this.clone(),
            work_idx: Default::default(),
            value: Default::default(),
            edges: Default::default(),
        })
    }

    pub fn add_edge(&self, node: &Nr<Node>, w: f32) {
        assert!(self.this() != *node);
        assert!(self.net == node.net);
        let mut self_edges = self.edges.borrow_mut();
        let mut node_edges = node.edges.borrow_mut();
        self_edges.insert(Nr::downgrade(&node), w);
        node_edges.insert(Nr::downgrade(&self.this()), w);
    }

    fn this(&self) -> Nr<Self> {
        self.this.upgrade().unwrap()
    }
}

impl NdeqNode<f32> for Node {
    fn idx(&self) -> usize {
        *self.work_idx.borrow()
    }

    fn set_idx(&self, value: usize) {
        *self.work_idx.borrow_mut() = value;
    }

    fn value(&self) -> f32 {
        *self.value.borrow()
    }

    fn set_value(&self, value: f32) {
        *self.value.borrow_mut() = value
    }

    fn edges(&self) -> Box<dyn Iterator<Item = (usize, f32)> + '_> {
        let iter = RefIter::new(self.edges.borrow(), |x| x.iter());
        let ret = iter.imap(|k, v| {
            let idx = k.upgrade().unwrap().idx();
            let weight = *v;
            (idx, weight)
        });
        Box::new(ret)
    }
}
