use crate::structs::project::Project;
use crate::tools::{pen::PenState, select::{commit_selection, SelectionState}, tools::Tool};
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct WorkspaceState {
    pub selected_layer_id: RwSignal<Option<usize>>,
    pub current_tool: RwSignal<Tool>,
    pub selection: RwSignal<Option<SelectionState>>,
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self {
            selected_layer_id: RwSignal::new(Some(0)),
            current_tool: RwSignal::new(Tool::Pen(PenState::default())),
            selection: RwSignal::new(None),
        }
    }
}

impl WorkspaceState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_tool(&self, tool: Tool, project: &RwSignal<Project>) {
        self.selection.with(|sel| {
            if let Some(sel) = sel {
                commit_selection(project, sel);
            }
        });
        self.selection.set(None);
        self.current_tool.set(tool);
    }
}
