use crate::view_state::ProjectViewState;
use crate::{state::workspace_state::WorkspaceState, structs::project::Project};
use leptos::prelude::*;
use leptos::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, PointerEvent, wasm_bindgen::JsCast};

use crate::render::canvas_renderer::{ViewTransform, composite_layers, draw_project_viewport};
use crate::tools::context::ToolContext;
use crate::tools::select::{commit_selection, SelectionBuffer, SelectionState};
use wasm_bindgen::prelude::*;


#[component]
pub fn CanvasArea() -> impl IntoView {
    let canvas_ref = NodeRef::new();

    let project = use_context::<RwSignal<Project>>().unwrap();
    let view_state = use_context::<ProjectViewState>().expect("ProjectViewState context missing");
    let workspace_state = use_context::<WorkspaceState>().expect("WorkspaceState context missing");

    let workspace_state_for_down = workspace_state.clone();
    let workspace_state_for_move = workspace_state.clone();
    let workspace_state_for_up = workspace_state.clone();
    let workspace_state_for_render = workspace_state.clone();

    let current_tool = workspace_state.current_tool;

    let canvas_size_trigger = RwSignal::new(0u32);

    let on_pointer_down = move |ev: PointerEvent| {
        if ev.button() != 0 {
            return;
        }

        if let Some(target) = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            let _ = target.set_pointer_capture(ev.pointer_id());
        }

        let canvas: HtmlCanvasElement = match canvas_ref.get() {
            Some(c) => c,
            None => return,
        };

        let zoom = view_state.zoom_factor.get();
        let pan_x = view_state.pan_x.get();
        let pan_y = view_state.pan_y.get();
        let selected_layer = workspace_state_for_down.selected_layer_id.get();

        let ctx = ToolContext {
            canvas: &canvas,
            project: &project,
            view_state: &view_state,
            workspace_state: &workspace_state_for_down,
            zoom,
            pan_x,
            pan_y,
            selected_layer,
        };

        current_tool.update(|t| t.on_pointer_down(&ev, &ctx));
        ev.prevent_default();
    };

    let on_pointer_move = move |ev: PointerEvent| {
        let canvas: HtmlCanvasElement = match canvas_ref.get() {
            Some(c) => c,
            None => return,
        };

        let zoom = view_state.zoom_factor.get();
        let pan_x = view_state.pan_x.get();
        let pan_y = view_state.pan_y.get();
        let selected_layer = workspace_state_for_move.selected_layer_id.get();

        let ctx = ToolContext {
            canvas: &canvas,
            project: &project,
            view_state: &view_state,
            workspace_state: &workspace_state_for_move,
            zoom,
            pan_x,
            pan_y,
            selected_layer,
        };

        current_tool.update(|t| t.on_pointer_move(&ev, &ctx));
        ev.prevent_default();
    };

    let on_pointer_up = move |ev: PointerEvent| {
        let canvas: HtmlCanvasElement = match canvas_ref.get() {
            Some(c) => c,
            None => return,
        };

        let zoom = view_state.zoom_factor.get();
        let pan_x = view_state.pan_x.get();
        let pan_y = view_state.pan_y.get();
        let selected_layer = workspace_state_for_up.selected_layer_id.get();

        let ctx = ToolContext {
            canvas: &canvas,
            project: &project,
            view_state: &view_state,
            workspace_state: &workspace_state_for_up,
            zoom,
            pan_x,
            pan_y,
            selected_layer,
        };

        current_tool.update(|t| t.on_pointer_up(&ev, &ctx));
        ev.prevent_default();
    };

    Effect::new(move || {
        if let Some(window) = web_sys::window() {
            let trigger = canvas_size_trigger;
            
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                trigger.update(|v| *v = v.wrapping_add(1));
            }) as Box<dyn FnMut(web_sys::Event)>);
            
            let _ = window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
            
            closure.forget();
        }
    });

    Effect::new(move || {
        if let Some(window) = web_sys::window() {
            let ws = workspace_state.clone();
            let tool = current_tool;
            let project = project;

            let closure = Closure::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
                if ev.key() == "Escape" {
                    if let Some(selection) = ws.selection.get() {
                        commit_selection(&project, &selection);
                    }
                    ws.selection.set(None);
                    tool.update(|t| t.on_pointer_cancel());
                    ev.prevent_default();
                }
            }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

            let _ = window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });

    Effect::new(move |_| {
        let canvas: HtmlCanvasElement = match canvas_ref.get() {
            Some(c) => c,
            None => return,
        };

        if view_state.did_center_view.get_untracked() {
            return;
        }

        let rect = canvas.get_bounding_client_rect();
        let viewport_w_css = rect.width() as f32;
        let viewport_h_css = rect.height() as f32;

        project.with_untracked(|project| {
            let layers = project.layers.get_untracked();
            let proj_w = if layers.is_empty() {
                project.width.get_untracked()
            } else {
                layers[0].canvas.width
            };
            let proj_h = if layers.is_empty() {
                project.height.get_untracked()
            } else {
                layers[0].canvas.height
            };

            let zoom = view_state.zoom_factor.get_untracked();
            view_state.ensure_centered_once(viewport_w_css, viewport_h_css, proj_w, proj_h, zoom);
        });
    });

    Effect::new(move |_| {
        let canvas: HtmlCanvasElement = match canvas_ref.get() {
            Some(c) => c,
            None => return,
        };

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let zoom = view_state.zoom_factor.get();
        let pan_x = view_state.pan_x.get();
        let pan_y = view_state.pan_y.get();
        let _ = canvas_size_trigger.get();

        let window = web_sys::window().expect("window missing");
        let device_pixel_ratio = window.device_pixel_ratio();

        let rect = canvas.get_bounding_client_rect();
        let cw = (rect.width() * device_pixel_ratio).max(1.0).round() as u32;
        let ch = (rect.height() * device_pixel_ratio).max(1.0).round() as u32;
        if canvas.width() != cw {
            canvas.set_width(cw);
        }
        if canvas.height() != ch {
            canvas.set_height(ch);
        }

        project.with(|project| {
            let layers = project.layers.get();
            let proj_w = if layers.is_empty() {
                project.width.get()
            } else {
                layers[0].canvas.width
            };
            let proj_h = if layers.is_empty() {
                project.height.get()
            } else {
                layers[0].canvas.height
            };

            if layers.is_empty() {
                let pixels = vec![0u8; (proj_w * proj_h * 4) as usize];
                draw_project_viewport(
                    &ctx,
                    cw,
                    ch,
                    &pixels,
                    proj_w,
                    proj_h,
                    ViewTransform {
                        zoom,
                        pan_x,
                        pan_y,
                        device_pixel_ratio,
                    },
                );
                return;
            }

            let (pixels, proj_w, proj_h) = composite_layers(&layers);
            draw_project_viewport(
                &ctx,
                cw,
                ch,
                &pixels,
                proj_w,
                proj_h,
                ViewTransform {
                    zoom,
                    pan_x,
                    pan_y,
                    device_pixel_ratio,
                },
            );
        });

        let active_layer = workspace_state_for_render.selected_layer_id.get();
        if let Some(selection) = workspace_state_for_render.selection.get() {
            if Some(selection.layer_id) != active_layer {
                return;
            }
            draw_selection_overlay(
                &ctx,
                &selection,
                ViewTransform {
                    zoom,
                    pan_x,
                    pan_y,
                    device_pixel_ratio,
                },
            );
        }
    });

    view! {
        <canvas
            node_ref=canvas_ref
            on:pointerdown=on_pointer_down
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up
            on:pointercancel=move |_| current_tool.update(|t| t.on_pointer_cancel())
            style=move || {
                let cursor = current_tool.get().cursor();
                let _ = view_state.zoom_factor.get();
                let _ = project.get_untracked();
                format!(
                    "
                    width:100%;
                    height:100%;
                    display:block;
                    image-rendering:pixelated;
                    background:#ccc;
                    cursor:{};
                    touch-action:none;
                    ",
                    cursor,
                )
            }

        />
    }
}

fn draw_selection_overlay(ctx: &CanvasRenderingContext2d, selection: &SelectionState, t: ViewTransform) {
    let scale = (t.zoom as f64) * t.device_pixel_ratio;
    let tx = (t.pan_x as f64) * t.device_pixel_ratio;
    let ty = (t.pan_y as f64) * t.device_pixel_ratio;

    let _ = ctx.set_transform(scale, 0.0, 0.0, scale, tx, ty);
    ctx.set_image_smoothing_enabled(false);

    let line_width = (1.0 / scale.max(0.0001)).max(0.5);
    ctx.set_line_width(line_width);

    let rect = &selection.rect;
    if rect.is_empty() {
        return;
    }
    ctx.set_stroke_style_str("#4a7cff");
    ctx.stroke_rect(rect.x as f64, rect.y as f64, rect.w as f64, rect.h as f64);

    if let Some(buffer) = selection.buffer.as_ref() {
        draw_selection_pixels(ctx, rect, buffer);
    }

    let handle_size = 6.0 / scale.max(0.0001);
    let hs = handle_size / 2.0;
    let x0 = rect.x as f64;
    let y0 = rect.y as f64;
    let x1 = rect.x as f64 + rect.w as f64;
    let y1 = rect.y as f64 + rect.h as f64;
    let xm = (x0 + x1) / 2.0;
    let ym = (y0 + y1) / 2.0;

    let handles = [
        (x0, y0),
        (x1, y0),
        (x1, y1),
        (x0, y1),
        (xm, y0),
        (x1, ym),
        (xm, y1),
        (x0, ym),
    ];

    ctx.set_fill_style_str("#ffffff");
    ctx.set_stroke_style_str("#4a7cff");

    for (hx, hy) in handles {
        ctx.fill_rect(hx - hs, hy - hs, handle_size, handle_size);
        ctx.stroke_rect(hx - hs, hy - hs, handle_size, handle_size);
    }
}

fn draw_selection_pixels(ctx: &CanvasRenderingContext2d, rect: &crate::tools::select::SelectionRect, buffer: &SelectionBuffer) {
    if buffer.width == 0 || buffer.height == 0 || rect.w <= 0 || rect.h <= 0 {
        return;
    }

    let offscreen = create_offscreen_canvas(buffer.width, buffer.height);
    let off_ctx = offscreen
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let rgba = buffer_to_rgba(buffer);
    let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&rgba),
        buffer.width,
        buffer.height,
    )
    .expect("Failed to create ImageData");

    off_ctx.put_image_data(&image_data, 0.0, 0.0).expect("Failed to put ImageData");

    let _ = ctx.draw_image_with_html_canvas_element_and_dw_and_dh(
        &offscreen,
        rect.x as f64,
        rect.y as f64,
        rect.w as f64,
        rect.h as f64,
    );
}

fn buffer_to_rgba(buffer: &SelectionBuffer) -> Vec<u8> {
    let mut out = Vec::with_capacity((buffer.width * buffer.height * 4) as usize);
    for c in &buffer.pixels {
        out.push(c.r);
        out.push(c.g);
        out.push(c.b);
        out.push((c.alpha * 255.0) as u8);
    }
    out
}

fn create_offscreen_canvas(width: u32, height: u32) -> HtmlCanvasElement {
    let document = web_sys::window()
        .and_then(|w| w.document())
        .expect("document missing");
    let canvas = document
        .create_element("canvas")
        .expect("Failed to create canvas element")
        .dyn_into::<HtmlCanvasElement>()
        .expect("Failed to cast element to HtmlCanvasElement");
    canvas.set_width(width);
    canvas.set_height(height);
    canvas
}
