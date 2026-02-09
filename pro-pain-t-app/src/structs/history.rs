use crate::structs::{pixel::Pixel, project::Project};
use leptos::prelude::{Get, RwSignal, Update};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Serialize, Deserialize)]
pub struct PixelDiff {
    pub before: Pixel,
    pub after: Pixel,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StrokeDiff {
    pub layer_id: usize,
    pub pixels: Vec<PixelDiff>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct History {
    max_count: usize,
    undo: VecDeque<StrokeDiff>,
    redo: VecDeque<StrokeDiff>,
}

impl History {
    pub fn new(max_count: usize) -> Self {
        Self {
            max_count,
            undo: VecDeque::with_capacity(max_count),
            redo: VecDeque::with_capacity(max_count),
        }
    }

    pub fn add(&mut self, stroke: StrokeDiff) {
        if self.undo.len() >= self.max_count {
            self.undo.pop_front();
        }
        self.undo.push_back(stroke);
        self.redo.clear();
    }

    pub fn undo(&mut self) -> Option<StrokeDiff> {
        let stroke = self.undo.pop_back()?;
        self.redo.push_back(stroke.clone());
        Some(stroke)
    }

    pub fn redo(&mut self) -> Option<StrokeDiff> {
        let stroke = self.redo.pop_back()?;
        self.undo.push_back(stroke.clone());
        Some(stroke)
    }

    pub fn apply_undo(&mut self, project: &RwSignal<Project>) {

        let mut stroke_opt = None;
        project.update(|project| {
            stroke_opt = project.history.undo();
        });

        let Some(stroke) = stroke_opt else { return };

        project.get().layers.update(|layers| {
            if let Some(layer) = layers.iter_mut().find(|l| l.id == stroke.layer_id) {
                let canvas = &mut layer.canvas;
                for diff in &stroke.pixels {
                    let _ = canvas.set_pixel(diff.before);
                }
            }
        });

    }

    pub fn apply_redo(&mut self, project: &RwSignal<Project>) {
     
        let mut stroke_opt = None;
        project.update(|project| {
            stroke_opt = project.history.redo();
        });

        let Some(stroke) = stroke_opt else { return };

        project.get().layers.update(|layers| {
            if let Some(layer) = layers.iter_mut().find(|l| l.id == stroke.layer_id) {
                let canvas = &mut layer.canvas;
                for diff in &stroke.pixels {
                    let _ = canvas.set_pixel(diff.after);
                }
            }
        });

    }


}
