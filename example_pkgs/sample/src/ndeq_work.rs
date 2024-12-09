use ndeq::NdeqNet;

pub struct NdeqWork {
    pub(crate) net: NdeqNet<f32>,
    pub(crate) update: Box<dyn FnMut(&NdeqNet<f32>)>,
}
