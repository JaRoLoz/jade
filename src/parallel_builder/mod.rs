use crate::builder::build_step::BuildStep;
use std::{
    path::PathBuf,
    sync::Arc,
    thread::{self, JoinHandle},
};

#[derive(Debug)]
pub struct ParallelBuildStep {
    pub steps: Vec<Arc<Box<dyn BuildStep + Send + Sync>>>,
}

impl BuildStep for ParallelBuildStep {
    fn build(&self, base_path: &PathBuf) {
        self.steps
            .iter()
            .map(|step| {
                let step = Arc::clone(step);
                let path = base_path.clone();
                thread::spawn(move || {
                    step.build(&path);
                })
            })
            .collect::<Vec<JoinHandle<()>>>()
            .into_iter()
            .for_each(|handle| {
                handle.join().unwrap();
            });
    }
}
