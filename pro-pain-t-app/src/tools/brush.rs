use crate::structs::history::{PixelDiff, StrokeDiff};
use crate::structs::pixel::Pixel;
use crate::tools::context::ToolContext;
use crate::tools::geometry::{draw_line, screen_to_canvas};
use leptos::prelude::{Get, Update};
use serde::{Deserialize, Serialize};
use web_sys::PointerEvent;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct BrushState {
    is_drawing: bool,
    last_pos: Option<(i32, i32)>,
    current_stroke: Vec<PixelDiff>,
}

impl BrushState {
    pub fn on_pointer_down(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        self.is_drawing = true;
        self.last_pos = None;
        self.current_stroke.clear();
        self.apply_at(e, ctx);
    }

    pub fn on_pointer_move(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        if !self.is_drawing {
            return;
        }
        self.apply_at(e, ctx);
    }

    pub fn on_pointer_up(&mut self, _e: &PointerEvent, ctx: &ToolContext) {
        self.is_drawing = false;
        self.last_pos = None;

        if self.current_stroke.is_empty() {
            return;
        }

        let Some(layer_id) = ctx.selected_layer else { return; };

        ctx.project.update(|project| {
            project.history.add(StrokeDiff {
                layer_id,
                pixels: std::mem::take(&mut self.current_stroke),
            });
        });
    }

    fn apply_at(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        let Some(layer_id) = ctx.selected_layer else { return; };

        let (x, y) = screen_to_canvas(
            ctx.canvas,
            e.client_x() as f64,
            e.client_y() as f64,
            ctx.zoom,
            ctx.pan_x,
            ctx.pan_y,
        );

        ctx.project.get().layers.update(|layers| {
            let Some(layer) = layers.iter_mut().find(|l| l.id == layer_id) else { return; };
            if layer.is_locked || !layer.is_visible { return; }

            let brush_size = ctx.workspace_state.brush_size.get().max(0.1);
            let radius = (brush_size * 2.0).ceil() as i32;
            let color = ctx.project.get().current_color.get();

            let canvas = &mut layer.canvas;

            let mut draw_pixel = |px: i32, py: i32| {
                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        if dx*dx + dy*dy > (radius*radius) { continue; }
                        let nx = px + dx;
                        let ny = py + dy;
                        if nx < 0 || ny < 0 { continue; }
                        let ux = nx as u32;
                        let uy = ny as u32;
                        if let Ok(old_pixel) = canvas.get_pixel(ux, uy) {
                            if old_pixel.color != color {
                                let new_pixel = Pixel::new(ux, uy, color);
                                self.current_stroke.push(PixelDiff {
                                    before: old_pixel,
                                    after: new_pixel,
                                });
                                let _ = canvas.set_pixel(new_pixel);
                            }
                        }
                    }
                }
            };

            let current = (x, y);
            if let Some((lx, ly)) = self.last_pos {
                draw_line(lx, ly, x, y, &mut draw_pixel);
            } else {
                draw_pixel(x, y);
            }

            self.last_pos = Some(current);
        });
    }

    pub fn cancel(&mut self) {
        self.is_drawing = false;
        self.last_pos = None;
        self.current_stroke.clear();
    }

    pub fn cursor(&self) -> &'static str {
        "crosshair"
    }
}
