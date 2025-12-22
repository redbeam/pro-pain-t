use leptos::prelude::*;

use crate::components::toolbar::Toolbar;
use crate::components::layer_panel::LayerPanel;
use crate::components::tool_palette::ToolPalette;
use crate::components::canvas_area::CanvasArea;
use crate::components::status_bar::StatusBar;

#[component]
pub fn App() -> impl IntoView {
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
                <LayerPanel />
            </div>
            <StatusBar />
        </div>
    }
}
