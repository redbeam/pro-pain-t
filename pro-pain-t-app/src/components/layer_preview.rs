use leptos::{prelude::{Effect, Get, NodeRef, NodeRefAttribute, StyleAttribute}, *};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, wasm_bindgen::Clamped, window};
use wasm_bindgen::JsCast;
use pro_pain_t_app::structs::layer::Layer;

fn create_offscreen_canvas(width: u32, height: u32) -> HtmlCanvasElement {
    let document = window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas").unwrap()
        .dyn_into::<HtmlCanvasElement>().unwrap();
    canvas.set_width(width);
    canvas.set_height(height);
    canvas
}

fn generate_checkerboard(width: u32, height: u32, tile_size: u32) -> Vec<u8> {
    let mut pixels = vec![0u8; (width * height * 4) as usize];

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            let is_light = ((x / tile_size + y / tile_size) % 2) == 0;
            let (r, g, b) = if is_light { (0xe0, 0xe0, 0xe0) } else { (0xb0, 0xb0, 0xb0) };
            pixels[idx] = r;
            pixels[idx + 1] = g;
            pixels[idx + 2] = b;
            pixels[idx + 3] = 255;
        }
    }

    pixels
}

fn draw_rgba_over_checkerboard(
    ctx: &CanvasRenderingContext2d,
    pixels: &[u8],
    width: u32,
    height: u32,
    scale: f32,
) {
    ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
    ctx.clear_rect(0.0, 0.0, f64::INFINITY, f64::INFINITY);
    ctx.scale(scale as f64, scale as f64).unwrap();

    let checkerboard = generate_checkerboard(width, height, 4);
    let data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&checkerboard), width, height
    ).unwrap();
    ctx.put_image_data(&data, 0.0, 0.0).unwrap();

    let offscreen = create_offscreen_canvas(width, height);
    let off_ctx = offscreen
        .get_context("2d").unwrap().unwrap()
        .dyn_into::<CanvasRenderingContext2d>().unwrap();

    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(pixels), width, height
    ).unwrap();
    off_ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();

    ctx.set_image_smoothing_enabled(false);
    ctx.draw_image_with_html_canvas_element(&offscreen, 0.0, 0.0).unwrap();
}

#[component]
pub fn LayerPreview(layer: Layer) -> impl IntoView {
    let canvas_ref = NodeRef::new();

    Effect::new(move |_| {
        let canvas: HtmlCanvasElement = match canvas_ref.get() {
            Some(c) => c,
            None => return,
        };

        let ctx = canvas
            .get_context("2d").unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>().unwrap();

        let preview_size = 50.0;
        let scale = preview_size / layer.canvas.width as f32;

        canvas.set_width(preview_size as u32);
        canvas.set_height(preview_size as u32);

        draw_rgba_over_checkerboard(
            &ctx,
            &layer.canvas.content,
            layer.canvas.width,
            layer.canvas.height,
            scale
        );
    });

    view! {
        <canvas
            node_ref=canvas_ref
            style="
                width:100px;
                height:50px;
                image-rendering:pixelated;
                border-radius:2px;
            "
        />
    }
}
