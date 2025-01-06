#[derive(Debug, Clone)]
pub struct FileSystemBlob {
    file_path: String,
}
impl FileSystemBlob {
    pub fn new(file_path: String) -> Self {
        FileSystemBlob { file_path }
    }
    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }
}
