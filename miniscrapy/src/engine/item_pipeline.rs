use crate::ItemPipeline;

pub(crate) struct Pipeline {
    item_pipeline: Vec<Box<dyn ItemPipeline>>,
}

impl Pipeline {
    pub(crate) fn run(&mut self) {
        todo!()
    }
}
