use std::path::PathBuf;

pub struct Category(pub PathBuf);

impl Category {
    pub fn file_name(&self) -> &str {
        self.0
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_end_matches(".txt")
    }
}
