use nv_parser::{AbstractSyntaxTree, Parser, SourceFileParser};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct IoError;

pub enum FileStoreError {
    IoError(IoError),
}

#[derive(Debug, Clone)]
pub struct StoredFile {
    pub path: String,
    pub content: String,
    pub tree: AbstractSyntaxTree,
}

#[derive(Debug, Clone)]
pub struct FileStore {
    files: HashMap<String, StoredFile>,
}

impl FileStore {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    fn parse(input: &str) -> AbstractSyntaxTree {
        let mut parser = SourceFileParser::new(input);

        parser.parse();

        parser.ast
    }

    fn read_file(file_path: &str) -> Result<String, FileStoreError> {
        std::fs::read_to_string(file_path).map_err(|_| FileStoreError::IoError(IoError))
    }

    fn store(&mut self, file_path: &str) -> Result<(), FileStoreError> {
        if self.files.contains_key(file_path) {
            return Ok(());
        }

        self.files.insert(
            file_path.to_owned(),
            StoredFile {
                tree: Self::parse(file_path),
                content: Self::read_file(file_path)?,
                path: file_path.to_owned(),
            },
        );

        Ok(())
    }

    pub fn get(&mut self, file_path: &str) -> Option<&StoredFile> {
        let _ = self.store(file_path);

        self.files.get(file_path)
    }
}
