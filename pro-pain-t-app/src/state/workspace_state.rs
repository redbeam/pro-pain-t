use crate::tools::{pen::PenState, tools::Tool};
use leptos::prelude::*;

#[derive(Clone)]
pub struct WorkspaceState {
    pub selected_layer_id: RwSignal<Option<usize>>,
    pub current_tool: RwSignal<Tool>,
    pub brush_size: RwSignal<f32>,
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self {
            selected_layer_id: RwSignal::new(Some(0)),
            current_tool: RwSignal::new(Tool::Pen(PenState::default())),
            brush_size: RwSignal::new(1.0),
        }
    }
}

impl WorkspaceState {
    pub fn new() -> Self {
        Self::default()
    }
}
