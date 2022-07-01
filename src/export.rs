use std::path::PathBuf;

pub struct Worker {
    pub source: Option<String>,
    pub out_file: Option<String>,
    pub source_path: Option<PathBuf>
}

impl Worker {

    /// Main calling method for struct
    pub fn exec(&self) {

    }
}
