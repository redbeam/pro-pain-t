use leptos::prelude::{Get, RwSignal, Update};
use crate::structs::{pixel::Pixel, project::Project};

use crate::tools::geometry::screen_to_canvas;

/// Bresenham line
fn draw_line(
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    mut plot: impl FnMut(i32, i32),
) {
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

/// State for the pen tool
#[derive(Default)]
pub struct PenState {
    pub is_drawing: bool,
    pub last_pos: Option<(i32, i32)>,
}

pub fn pen_mouse_down(state: &mut PenState) {
    state.is_drawing = true;
    state.last_pos = None;
}

pub fn pen_mouse_up(state: &mut PenState) {
    state.is_drawing = false;
    state.last_pos = None;
}

pub fn pen_mouse_move(
    state: &mut PenState,
    e: &web_sys::MouseEvent,
    project: &RwSignal<Project>,
    canvas: &web_sys::HtmlCanvasElement,
    zoom: f32,
) {
    if !state.is_drawing {
        return;
    }

    let (x, y) = screen_to_canvas(canvas, e.client_x() as f64, e.client_y() as f64, zoom);

    if x < 0 || y < 0 {
        return;
    }

    let color = project.get().current_color.get();
    let current = (x, y);

    project.get().layers.update(|layers| {
        let layer_index = project.get().active_layer.get();

        let Some(layer) = layers.get_mut(layer_index) else {
            return;
        };

        if layer.is_locked || !layer.is_visible {
            return;
        }

        let canvas = &mut layer.canvas;

        if let Some((lx, ly)) = state.last_pos {
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

    state.last_pos = Some(current);
}
