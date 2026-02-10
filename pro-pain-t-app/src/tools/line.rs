use crate::structs::history::{PixelDiff, StrokeDiff};
use crate::structs::pixel::Pixel;
use leptos::prelude::{Get, Update};
use serde::{Deserialize, Serialize};
use crate::tools::context::ToolContext;
use crate::tools::geometry::{draw_line, screen_to_canvas};
use web_sys::PointerEvent;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct LineState {
    start_point: Option<(i32, i32)>,
    current_stroke: Vec<PixelDiff>,
}

impl LineState {
    pub fn on_pointer_down(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        let (x, y) = screen_to_canvas(
            ctx.canvas,
            e.client_x() as f64,
            e.client_y() as f64,
            ctx.zoom,
            ctx.pan_x,
            ctx.pan_y,
        );

        if self.start_point.is_none() {
            self.start_point = Some((x, y));
        } else {
            let (sx, sy) = self.start_point.unwrap();
            let Some(layer_id) = ctx.selected_layer else { return; };
            let size = ctx.workspace_state.brush_size.get().max(0.1);
            let color = ctx.project.get().current_color.get();

            ctx.project.update(|project| {
                project.layers.update(|layers| {
                    let Some(layer) = layers.iter_mut().find(|l| l.id == layer_id) else { return; };
                    if layer.is_locked || !layer.is_visible { return; }

                    let canvas = &mut layer.canvas;
                    let mut plot_pixel = |px: i32, py: i32| {
                        let radius = size / 2.0;
                        for dy in -(radius as i32)..=(radius as i32) {
                            for dx in -(radius as i32)..=(radius as i32) {
                                let dist = ((dx as f32).powi(2) + (dy as f32).powi(2)).sqrt();
                                if dist > radius { continue; }

                                let nx = px + dx;
                                let ny = py + dy;
                                if nx < 0 || ny < 0 { continue; }
                                let ux = nx as u32;
                                let uy = ny as u32;
                                let old_pixel = canvas.get_pixel(ux, uy).unwrap_or(Pixel::new(ux, uy, canvas.background_color));
                                if old_pixel.color != color {
                                    self.current_stroke.push(PixelDiff {
                                        before: old_pixel,
                                        after: Pixel::new(ux, uy, color),
                                    });
                                    let _ = canvas.set_pixel(Pixel::new(ux, uy, color));
                                }
                            }
                        }
                    };

                    draw_line(sx, sy, x, y, &mut plot_pixel);
                });
            });

            if !self.current_stroke.is_empty() {
                let Some(layer_id) = ctx.selected_layer else { return; };
                ctx.project.update(|project| {
                    project.history.add(StrokeDiff {
                        layer_id,
                        pixels: std::mem::take(&mut self.current_stroke),
                    });
                });
            }

            self.start_point = None;
        }
    }

    pub fn on_pointer_move(&mut self, _e: &PointerEvent, _ctx: &ToolContext) {
    }

    pub fn on_pointer_up(&mut self, _e: &PointerEvent, _ctx: &ToolContext) {
        self.start_point = self.start_point.take();
        self.current_stroke.clear();
    }

    pub fn cancel(&mut self) {
        self.start_point = None;
        self.current_stroke.clear();
    }

    pub fn cursor(&self) -> &'static str {
        "crosshair"
    }
}
