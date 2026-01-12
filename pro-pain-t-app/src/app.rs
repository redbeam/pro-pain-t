use crate::components::canvas_area::CanvasArea;
use crate::components::canvas_size_window::CanvasSizeWindow;
use crate::components::layer_panel::LayerPanel;
use crate::components::new_project_window::NewProjectWindow;
use crate::components::status_bar::StatusBar;
use crate::components::tool_palette::ToolPalette;
use leptos::html::Dialog;
use leptos::prelude::*;
use pro_pain_t_app::structs::project::Project;

#[component]
pub fn App() -> impl IntoView {
    let project = RwSignal::new(Project::default());

    provide_context(project);

    let new_project_window_ref: NodeRef<Dialog> = NodeRef::new();
    let is_new_project_window_open = RwSignal::new(false);
    //is_new_project_window_open.set(true); TODO :(

    let is_canvas_size_open = RwSignal::new(false);

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
