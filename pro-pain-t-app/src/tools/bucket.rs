use crate::structs::history::{PixelDiff, StrokeDiff};
use crate::structs::pixel::Pixel;
use leptos::prelude::{Get, Update};
use serde::{Deserialize, Serialize};

use crate::tools::context::ToolContext;
use crate::tools::geometry::screen_to_canvas;
use web_sys::PointerEvent;
use std::collections::VecDeque;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct BucketState {
    pub current_stroke: Vec<PixelDiff>,
}

impl BucketState {
    pub fn on_pointer_down(&mut self, _e: &PointerEvent, _ctx: &ToolContext) {
        self.current_stroke.clear();
    }

    pub fn on_pointer_move(&mut self, _e: &PointerEvent, _ctx: &ToolContext) {
        // don't do anything
    }

    pub fn on_pointer_up(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        let Some(layer_id) = ctx.selected_layer else { return; };

        let (x, y) = screen_to_canvas(
            ctx.canvas,
            e.client_x() as f64,
            e.client_y() as f64,
            ctx.zoom,
            ctx.pan_x,
            ctx.pan_y,
        );

        if x < 0
            || y < 0
            || x as u32 >= ctx.project.get().width.get()
            || y as u32 >= ctx.project.get().height.get()
        {
            return;
        }

        let fill_color = ctx.project.get().current_color.get();

        ctx.project.get().layers.update(|layers| {
            let Some(layer) = layers.iter_mut().find(|l| l.id == layer_id) else { return; };

            if layer.is_locked || !layer.is_visible {
                return;
            }

            let canvas = &mut layer.canvas;

            let ux = x as u32;
            let uy = y as u32;

            let start_pixel = canvas
                .get_pixel(ux, uy)
                .ok()
                .unwrap_or(Pixel::new(ux, uy, canvas.background_color));

            let target_color = start_pixel.color;

            if target_color == fill_color {
                return;
            }

            let mut queue = VecDeque::new();
            queue.push_back((x, y));

            let w = canvas.width as i32;
            let h = canvas.height as i32;

            while let Some((cx, cy)) = queue.pop_front() {
                if cx < 0 || cy < 0 || cx >= w || cy >= h {
                    continue;
                }

                let ux = cx as u32;
                let uy = cy as u32;

                let old_pixel = canvas
                    .get_pixel(ux, uy)
                    .ok()
                    .unwrap_or(Pixel::new(ux, uy, canvas.background_color));

                if old_pixel.color != target_color {
                    continue;
                }

                let new_pixel = Pixel::new(ux, uy, fill_color);

                self.current_stroke.push(PixelDiff {
                    before: old_pixel.clone(),
                    after: new_pixel.clone(),
                });

                let _ = canvas.set_pixel(new_pixel);

                queue.push_back((cx + 1, cy));
                queue.push_back((cx - 1, cy));
                queue.push_back((cx, cy + 1));
                queue.push_back((cx, cy - 1));
            }
        });

        if self.current_stroke.is_empty() {
            return;
        }

        ctx.project.update(|project| {
            project.history.add(StrokeDiff {
                layer_id,
                pixels: std::mem::take(&mut self.current_stroke),
            });
        });
    }

    pub fn cancel(&mut self) {
        self.current_stroke.clear();
    }

    pub fn cursor(&self) -> &'static str {
        "cell"
    }
}
