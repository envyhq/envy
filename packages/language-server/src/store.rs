use nv_lexer::{Lexer, SourceFileLexer};
use nv_parser::{AbstractSyntaxNode, SourceFileParser};
use nv_position_indexer::Indexer;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub struct IoError;

pub enum FileStoreError {
    IoError(IoError),
}

#[derive(Debug, Clone)]
pub struct StoredFile {
    pub path: String,
    pub content: String,
    pub position_indexer: Indexer,
    pub root: Arc<AbstractSyntaxNode>,
}

#[derive(Debug, Clone, Default)]
pub struct FileStore {
    files: HashMap<String, StoredFile>,
}

impl FileStore {
    fn parse(content: &str) -> Arc<AbstractSyntaxNode> {
        let mut lexer = SourceFileLexer::new(content);
        lexer.lex();

        let (_, ast) = SourceFileParser::parse(&lexer.tokens);

        ast
    }

    fn read_file(file_path: &str) -> Result<String, FileStoreError> {
        std::fs::read_to_string(file_path).map_err(|_| FileStoreError::IoError(IoError))
    }

    fn init_position_indexer(node: &Arc<AbstractSyntaxNode>) -> Indexer {
        let mut indexer = Indexer::default();

        // TODO: Make lazy
        indexer.index(node);

        indexer
    }

    fn store(&mut self, file_path: &str) -> Result<(), FileStoreError> {
        if self.files.contains_key(file_path) {
            return Ok(());
        }

        let content = Self::read_file(file_path)?;

        let node = Self::parse(&content);

        self.files.insert(
            file_path.to_owned(),
            StoredFile {
                root: node.clone(),
                content,
                path: file_path.to_owned(),
                position_indexer: Self::init_position_indexer(&node),
            },
        );

        Ok(())
    }

    pub fn get(&mut self, file_path: &str) -> Option<&StoredFile> {
        let _ = self.store(file_path);

        self.files.get(file_path)
    }
}
