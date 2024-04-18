//
// node.rs
// Copyright (C) 2024 imotai <imotai@imotai-ub>
// Distributed under terms of the MIT license.
//
use std::boxed::Box;

pub struct TreeNode {
    // the level of the paragraph
    paragraph_level: u32,
    // the content of the paragraph
    content: String,
    // the children of the paragraph
    children: Vec<Box<TreeNode>>,
    // the index of the paragraph
    paragraph_index: u32,
}

impl TreeNode {
    pub fn new(paragraph_level: u32, paragraph_index: u32, content: String) -> TreeNode {
        TreeNode {
            paragraph_level,
            content,
            children: Vec::new(),
            paragraph_index,
        }
    }

    pub fn add_child(&mut self, child: Box<TreeNode>) {
        self.children.push(child);
    }

    pub fn get_children(&self) -> &Vec<Box<TreeNode>> {
        &self.children
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_paragraph_level(&self) -> u32 {
        self.paragraph_level
    }
}
