use leptos::prelude::*;
use pro_pain_t_app::structs::color::Color;
use pro_pain_t_app::structs::project::Project;

use crate::components::toolbar::Toolbar;
use crate::components::layer_panel::LayerPanel;
use crate::components::tool_palette::ToolPalette;
use crate::components::canvas_area::CanvasArea;
use crate::components::status_bar::StatusBar;

#[component]
pub fn App() -> impl IntoView {
    let project = Project::new("test".to_string(), 800, 600, Color::new(255, 255, 255, 1.0));

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
            <Toolbar />
            <div
                style="
                    flex:1;
                    display:flex;
                    min-height:0;
                "
            >
                <ToolPalette />
                <CanvasArea />
                <LayerPanel
                    canvas_width = project.width
                    canvas_height = project.height
                    layers = project.layers
                    layer_id = project.next_layer_id/> // TODO: udpate based on the value set in the project! This is just a placeholder
            </div>
            <StatusBar />
        </div>
    }
}
