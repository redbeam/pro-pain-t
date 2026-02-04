use crate::structs::pixel::Pixel;
use leptos::prelude::{Get, Update};
use serde::{Deserialize, Serialize};

use crate::tools::context::ToolContext;
use crate::tools::geometry::screen_to_canvas;
use web_sys::PointerEvent;

/// Bresenham line
fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, mut plot: impl FnMut(i32, i32)) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        plot(x, y);

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PenState {
    pub is_drawing: bool,
    pub last_pos: Option<(i32, i32)>,
}

impl PenState {
    pub fn on_pointer_down(&mut self, _e: &PointerEvent, _ctx: &ToolContext) {
        self.is_drawing = true;
        self.last_pos = None;
    }

    pub fn on_pointer_up(&mut self, _e: &PointerEvent, _ctx: &ToolContext) {
        self.is_drawing = false;
        self.last_pos = None;
    }

    pub fn on_pointer_move(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        if !self.is_drawing {
            return;
        }

        let Some(layer_id) = ctx.selected_layer else {
            return;
        };

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

        let color = ctx.project.get().current_color.get();
        let current = (x, y);

        ctx.project.get().layers.update(|layers| {
            let Some(layer) = layers.iter_mut().find(|l| l.id == layer_id) else {
                return;
            };

            if layer.is_locked || !layer.is_visible {
                return;
            }

            let canvas = &mut layer.canvas;

            if let Some((lx, ly)) = self.last_pos {
                draw_line(lx, ly, x, y, |px, py| {
                    if px < 0 || py < 0 {
                        return;
                    }

                    let _ = canvas.set_pixel(Pixel {
                        x: px as u32,
                        y: py as u32,
                        color,
                    });
                });
            } else {
                let _ = canvas.set_pixel(Pixel {
                    x: x as u32,
                    y: y as u32,
                    color,
                });
            }
        });

        self.last_pos = Some(current);
    }

    pub fn cancel(&mut self) {
        self.is_drawing = false;
        self.last_pos = None;
    }

    pub fn cursor(&self) -> &'static str {
        "default"
    }
}
