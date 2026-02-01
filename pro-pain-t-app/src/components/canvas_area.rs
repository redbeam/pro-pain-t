use leptos::prelude::*;
use leptos::*;
use crate::{state::workspace_state::WorkspaceState, structs::{color::Color, layer::Layer, project::Project}};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, wasm_bindgen::{JsCast, JsValue}};
use crate::view_state::ProjectViewState;

pub fn draw_checkerboard(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    tile_size: u32,
) {
    let light = "#e0e0e0";
    let dark = "#b0b0b0";

    for y in (0..height).step_by(tile_size as usize) {
        for x in (0..width).step_by(tile_size as usize) {
            let is_dark = (x / tile_size + y / tile_size).is_multiple_of(2);
            let color = if is_dark { dark } else { light };

            #[allow(deprecated)]
            ctx.set_fill_style(&JsValue::from_str(color));

            ctx.fill_rect(
                x as f64,
                y as f64,
                tile_size as f64,
                tile_size as f64,
            );
        }
    }
}

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
) {
    ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
    ctx.clear_rect(0.0, 0.0, f64::INFINITY, f64::INFINITY);

    draw_checkerboard(ctx, width, height, 8);

    for y in 0..height {
        for x in 0..width {
            let i = ((y * width + x) * 4) as usize;
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

#[component]
pub fn CanvasArea(
) -> impl IntoView {
    let canvas_ref = NodeRef::new();

    let project = use_context::<RwSignal<Project>>().unwrap();
    let view_state = use_context::<ProjectViewState>().expect("ProjectViewState context missing");
    let workspace_state = use_context::<WorkspaceState>().expect("WorkspaceState context missing");

    let current_tool = workspace_state.current_tool;

    Effect::new(move |_| {
        let canvas: HtmlCanvasElement = match canvas_ref.get() {
            Some(c) => c,
            None => return,
        };

        let ctx = canvas
            .get_context("2d").unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>().unwrap();

        let layers = project.get().layers.get();
        if layers.is_empty() {
            let width = project.get().width.get();
            let height = project.get().height.get();
            canvas.set_width(width);
            canvas.set_height(height);

            ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
            ctx.clear_rect(0.0, 0.0, f64::INFINITY, f64::INFINITY);
            draw_checkerboard(&ctx, width, height, 8);
            return;
        }

        let (pixels, width, height) = composite_layers(&layers);

        canvas.set_width(width);
        canvas.set_height(height);

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
                    (project.get().width.get() as f32 * zoom),
                    (project.get().height.get() as f32 * zoom),
                )
            }

            on:mousedown = move |_| { current_tool.update(|t| t.on_mouse_down()) }
            on:mousemove = move |e| {
                 let canvas = match canvas_ref.get() {
                    Some(c) => c,
                    None => return,
                };

                let zoom = view_state.zoom_factor.get();
                let Some(layer_id) = workspace_state.selected_layer_id.get() else {
                    return;
                };
                current_tool.update(|t| t.on_mouse_move(&e, &canvas, zoom, layer_id, &project)) 
            }
            on:mouseup = move |_| { current_tool.update(|t| t.on_mouse_up()) }
            on:mouseleave = move |_| { current_tool.update(|t| t.on_mouse_up()) }     
            
        />
    }
}