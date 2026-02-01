use crate::tools::{pen::PenState, tools::Tool};
use leptos::prelude::*;

#[derive(Clone)]
pub struct WorkspaceState {
    pub selected_layer_id: RwSignal<Option<usize>>,
    pub current_tool: RwSignal<Tool>,
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self {
            selected_layer_id: RwSignal::new(None),
            current_tool: RwSignal::new(Tool::Pen(PenState::default())),
        }
    }
}

impl WorkspaceState {
    pub fn new() -> Self {
        Self::default()
    }
}
