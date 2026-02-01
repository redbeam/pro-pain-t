use crate::structs::{color::Color, layer::Layer};
use web_sys::{CanvasRenderingContext2d, wasm_bindgen::JsValue};

#[derive(Clone, Copy, Debug)]
pub struct ViewTransform {
    pub zoom: f32,
    pub pan_x: f32,
    pub pan_y: f32,
    pub dpr: f64,
}

pub fn draw_checkerboard(ctx: &CanvasRenderingContext2d, width: u32, height: u32, tile_size: u32) {
    let light = "#e0e0e0";
    let dark = "#b0b0b0";

    for y in (0..height).step_by(tile_size as usize) {
        for x in (0..width).step_by(tile_size as usize) {
            let is_dark = (x / tile_size + y / tile_size).is_multiple_of(2);
            let color = if is_dark { dark } else { light };

            #[allow(deprecated)]
            ctx.set_fill_style(&JsValue::from_str(color));

            ctx.fill_rect(x as f64, y as f64, tile_size as f64, tile_size as f64);
        }
    }
}

fn blend(dst: Color, src: Color) -> Color {
    let sa = src.alpha.clamp(0.0, 1.0);
    let da = dst.alpha.clamp(0.0, 1.0);

    let out_a = sa + da * (1.0 - sa);

    if out_a == 0.0 {
        return Color {
            r: 0,
            g: 0,
            b: 0,
            alpha: 0.0,
        };
    }

    let sr = src.r as f32 / 255.0;
    let sg = src.g as f32 / 255.0;
    let sb = src.b as f32 / 255.0;

    let dr = dst.r as f32 / 255.0;
    let dg = dst.g as f32 / 255.0;
    let db = dst.b as f32 / 255.0;

    let r = (sr * sa + dr * da * (1.0 - sa)) / out_a;
    let g = (sg * sa + dg * da * (1.0 - sa)) / out_a;
    let b = (sb * sa + db * da * (1.0 - sa)) / out_a;

    Color {
        r: (r * 255.0) as u8,
        g: (g * 255.0) as u8,
        b: (b * 255.0) as u8,
        alpha: out_a,
    }
}

pub fn composite_layers(layers: &[Layer]) -> (Vec<u8>, u32, u32) {
    let base_canvas = &layers[0].canvas;
    let width = base_canvas.width;
    let height = base_canvas.height;

    let mut out = vec![
        Color {
            r: base_canvas.background_color.r,
            g: base_canvas.background_color.g,
            b: base_canvas.background_color.b,
            alpha: base_canvas.background_color.alpha,
        };
        (width * height) as usize
    ];

    for layer in layers {
        if !layer.is_visible {
            continue;
        }

        for pixel in &layer.canvas.content {
            let idx = (pixel.y * width + pixel.x) as usize;

            let dst = out[idx];
            let src = pixel.color;

            out[idx] = blend(dst, src);
        }
    }

    let mut bytes = Vec::with_capacity((width * height * 4) as usize);

    for c in out {
        bytes.push(c.r);
        bytes.push(c.g);
        bytes.push(c.b);
        bytes.push((c.alpha * 255.0) as u8);
    }

    (bytes, width, height)
}

/// Draw a project-sized pixel buffer into a viewport-sized canvas using pan+zoom.
/// - `viewport_w/h` are in *device pixels* (canvas.width/canvas.height)
/// - `proj_w/h` are in project pixels
pub fn draw_project_viewport(
    ctx: &CanvasRenderingContext2d,
    viewport_w: u32,
    viewport_h: u32,
    pixels: &[u8],
    proj_w: u32,
    proj_h: u32,
    t: ViewTransform,
) {
    let _ = ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    ctx.clear_rect(0.0, 0.0, viewport_w as f64, viewport_h as f64);
    ctx.set_fill_style_str("#ccc");
    ctx.fill_rect(0.0, 0.0, viewport_w as f64, viewport_h as f64);

    let scale = (t.zoom as f64) * t.dpr;
    let tx = (t.pan_x as f64) * t.dpr;
    let ty = (t.pan_y as f64) * t.dpr;
    let _ = ctx.set_transform(scale, 0.0, 0.0, scale, tx, ty);
    ctx.set_image_smoothing_enabled(false);

    draw_checkerboard(ctx, proj_w, proj_h, 8);

    for y in 0..proj_h {
        for x in 0..proj_w {
            let i = ((y * proj_w + x) * 4) as usize;
            let r = pixels[i];
            let g = pixels[i + 1];
            let b = pixels[i + 2];
            let a = pixels[i + 3] as f64 / 255.0;

            if a == 0.0 {
                continue;
            }

            ctx.set_global_alpha(a);
            ctx.set_fill_style_str(&format!("rgb({},{},{})", r, g, b));
            ctx.fill_rect(x as f64, y as f64, 1.0, 1.0);
        }
    }

    ctx.set_global_alpha(1.0);
}
