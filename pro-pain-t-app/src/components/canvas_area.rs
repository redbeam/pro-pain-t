use leptos::prelude::*;
use leptos::*;
use pro_pain_t_app::structs::{color::Color, layer::Layer, project::Project};
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, wasm_bindgen::JsCast};

use crate::view_state::ProjectViewState;

#[inline]
fn blend(dst: Color, src: Color) -> Color {
    let sa = src.alpha.clamp(0.0, 1.0);
    let da = dst.alpha.clamp(0.0, 1.0);

    let out_a = sa + da * (1.0 - sa);

    if out_a == 0.0 {
        return Color { r: 0, g: 0, b: 0, alpha: 0.0 };
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

fn draw_to_canvas(
    ctx: &CanvasRenderingContext2d,
    pixels: &[u8],
    width: u32,
    height: u32,
    zoom: f32,
) {
    ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
    ctx.clear_rect(0.0, 0.0, f64::INFINITY, f64::INFINITY);

    ctx.scale(zoom as f64, zoom as f64).unwrap();

    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(pixels),
        width,
        height,
    ).unwrap();

    ctx.set_image_smoothing_enabled(false);
    ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();
}

#[component]
pub fn CanvasArea(
) -> impl IntoView {
    let canvas_ref = NodeRef::new();

    let project = use_context::<RwSignal<Project>>().unwrap().get();
    let view_state = use_context::<ProjectViewState>().expect("ProjectViewState context missing");

    Effect::new(move |_| {
        let canvas: HtmlCanvasElement = match canvas_ref.get() {
            Some(c) => c,
            None => return,
        };

        let ctx = canvas
            .get_context("2d").unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>().unwrap();

        let layers = project.layers.get();
        if layers.is_empty() {
            return;
        }

        let zoom = view_state.zoom_factor.get();
        let (pixels, width, height) = composite_layers(&layers);

        canvas.set_width(width);
        canvas.set_height(height);

        draw_to_canvas(&ctx, &pixels, width, height, zoom);
    });

    view! {
        <canvas
            node_ref=canvas_ref
            style=move || {
                let zoom = view_state.zoom_factor.get();
                format!(
                    "
                    width:{}px;
                    height:{}px;
                    image-rendering:pixelated;
                    background:#ccc;
                    ",
                    (project.width.get() as f32 * zoom),
                    (project.height.get() as f32 * zoom),
                )
            }
        />
    }
}