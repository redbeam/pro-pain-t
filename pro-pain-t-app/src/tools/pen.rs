/*use leptos::prelude::{Get, RwSignal};

use crate::structs::{layer::Layer, color::Color, project::Project};

use crate::structs::pixel::Pixel;

#[derive(Clone, Copy)]
pub struct PenState {
    pub is_drawing: bool,
    pub last_x: i32,
    pub last_y: i32,
}

pub fn set_pixel(layer: &mut Layer, x: i32, y: i32, color: Color) {
    layer.canvas.content.retain(|p| !(p.x == x as u32 && p.y == y as u32));
    let _ = layer.canvas.set_pixel(Pixel {
        x: x as u32,
        y: y as u32,
        color,
    });
}

fn pen_draw(project: RwSignal<Project>, x: u32, y: u32) {

        let idx = project.get().active_layer.get();
        let layers = project.get().layers;

        let Some(&layer) = layers.get().get(idx) else { return };
        if layer.is_locked || !layer.is_visible {
            return;
        }

        layer.canvas.set_pixel(Pixel {
            x,
            y,
            color: project.get().current_color.get(),
        });
}

pub fn draw_line(
    layer: &mut Layer,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: Color,
) {
    // Bresenham
    let mut x = x0;
    let mut y = y0;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        set_pixel(layer, x, y, color);
        if x == x1 && y == y1 { break; }

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
}*/
