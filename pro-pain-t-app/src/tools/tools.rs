use leptos::prelude::{RwSignal};
use serde::{Deserialize, Serialize};
use web_sys::HtmlCanvasElement;

use crate::{structs::project::Project, tools::pen::{PenState}};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tool {
    Pen(PenState),
}

impl Tool {
    pub fn on_mouse_down(&mut self) {
        match self {
            Tool::Pen(state) => state.pen_mouse_down(),
        }
    }

    pub fn on_mouse_move(&mut self, e: &web_sys::MouseEvent, canvas: &HtmlCanvasElement, zoom: f32, project: &RwSignal<Project>) {
        match self {
            Tool::Pen(state) => state.pen_mouse_move(e, project, canvas, zoom),
        }
    }

    pub fn on_mouse_up(&mut self) {
        match self {
                Tool::Pen(state) => state.pen_mouse_up(),
        }
    }
}