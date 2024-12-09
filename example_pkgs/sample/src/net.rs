use crate::ndeq_work::NdeqWork;
use crate::node::Node;
use easy_node::prelude::*;
use ndeq::{NdeqNet, NdeqNode, NodeView};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

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

    pub fn create_ndeq_work(&self) -> NdeqWork {
        let nodes = self.create_nodes_vec();
        let mut ndeq_nodes = Vec::new();
        let mut key_to_index = BTreeMap::new();

        for (i, node) in nodes.iter().enumerate() {
            ndeq_nodes.push(NdeqNode::new(node.value()));
            key_to_index.insert(node.key(), i);
        }

        for (i, node) in nodes.iter().enumerate() {
            for (key, weight) in node.edges() {
                let index = key_to_index[&key];
                ndeq_nodes[i].add_edge(index, weight);
            }
        }

        let net = NdeqNet::from_nodes(ndeq_nodes);
        let update = Self::create_update(nodes, key_to_index);
        NdeqWork { net, update }
    }

    pub fn add_node(&self) -> Nr<Node> {
        let ret = Node::new(self.weak.clone());
        let mut nodes = self.nodes.borrow_mut();
        nodes.insert(ret.clone());
        ret
    }

    fn create_nodes_vec(&self) -> Vec<Nr<dyn NodeView<f32>>> {
        let nodes = self.nodes.borrow();
        nodes.iter().map(Node::conv).collect::<Vec<_>>()
    }

    fn create_update(
        nodes: Vec<Nr<dyn NodeView<f32>>>,
        key_to_index: BTreeMap<usize, usize>,
    ) -> Box<dyn FnMut(&NdeqNet<f32>)> {
        Box::new(move |net| {
            for node in nodes.iter() {
                let index = key_to_index[&node.key()];
                let value = net.nodes()[index].value();
                node.set_value(value);
            }
        })
    }
}
