use crate::node::Node;
use easy_node::prelude::*;
use ndeq::prelude::*;
use ref_iter::prelude::*;
use std::cell::RefCell;

#[derive(Default)]
pub struct Net {
    weak: Nw<Self>,
    nodes: RefCell<Vec<Nr<Node>>>,
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
        nodes.insert(ret.idx(), ret.clone());
        ret
    }

    fn node_edges(node: &Nr<Node>) -> impl Iterator<Item = (usize, usize, f32)> + '_ {
        node.edges().map(|(i, w)| (node.idx(), i, w))
    }
}

impl NdeqFlow<f32> for Net {
    fn len(&self) -> usize {
        self.nodes.borrow().len()
    }

    fn edges(&self) -> Box<dyn Iterator<Item = (usize, usize, f32)> + '_> {
        let nodes = RefIter::new(self.nodes.borrow(), |x| x.iter());
        let ret = nodes.iflat_map(Self::node_edges);
        return Box::new(ret);
    }

    fn import_curr_values(&self, values: &[f32]) {
        assert_eq!(values.len(), self.nodes.borrow().len());
        for node in self.nodes.borrow_mut().iter_mut() {
            let value = values[node.idx()];
            node.set_value(value);
        }
    }

    fn export_last_values(&self, values: &mut Vec<f32>) {
        values.clear();
        for node in self.nodes.borrow().iter() {
            values.push(node.value());
        }
    }
}
