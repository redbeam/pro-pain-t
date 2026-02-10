use crate::tools::context::ToolContext;
use crate::tools::geometry::screen_to_canvas;
use leptos::prelude::{Get, Set, Update};
use serde::{Deserialize, Serialize};
use web_sys::PointerEvent;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct EyedropperState;

impl EyedropperState {
    pub fn on_pointer_down(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        let (x, y) = screen_to_canvas(
            ctx.canvas,
            e.client_x() as f64,
            e.client_y() as f64,
            ctx.zoom,
            ctx.pan_x,
            ctx.pan_y,
        );

        let Some(layer_id) = ctx.selected_layer else { return; };

        ctx.project.get().layers.update(|layers| {
            let Some(layer) = layers.iter().find(|l| l.id == layer_id) else { return; };

            if layer.is_locked || !layer.is_visible { return; }

            if x >= 0 && y >= 0 {
                let ux = x as u32;
                let uy = y as u32;

                if let Ok(pixel) = layer.canvas.get_pixel(ux, uy) {
                    ctx.project.update(|project| {
                        project.current_color.set(pixel.color);
                    });
                }
            }
        });
    }

    pub fn on_pointer_move(&mut self, _e: &PointerEvent, _ctx: &ToolContext) {
    }

    pub fn on_pointer_up(&mut self, _e: &PointerEvent, _ctx: &ToolContext) {
    }

    pub fn cancel(&mut self) {
    }

    pub fn cursor(&self) -> &'static str {
        "copy"
    }
}
