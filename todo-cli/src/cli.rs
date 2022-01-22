use std::path::PathBuf;

pub enum Action {
    Add { task: String },
    Done { position: usize },
    List,
}
