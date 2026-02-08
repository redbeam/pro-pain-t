use crate::components::canvas_area::CanvasArea;
use crate::components::canvas_size_window::CanvasSizeWindow;
use crate::components::layer_panel::LayerPanel;
use crate::components::new_project_window::NewProjectWindow;
use crate::components::status_bar::StatusBar;
use crate::components::tool_palette::ToolPalette;
use crate::events::listeners::{
    canvas_size_listener, create_new_project_listener, export_project_listener,
    import_as_layer_listener, open_project_listener, save_project_listener,
};
use crate::state::workspace_state::WorkspaceState;
use crate::structs::project::Project;
use crate::view_state::ProjectViewState;
use leptos::html::Dialog;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let project = RwSignal::new(Project::default());
    let view_state = ProjectViewState::new();
    let workspace_state = WorkspaceState::new();

    provide_context(project);
    provide_context(view_state);
    provide_context(workspace_state);

    let new_project_window_ref: NodeRef<Dialog> = NodeRef::new();
    let is_new_project_window_open = RwSignal::new(false);

    let is_canvas_size_open = RwSignal::new(false);

    create_new_project_listener(is_new_project_window_open);
    open_project_listener(project);
    save_project_listener(project);
    import_as_layer_listener(project);
    export_project_listener(project);
    canvas_size_listener(is_canvas_size_open);

    view! {
        <div class="app-root">
            <div class="app-main">
                <ToolPalette />
                <div class="app-canvas-wrapper">
                    <CanvasArea />
                </div>
                <LayerPanel />
            </div>
            <StatusBar
                is_open = is_canvas_size_open
            />
            <CanvasSizeWindow
                is_open = is_canvas_size_open
            />
            <NewProjectWindow
                dialog_ref = new_project_window_ref
                is_open = is_new_project_window_open
            />
        </div>
    }
}
