use std::collections::VecDeque;
use serde::{Deserialize, Serialize};
use crate::structs::pixel::Pixel;

#[derive(Clone, Serialize, Deserialize)]
pub struct History {
    max_count: usize,
    undo: VecDeque<(Vec<Pixel>, usize)>, // pixel diff, layer id
    redo: VecDeque<(Vec<Pixel>, usize)>, // pixel diff, layer id
}

impl History {
    pub fn new(max_count: usize) -> Self {
        Self {
            max_count,
            undo: VecDeque::with_capacity(max_count),
            redo: VecDeque::with_capacity(max_count),
        }
    }

    pub fn add(&mut self, pixel_diff: (Vec<Pixel>, usize)) {
        if self.undo.len() >= self.max_count {
            self.undo.pop_front();
        }
        self.undo.push_back(pixel_diff);
        self.redo.clear();
    }

    pub fn undo(&mut self) -> Result<(Vec<Pixel>, usize), String> {
        if self.undo.is_empty() {
            return Err("Undo stack is empty".to_string());
        }

        let diff = self.undo.pop_back().unwrap();

        self.redo.push_back(diff.clone());

        Ok(diff)
    }

    pub fn redo(&mut self) -> Result<(Vec<Pixel>, usize), String> {
        if self.redo.is_empty() {
            return Err("Redo stack is empty".to_string());
        }

        let diff = self.redo.pop_back().unwrap();

        self.undo.push_back(diff.clone());

        Ok(diff)
    }
}
