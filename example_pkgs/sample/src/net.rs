use crate::node::Node;
use easy_node::prelude::*;
use ndeq::prelude::*;
use ref_iter::prelude::*;
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

    fn node_edges(node: &Nr<Node>) -> impl Iterator<Item = (usize, usize, f32)> + '_ {
        node.edges().map(|(i, w)| (node.idx(), i, w))
    }
}

impl NdeqNet<f32> for Net {
    fn edges(&self) -> Box<dyn Iterator<Item = (usize, usize, f32)> + '_> {
        let nodes = RefIter::new(self.nodes.borrow(), |x| x.iter());
        let ret = nodes.iflat_map(Self::node_edges);
        return Box::new(ret);
    }

    fn export_values(&self, values: &mut Vec<f32>) {
        values.clear();
        for (i, node) in self.nodes.borrow().iter().enumerate() {
            node.set_idx(i);
            values.push(node.value());
        }
    }

    fn import_values(&self, values: &[f32]) {
        assert_eq!(values.len(), self.nodes.borrow().len());

        for node in self.nodes.borrow_mut().iter() {
            let value = values[node.idx()];
            node.set_value(value);
        }
    }
}
