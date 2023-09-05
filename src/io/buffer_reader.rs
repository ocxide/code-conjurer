use std::{path::Path, fs::read_to_string};

trait BufferReader {
    fn read_content(&self, path: &Path) -> Result<String, std::io::Error>;
}

struct FileReader;
impl BufferReader for FileReader {
    fn read_content(&self, path: &Path) -> Result<String, std::io::Error> {
        read_to_string(path)
    }
}
