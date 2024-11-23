use crate::network::Network;
use easy_node::{Nr, Nw};
use ndeq::prelude::*;
use ref_iter::prelude::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Default)]
pub struct Node {
    weak: Nw<Self>,
    network: Nw<Network>,
    value: RefCell<f32>,
    edges: RefCell<BTreeMap<Nw<Node>, f32>>,
}

impl Node {
    pub fn new(network: Nw<Network>) -> Nr<Self> {
        Nr::new_cyclic(|weak| {
            let weak = weak.clone();
            Self {
                weak,
                network,
                ..Default::default()
            }
        })
    }

    pub fn conv(this: &Nr<Node>) -> Rc<dyn NdeqNode<f32>> {
        Nr::bp(this).clone() as Rc<dyn NdeqNode<f32>>
    }

    pub fn value(&self) -> f32 {
        *self.value.borrow()
    }

    pub fn set_value(&self, value: f32) {
        *self.value.borrow_mut() = value
    }

    pub fn edges(&self) -> impl RefKvIterator<K = Nw<Node>, V = f32> + '_ {
        RefIter::new(self.edges.borrow(), |x| x.iter())
    }

    pub fn set_edge(&self, node: &Nr<Node>, w: f32) {
        assert!(self.this() != *node);
        assert!(self.network == node.network);
        let mut self_edges = self.edges.borrow_mut();
        let mut node_edges = node.edges.borrow_mut();
        self_edges.insert(Nr::downgrade(&node), w);
        node_edges.insert(Nr::downgrade(&self.this()), w);
    }

    fn this(&self) -> Nr<Self> {
        self.weak.upgrade().unwrap()
    }
}

impl NdeqNode<f32> for Node {
    fn value(&self) -> f32 {
        self.value()
    }

    fn set_value(&self, value: f32) {
        self.set_value(value);
    }

    fn in_edges(&self) -> Box<dyn Iterator<Item = (Rc<dyn NdeqNode<f32>>, f32)> + '_> {
        let edges = self.edges();
        let as_tuple = move |n: &Nw<Node>, w: &f32| {
            let nr = &n.upgrade().unwrap();
            let rc = Node::conv(&nr);
            (rc, *w)
        };

        Box::new(edges.map(as_tuple))
    }
}
