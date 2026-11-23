#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceBuffer {
    file_path: String,
    content: String,
}

impl SourceBuffer {
    pub fn new(file_path: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
            content: content.into(),
        }
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn line_count(&self) -> usize {
        self.content.lines().count().max(1)
    }
}
