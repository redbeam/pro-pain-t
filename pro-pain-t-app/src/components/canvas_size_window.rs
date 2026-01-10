use leptos::{html, prelude::*};
use pro_pain_t_app::structs::project::{Project};

#[component]
pub fn CanvasSizeWindow(is_open: RwSignal<bool>) -> impl IntoView
{
    let width_input_ref: NodeRef<html::Input> = NodeRef::new();
    let ok_button_ref: NodeRef<html::Button> = NodeRef::new();

    let project = use_context::<RwSignal<Project>>().unwrap().get();

    let (local_width, set_local_width) = signal(project.width.get());
    let (local_height, set_local_height) = signal(project.height.get());

    {
        let width_input_ref = width_input_ref.clone();
        Effect::new(move |_| {
            if is_open.get() {
                set_local_width.set(project.width.get());
                set_local_height.set(project.height.get());

                if let Some(input) = width_input_ref.get() {
                    let _ = input.focus();
                }
            }
        });
    }

    let on_resize_canvas = move |new_w: u32, new_h: u32| {
        project.layers.update(|layers_vec| {
            for layer in layers_vec.iter_mut() {
                layer.resize_canvas(new_w, new_h);
            }
        });
        project.width.set(new_w);
        project.height.set(new_h);
    };

    let on_width_input = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        let parsed = value
            .parse::<u32>()
            .ok()
            .filter(|v| *v > 0)
            .unwrap_or(1);
        set_local_width.set(parsed);
    };

    let on_height_input = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        let parsed = value
            .parse::<u32>()
            .ok()
            .filter(|v| *v > 0)
            .unwrap_or(1);
        set_local_height.set(parsed);
    };

    let on_key_down = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Escape" {
            is_open.set(false);
            ev.prevent_default();
            ev.stop_propagation();
        }
    };

    view! {
        <style>
            {"
            .canvas-dialog-button:focus-visible {
                outline: 2px solid #ffffff;
                outline-offset: 2px;
            }
            "}
        </style>
        <div
            style=move || format!(
                "position:fixed; inset:0; background:rgba(0,0,0,0.4); display:{}; align-items:center; justify-content:center; z-index:1000;",
                if is_open.get() { "flex" } else { "none" }
            )
        >
            <div tabindex="0" on:focus=move |_| {
                if let Some(btn) = ok_button_ref.get() {
                    let _ = btn.focus();
                }
            }></div>
            <div
                style="
                    background:#2b2b2b;
                    padding:1rem 1.25rem;
                    border-radius:4px;
                    color:#f5f5f5;
                    min-width:260px;
                    font-family:system-ui, sans-serif;
                    box-shadow:0 12px 30px rgba(0,0,0,0.7);
                "
                tabindex="-1"
                on:keydown=on_key_down
            >
                <h2 style="margin:0 0 0.75rem 0; font-size:0.95rem;">"Canvas Size"</h2>
                <table style="width:100%; font-size:0.8rem;">
                            <tr>
                                <td style="padding:0.15rem 0.5rem 0.15rem 0;">"Width (px)"</td>
                                <td style="padding:0.15rem 0;">
                                    <input
                                        node_ref=width_input_ref
                                        type="number"
                                        min="1"
                                        prop:value=move || local_width.get().to_string()
                                        on:input=on_width_input
                                        style="width:100%; box-sizing:border-box;"
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td style="padding:0.15rem 0.5rem 0.15rem 0;">"Height (px)"</td>
                                <td style="padding:0.15rem 0;">
                                    <input
                                        type="number"
                                        min="1"
                                        prop:value=move || local_height.get().to_string()
                                        on:input=on_height_input
                                        style="width:100%; box-sizing:border-box;"
                                    />
                                </td>
                            </tr>
                </table>
                <div
                            style="
                                margin-top:0.75rem;
                                display:flex;
                                justify-content:flex-end;
                                gap:0.5rem;
                            "
                        >
                    <button
                        class="canvas-dialog-button"
                        on:click=move |_| {
                            is_open.set(false);
                        }
                                style="
                                    padding:0.25rem 0.6rem;
                                    border-radius:2px;
                                    border:none;
                                    background:#3a3a3a;
                                    color:#f5f5f5;
                                    font-size:0.8rem;
                            "
                        >"Cancel"</button>
                    <button
                        class="canvas-dialog-button"
                        node_ref=ok_button_ref
                        on:click=move |_| {
                            let w = local_width.get();
                            let h = local_height.get();
                            on_resize_canvas(w, h);
                            is_open.set(false);
                        }
                                style="
                                    padding:0.25rem 0.75rem;
                                    border-radius:2px;
                                    border:none;
                                    background:#4a7cff;
                                    color:#f5f5f5;
                                    font-size:0.8rem;
                            "
                        >"OK"</button>
                </div>
            </div>
            <div tabindex="0" on:focus=move |_| {
                if let Some(input) = width_input_ref.get() {
                    let _ = input.focus();
                }
            }></div>
        </div>
    }
}
