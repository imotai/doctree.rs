//
// lib.rs
// Copyright (C) 2024 imotai <imotai@imotai-ub>
// Distributed under terms of the MIT license.
//

mod node;
use anyhow::Result;
use docx_rust::{document::BodyContent, DocxFile};
pub use node::TreeNode;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn parse_docx<P: AsRef<Path>>(path: P) -> Result {
    let doc_file = DocxFile::from_file(path)
        .map_err(|_| anyhow::anyhow!("fail to load file from path {}", path))?;
    let docx = doc_file
        .parse()
        .map_err(|_| anyhow::anyhow!("fail to parse docx from path {}", path))?;
    for (index, content) in docx.document.body.content.iter().enumerate() {
        match content {
            BodyContent::Paragraph(paragraph) => {
                let text = paragraph.text();
                println!("{}: {}", index, text);
            }
            _ => {}
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_docx() {
        parse_docx("/home/imotai/shekang.docx").unwrap();
    }
}
