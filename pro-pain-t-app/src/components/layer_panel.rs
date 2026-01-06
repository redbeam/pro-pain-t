use leptos::{html::Dialog, logging, prelude::*};
use pro_pain_t_app::structs::layer::Layer;

use crate::components::new_layer_window::NewLayerWindow;


#[component]
pub fn LayerPanel(canvas_width: RwSignal<u32>, canvas_height: RwSignal<u32>, layers: RwSignal<Vec<Layer>>, layer_id: RwSignal<usize>) -> impl IntoView {
    let new_layer_window_ref: NodeRef<Dialog> = NodeRef::new();
    let is_new_layer_window_open = RwSignal::new(false);

    view! {
        <aside
            style="
                width:210px;
                min-width:200px;
                max-width:240px;
                background:#2a2a2a;
                color:#f5f5f5;
                padding:0.5rem 0.5rem 0.75rem 0.5rem;
                box-sizing:border-box;
                font-family:system-ui, sans-serif;
                display:flex;
                flex-direction:column;
                gap:0.5rem;
            "
        >
            <h2 style="font-size:0.85rem; margin:0 0 0.25rem 0; text-transform:uppercase; letter-spacing:0.06em;">
                "Layers"
            </h2>

            <div style="
                    flex:1;
                    border-radius:2px;
                    background:#1f1f1f;
                    padding:0.35rem;
                    box-sizing:border-box;
                    display:flex;
                    flex-direction:column;
                    gap:0.4rem;
                    font-size:0.8rem;
                ">
                <For
                    each=move || layers.get()
                    key=|layer| layer.id
                    children=move |layer: Layer| {
                        view! {
                            <div
                                style="
                                    display:flex;
                                    align-items:center;
                                    gap:0.35rem;
                                    background:#2c2c2c;
                                    padding:0.25rem 0.3rem;
                                    border-radius:2px;
                                "
                            >
                                <div
                                    style="
                                        display:flex;
                                        flex-direction:column;
                                        gap:0.75rem;
                                        font-size:0.7rem;
                                        align-items:center;
                                        color:#d0d0d0;
                                    "
                                >
                                    <button on:click = move |_| {
                                        layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                layers[index].is_visible = !layers[index].is_visible;
                                                logging::log!("Layer {} visibility toggle: {}", layers[index].id, layers[index].is_visible);
                                            }
                                        });
                                    }>
                                    "👀"
                                    </button>

                                    <button on:click = move |_| {
                                        layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                layers[index].is_locked = !layers[index].is_locked;
                                                logging::log!("Layer {} locked toggle: {}", layers[index].id, layers[index].is_locked);
                                            }
                                        });
                                    }>
                                    "🔒"
                                    </button>

                                    <button on:click = move |_| {
                                        layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                layers.remove(index);
                                                logging::log!("Layer {} delete pressed", layer.id);
                                            }
                                        });
                                    }>
                                    "🗑️"
                                    </button>
                                </div>
                                <div
                                    style="
                                        flex:1;
                                        display:flex;
                                        flex-direction:column;
                                        gap:0.15rem;
                                    "
                                >
                                    <div
                                        style="
                                            height:50px;
                                            background:#d8d8d8;
                                            border-radius:2px;
                                        "
                                    ></div>
                                    <span style="font-size:0.8rem;">{layer.title.clone()}</span>
                                </div>
                                <div
                                    style="
                                        display:flex;
                                        flex-direction:column;
                                        gap:0.15rem;
                                        font-size:0.7rem;
                                        color:#d0d0d0;
                                    "
                                >
                                    <button
                                    disabled = move || {
                                        if let Some(index) = layers.get().iter().position(|l| l.id == layer.id) {
                                            index <= 0
                                        }
                                        else {
                                            true
                                        }
                                    }
                                    on:click = move |_| {
                                        layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                if index <= 0 {
                                                    return;
                                                }
                                                layers.swap(index, index - 1);
                                                logging::log!("Layer {} moved up", layers[index].id);
                                            }
                                        });
                                    }>
                                    "▲"
                                    </button>

                                    <button
                                    disabled = move || {
                                        if let Some(index) = layers.get().iter().position(|l| l.id == layer.id) {
                                            index >= layers.get().iter().count() - 1
                                        }
                                        else {
                                            true
                                        }
                                    }
                                    on:click = move |_| {
                                        layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                if index >= layers.len() - 1 {
                                                    return;
                                                }
                                                layers.swap(index, index + 1);
                                                logging::log!("Layer {} moved down", layer.id);
                                            }
                                        });
                                    }>
                                    "▼"
                                    </button>
                                    <button on:click = move |_| {
                                        let mut layer_cloned = layer.clone();
                                        layer_cloned.title = (layer_cloned.title + " (Copy)").to_string();
                                        layer_cloned.id = layer_id.get();
                                        layer_id.set(layer_id.get() + 1);
                                        
                                        layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                layers.insert(index + 1, layer_cloned);
                                                logging::log!("Layer {} cloned", layer.id);
                                            }
                                        });
                                    }>
                                    "📄"
                                    </button>
                                </div>
                            </div>
                        }
                    }
                />
                </div>


            <NewLayerWindow dialog_ref = new_layer_window_ref is_open = is_new_layer_window_open width = canvas_width height = canvas_height layers = layers id = layer_id/>
            <button
                on:click = move |_| {
                    logging::log!("Button clicked!");
                    new_layer_window_ref.get().unwrap().open();
                    is_new_layer_window_open.set(true);
                }
                style="
                    margin-top:0.25rem;
                    padding:0.25rem 0.5rem;
                    border-radius:2px;
                    border:none;
                    background:#3a3a3a;
                    color:#f5f5f5;
                    font-size:0.8rem;
                    text-align:center;
                "
            >"Add layer"</button>
        </aside>
    }
}
