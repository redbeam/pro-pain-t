use leptos::prelude::*;
use leptos::*;
use pro_pain_t_app::structs::{layer::Layer, project::Project};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, wasm_bindgen::{Clamped, JsCast}};

use crate::view_state::ProjectViewState;

fn generate_checkerboard(width: u32, height: u32, tile_size: u32) -> Vec<u8> {
    let mut pixels = vec![0u8; (width * height * 4) as usize];

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            let is_light = (x / tile_size + y / tile_size).is_multiple_of(2);
            let (r, g, b) = if is_light { (0xe0, 0xe0, 0xe0) } else { (0xb0, 0xb0, 0xb0) };

            pixels[idx] = r;
            pixels[idx + 1] = g;
            pixels[idx + 2] = b;
            pixels[idx + 3] = 255;
        }
    }

    pixels
}

fn composite_layers(
    layers: &[Layer],
    width: u32,
    height: u32,
    tile_size: u32,
) -> Vec<u8> {
    let len = (width * height * 4) as usize;

    let mut out = generate_checkerboard(width, height, tile_size);

    for layer in layers.iter().filter(|l| l.is_visible) {
        let src = &layer.canvas.content;
        for i in (0..len).step_by(4) {
            let sa = src[i + 3] as f32 / 255.0;
            if sa == 0.0 { continue; }
            let inv = 1.0 - sa;

            out[i]     = (src[i]     as f32 * sa + out[i]     as f32 * inv) as u8;
            out[i + 1] = (src[i + 1] as f32 * sa + out[i + 1] as f32 * inv) as u8;
            out[i + 2] = (src[i + 2] as f32 * sa + out[i + 2] as f32 * inv) as u8;
            out[i + 3] = 255;
        }
    }

    out
}

fn draw_to_canvas(ctx: &CanvasRenderingContext2d, pixels: &[u8], width: u32, height: u32) {
    let data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(pixels),
        width,
        height,
    ).unwrap();
    ctx.put_image_data(&data, 0.0, 0.0).unwrap();
}

#[component]
pub fn CanvasArea() -> impl IntoView {
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

        let width = project.width.get();
        let height = project.height.get();
        canvas.set_width(width);
        canvas.set_height(height);

        let layers = project.layers.get();

        let pixels = composite_layers(&layers, width, height, 8);

        draw_to_canvas(&ctx, &pixels, width, height);
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