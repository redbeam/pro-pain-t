use leptos::{prelude::{Effect, Get, NodeRef, NodeRefAttribute, StyleAttribute}, *};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, wasm_bindgen::Clamped, window};
use wasm_bindgen::JsCast;
use pro_pain_t_app::structs::layer::Layer;

use crate::components::canvas_area::draw_checkerboard;


fn create_offscreen_canvas(width: u32, height: u32) -> HtmlCanvasElement {
    let document = window().unwrap().document().unwrap();

    let canvas = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    canvas.set_width(width);
    canvas.set_height(height);

    canvas
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

    draw_checkerboard(ctx, width, height, 12);

    let offscreen = create_offscreen_canvas(width, height);
    let off_ctx = offscreen
        .get_context("2d").unwrap().unwrap()
        .dyn_into::<CanvasRenderingContext2d>().unwrap();

    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(pixels),
        width,
        height,
    ).unwrap();

    off_ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();

    ctx.set_image_smoothing_enabled(false);
    ctx.draw_image_with_html_canvas_element(&offscreen, 0.0, 0.0).unwrap();
}


#[component]
pub fn LayerPreview(layer: Layer) -> impl IntoView {
    let mut width = 120;
    let mut height = 80;

    let htw_ratio = layer.canvas.height as f32 / layer.canvas.width as f32;
    if htw_ratio >= 1.5 {
        height = (120f32 / htw_ratio) as i32;
    }
    else {
        width = (80f32 * htw_ratio) as i32;
    }

    let mut width = width.to_string();
    width.push_str("px");
    let mut height = height.to_string();
    height.push_str("px");

    let canvas_ref = NodeRef::new();

    Effect::new(move |_| {
        let canvas: HtmlCanvasElement = match canvas_ref.get() {
            Some(c) => c,
            None => return,
        };

        let ctx = canvas
            .get_context("2d").unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>().unwrap();

        let (pixels, w, h) = layer.to_rgba();

        let preview_size = 50.0;
        let scale = preview_size / w as f32;

        canvas.set_width(preview_size as u32);
        canvas.set_height(preview_size as u32);

        draw_rgba_over_checkerboard(&ctx, &pixels, w, h, scale);
    });

    view! {
        <canvas
            node_ref=canvas_ref
            style="
                image-rendering:pixelated;
                border-radius:2px;
            "
            style:width = {width}
            style:height = {height}
        />
    }
}