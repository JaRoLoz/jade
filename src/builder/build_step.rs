use std::fmt::Debug;

pub trait BuildStep: Debug + Send + Sync {
    // the resource_name is only passed for logging purposes
    fn build(&self, resource_name: &String);
}
