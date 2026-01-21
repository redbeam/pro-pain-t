use crate::components::new_layer_window::NewLayerWindow;
use crate::components::layer_preview::LayerPreview;
use leptos::{html::Dialog, logging, prelude::*};
use pro_pain_t_app::structs::layer::Layer;
use pro_pain_t_app::structs::project::Project;

#[component]
pub fn LayerPanel() -> impl IntoView {
    let project = use_context::<RwSignal<Project>>().unwrap().get();

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
                    each=move || project.layers.get().into_iter().rev()
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
                                    <button
                                    disabled = move || {
                                        if let Some(layer_reactive) = project.layers.get().iter().find(|l| l.id == layer.id) {
                                            layer_reactive.is_locked
                                        }
                                        else {
                                            true
                                        }
                                    }
                                    on:click = move |_| {
                                        project.layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                layers[index].is_visible = !layers[index].is_visible;
                                                logging::log!("Layer {} visibility toggle: {}", layers[index].id, layers[index].is_visible);
                                            }
                                        });
                                    }>
                                    "👀"
                                    </button>

                                    <button on:click = move |_| {
                                        project.layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                layers[index].is_locked = !layers[index].is_locked;
                                                logging::log!("Layer {} locked toggle: {}", layers[index].id, layers[index].is_locked);
                                            }
                                        });
                                    }>
                                    "🔒"
                                    </button>

                                    <button
                                    disabled = move || {
                                        if let Some(layer_reactive) = project.layers.get().iter().find(|l| l.id == layer.id) {
                                            layer_reactive.is_locked
                                        }
                                        else {
                                            true
                                        }
                                    }
                                    on:click = move |_| {
                                        project.layers.update(|layers| {
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
                                    <LayerPreview layer=layer.clone() />
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
                                        if let Some(index) = project.layers.get().iter().position(|l| l.id == layer.id) {
                                            index >= project.layers.get().iter().count() - 1
                                        }
                                        else {
                                            true
                                        }
                                    }
                                    on:click = move |_| {
                                        project.layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                if index >= layers.len() - 1 {
                                                    return;
                                                }
                                                layers.swap(index, index + 1);
                                                logging::log!("Layer {} moved down", layer.id);
                                            }
                                        });
                                    }>
                                    "▲"
                                    </button>
                                    <button
                                    disabled = move || {
                                        if let Some(index) = project.layers.get().iter().position(|l| l.id == layer.id) {
                                            index <= 0
                                        }
                                        else {
                                            true
                                        }
                                    }
                                    on:click = move |_| {
                                        project.layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                if index <= 0 {
                                                    return;
                                                }
                                                layers.swap(index, index - 1);
                                                logging::log!("Layer {} moved up", layers[index].id);
                                            }
                                        });
                                    }>
                                    "▼"
                                    </button>                      
                                    <button
                                    on:click = move |_| {
                                        project.layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                let original_layer = layers.get(index).unwrap();
                                                let mut layer_cloned = original_layer.clone();
                                                layer_cloned.title = (layer_cloned.title + " (Copy)").to_string();
                                                let layer_id = project.next_layer_id.get();
                                                layer_cloned.id = layer_id;
                                                project.next_layer_id.set(layer_id + 1);

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

            <NewLayerWindow
                dialog_ref = new_layer_window_ref
                is_open = is_new_layer_window_open
            />
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
