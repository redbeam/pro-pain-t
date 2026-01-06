use leptos::prelude::*;
use pro_pain_t_app::structs::color::Color;
use pro_pain_t_app::structs::project::Project;

use crate::components::layer_panel::LayerPanel;
use crate::components::tool_palette::ToolPalette;
use crate::components::canvas_area::CanvasArea;
use crate::components::status_bar::StatusBar;
use crate::components::canvas_size_window::CanvasSizeWindow;

#[component]
pub fn App() -> impl IntoView {
    let project = Project::new("test".to_string(), 400, 300, Color::new(0, 0, 0, 0));

    let canvas_width = project.width;
    let canvas_height = project.height;
    let layers = project.layers;

    let is_canvas_size_open = RwSignal::new(false);

    let open_canvas_size = move || {
        is_canvas_size_open.set(true);
    };

    let on_resize_canvas = move |new_w: u32, new_h: u32| {
        layers.update(|layers_vec| {
            for layer in layers_vec.iter_mut() {
                layer.resize_canvas(new_w, new_h);
            }
        });

        canvas_width.set(new_w);
        canvas_height.set(new_h);
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
                <CanvasArea
                    canvas_width = canvas_width
                    canvas_height = canvas_height
                />
                <LayerPanel
                    canvas_width = canvas_width
                    canvas_height = canvas_height
                    layers = layers
                    layer_id = project.next_layer_id/>
            </div>
            <StatusBar
                canvas_width = canvas_width
                canvas_height = canvas_height
                on_open_canvas_size = open_canvas_size
            />
            <CanvasSizeWindow
                is_open = is_canvas_size_open
                canvas_width = canvas_width
                canvas_height = canvas_height
                on_confirm = on_resize_canvas
            />
        </div>
    }
}
