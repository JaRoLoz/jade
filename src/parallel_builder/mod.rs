use crate::builder::build_step::BuildStep;
use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

#[derive(Debug)]
pub struct ParallelBuildStep {
    pub steps: Vec<Arc<Box<dyn BuildStep>>>,
}

impl BuildStep for ParallelBuildStep {
    fn build(&self, resource_name: &String) {
        self.steps
            .iter()
            .map(|step| {
                let step = Arc::clone(step);
                let resource_name = resource_name.clone();
                thread::spawn(move || step.build(&resource_name))
            })
            .collect::<Vec<JoinHandle<()>>>()
            .into_iter()
            .for_each(|handle| handle.join().unwrap());
    }
}
