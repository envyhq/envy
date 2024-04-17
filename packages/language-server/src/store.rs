use nv_lexer::{Lexer, SourceFileLexer};
use nv_parser::{AbstractSyntaxNode, SourceFileParser};
use nv_position_indexer::{Indexer, PositionIndex};
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

#[derive(Debug, Clone)]
pub struct IoError;

pub enum FileStoreError {
    IoError(IoError),
}

#[derive(Debug, Clone)]
pub struct StoredFile {
    pub path: String,
    pub content: String,
    pub position_index: PositionIndex,
    pub root: Weak<AbstractSyntaxNode>,
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

    fn parse(content: &str) -> Arc<AbstractSyntaxNode> {
        let mut lexer = SourceFileLexer::new(content);
        lexer.lex();

        let (_, ast) = SourceFileParser::parse(&lexer.tokens);

        ast
    }

    fn read_file(file_path: &str) -> Result<String, FileStoreError> {
        std::fs::read_to_string(file_path).map_err(|_| FileStoreError::IoError(IoError))
    }

    fn build_position_index(tree: Weak<AbstractSyntaxNode>) -> PositionIndex {
        let index = Indexer::index_node(tree);

        index
    }

    fn store(&mut self, file_path: &str) -> Result<(), FileStoreError> {
        if self.files.contains_key(file_path) {
            return Ok(());
        }

        let content = Self::read_file(file_path)?;

        let tree = Self::parse(&content);

        self.files.insert(
            file_path.to_owned(),
            StoredFile {
                root: Arc::downgrade(&tree),
                content: content,
                path: file_path.to_owned(),
                position_index: Self::build_position_index(Arc::downgrade(&tree)),
            },
        );

        Ok(())
    }

    pub fn get(&mut self, file_path: &str) -> Option<&StoredFile> {
        let _ = self.store(file_path);

        self.files.get(file_path)
    }
}
