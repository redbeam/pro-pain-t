use crate::view_state::ProjectViewState;
use crate::{state::workspace_state::WorkspaceState, structs::project::Project};
use leptos::prelude::*;
use leptos::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, PointerEvent, wasm_bindgen::JsCast};

use crate::render::canvas_renderer::{ViewTransform, composite_layers, draw_project_viewport};
use crate::tools::context::ToolContext;

#[component]
pub fn CanvasArea() -> impl IntoView {
    let canvas_ref = NodeRef::new();

    let project = use_context::<RwSignal<Project>>().unwrap();
    let view_state = use_context::<ProjectViewState>().expect("ProjectViewState context missing");
    let workspace_state = use_context::<WorkspaceState>().expect("WorkspaceState context missing");

    let current_tool = workspace_state.current_tool;

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
        let selected_layer = workspace_state.selected_layer_id.get();

        let rect = canvas.get_bounding_client_rect();

        let ctx = ToolContext {
            canvas: &canvas,
            project: &project,
            view_state: &view_state,
            viewport_w: rect.width() as f32,
            viewport_h: rect.height() as f32,
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
        let selected_layer = workspace_state.selected_layer_id.get();

        let rect = canvas.get_bounding_client_rect();

        let ctx = ToolContext {
            canvas: &canvas,
            project: &project,
            view_state: &view_state,
            viewport_w: rect.width() as f32,
            viewport_h: rect.height() as f32,
            zoom,
            pan_x,
            pan_y,
            selected_layer,
        };

        current_tool.update(|t| t.on_pointer_move(&ev, &ctx));
        ev.prevent_default();
    };

    let on_pointer_up = move |ev: PointerEvent| {
        current_tool.update(|t| t.on_pointer_up(&ev));
        ev.prevent_default();
    };

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
        let pan_x_tracked = view_state.pan_x.get();
        let pan_y_tracked = view_state.pan_y.get();

        let window = web_sys::window().expect("window missing");
        let dpr = window.device_pixel_ratio();

        let rect = canvas.get_bounding_client_rect();
        let viewport_w_css = rect.width() as f32;
        let viewport_h_css = rect.height() as f32;
        let cw = (rect.width() * dpr).max(1.0).round() as u32;
        let ch = (rect.height() * dpr).max(1.0).round() as u32;
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

            let (pan_x, pan_y) = match view_state.ensure_centered_once(
                viewport_w_css,
                viewport_h_css,
                proj_w,
                proj_h,
                zoom,
            ) {
                Some((x, y)) => (x, y),
                None => (pan_x_tracked, pan_y_tracked),
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
                        dpr,
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
                    dpr,
                },
            );
        });
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
