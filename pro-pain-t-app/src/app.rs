use crate::components::canvas_area::CanvasArea;
use crate::components::canvas_size_window::CanvasSizeWindow;
use crate::components::layer_panel::LayerPanel;
use crate::components::new_project_window::NewProjectWindow;
use crate::components::status_bar::StatusBar;
use crate::components::tool_palette::ToolPalette;
use crate::view_state::ProjectViewState;
use leptos::html::Dialog;
use leptos::prelude::*;
use pro_pain_t_app::structs::project::Project;
use std::{env, fs};

#[component]
pub fn App() -> impl IntoView {
    let project = RwSignal::new(Project::default());
    let view_state = ProjectViewState::new();

    provide_context(project);
    provide_context(view_state);

    let new_project_window_ref: NodeRef<Dialog> = NodeRef::new();
    let is_new_project_window_open = RwSignal::new(false);
    //is_new_project_window_open.set(true); TODO :(

    let is_canvas_size_open = RwSignal::new(false);

    let _open_project_handler = || {
        let project_file_path = String::new(); // TODO receive from tauri event
        let _project_file_data = fs::read(project_file_path).expect("Failed to read file");
        let project_loaded = Project::default(); // TODO deserialize and load
        project.update(|project| {
            project.replace_project_with_blank(project_loaded.name, project_loaded.width.get(), project_loaded.height.get(), project_loaded.background_color);
        });
    };

    let _save_project_handler = || {
        let project_file_save_path = String::new(); // TODO receive from event
        let project_serialized: Vec<u8> = Vec::new(); // TODO serialize project
        fs::write(project_file_save_path, project_serialized).expect("Failed to write to file");
    };

    let _autosave_handler = || {
        let _autosave_path = env::temp_dir().set_file_name(format!("{}_autosave.ppp", project.get().name));
        // TODO call save project handler with this path
    };

    view! {
        <div
            style="
                display:flex;
                flex-direction:column;
                height:100vh;
                margin:0;
                background:#111;
            "
        >
            <div
                style="
                    flex:1;
                    display:flex;
                    min-height:0;
                "
            >
                <ToolPalette />
                <CanvasArea />
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
