use crate::components::edit_layer_window::EditLayerWindow;
use crate::components::new_layer_window::NewLayerWindow;
use crate::components::layer_preview::LayerPreview;
use leptos::{html::Dialog, logging, prelude::*};
use pro_pain_t_app::state::workspace_state::WorkspaceState;
use crate::structs::layer::Layer;
use crate::structs::project::Project;

#[component]
pub fn LayerPanel() -> impl IntoView {
    let project = use_context::<RwSignal<Project>>().unwrap();
    let id_to_edit = RwSignal::new(None);

    let workspace_state = use_context::<WorkspaceState>().expect("WorkspaceState context missing");

    let new_layer_window_ref: NodeRef<Dialog> = NodeRef::new();
    let is_new_layer_window_open = RwSignal::new(false);

    let edit_layer_window_ref: NodeRef<Dialog> = NodeRef::new();
    let is_edit_layer_window_open = RwSignal::new(false);

    let open_edit_layer_window = move |id: usize| {
        id_to_edit.set(Some(id));
        is_edit_layer_window_open.set(true);
    };

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
                    each=move || project.get().layers.get().into_iter().rev()
                    key=|layer| layer.id
                    children=move |layer: Layer| {
                        view! {
                            <div
                                style= format!("
                                    display:flex;
                                    align-items:center;
                                    gap:0.35rem;
                                    padding:0.25rem 0.3rem;
                                    border-radius:2px;
                                ")
                                style:background-color = move || {
                                    if workspace_state.selected_layer_id.with(|_| workspace_state.selected_layer_id.get()) == Some(layer.id) {
                                        "#151515"
                                    } else {
                                        "#2c2c2c"
                                    }
                                }
                            >
                                <div
                                    style="
                                        display:flex;
                                        flex-direction:column;
                                        gap:0.15rem;
                                        font-size:0.7rem;
                                    "
                                >
                                    <button
                                    style:background = move || {
                                        if let Some(layer_reactive) = project.get().layers.get().iter().find(|l| l.id == layer.id) {
                                            if layer_reactive.is_locked {
                                                "#757575"
                                            }
                                            else if layer_reactive.is_visible {
                                                "#B0B0B0"
                                            }
                                            else {
                                                "#404040"
                                            }
                                        }
                                        else {
                                            "#B0B0B0"
                                        }
                                    }
                                    disabled = move || {
                                        if let Some(layer_reactive) = project.get().layers.get().iter().find(|l| l.id == layer.id) {
                                            layer_reactive.is_locked
                                        }
                                        else {
                                            true
                                        }
                                    }
                                    on:click = move |_| {
                                        project.get().layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                layers[index].is_visible = !layers[index].is_visible;
                                                logging::log!("Layer {} visibility toggle: {}", layers[index].id, layers[index].is_visible);
                                            }
                                        });
                                    }>
                                    "👀"
                                    </button>

                                    <button
                                    style:background = move || {
                                        if let Some(layer_reactive) = project.get().layers.get().iter().find(|l| l.id == layer.id) {
                                            if layer_reactive.is_locked {
                                                "#404040"
                                            }
                                            else {
                                                "#B0B0B0"
                                            }
                                        }
                                        else {
                                            "#B0B0B0"
                                        }
                                    }
                                    on:click = move |_| {
                                        project.get().layers.update(|layers| {
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
                                        if let Some(layer_reactive) = project.get().layers.get().iter().find(|l| l.id == layer.id) {
                                            layer_reactive.is_locked
                                        }
                                        else {
                                            true
                                        }
                                    }
                                    on:click = move |_| {
                                        let current_project = project.get();
                                        let selected = workspace_state.selected_layer_id.get();
                                        let layers_original = current_project.layers.get();
                                        let layer_index = layers_original.iter().position(|l| l.id == layer.id).unwrap();
                                        let mut new_selected = selected;

                                        if selected.is_some() && selected.unwrap() == layer.id {
                                            if layers_original.len() <= 1 {
                                                new_selected = None;
                                                logging::log!("Selected layer after delete: None")
                                            } else if layer_index == layers_original.len() - 1 {
                                                new_selected = Some(layers_original[layer_index - 1].id);
                                                logging::log!("Selected layer after delete: {}", layers_original[layer_index - 1].id);
                                            } else {
                                                new_selected = Some(layers_original[layer_index + 1].id);
                                                logging::log!("Selected layer after delete: {}", layers_original[layer_index + 1].id);
                                            }
                                        }

                                        project.get().layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                layers.remove(index);
                                                logging::log!("Layer {} delete pressed", layer.id);
                                            }
                                        });
                                        
                                        workspace_state.selected_layer_id.set(new_selected);
                                    }>
                                    "🗑️"
                                    </button>

                                    <button
                                    disabled = move || {
                                        if let Some(layer_reactive) = project.get().layers.get().iter().find(|l| l.id == layer.id) {
                                            layer_reactive.is_locked
                                        }
                                        else {
                                            true
                                        }
                                    }
                                    on:click = move |_| {
                                        logging::log!("Edit button clicked!");
                                        open_edit_layer_window(layer.id);
                                    }>
                                    "✏️"
                                    </button>
                                </div>
                                <div
                                    style="
                                        flex:1;
                                        display:flex;
                                        flex-direction:column;
                                        gap:0.15rem;
                                        align-items:center
                                    "
                                    on:click = move |_| {
                                        logging::log!("Layer {} selected: ", layer.id);
                                        workspace_state.selected_layer_id.set(Some(layer.id));
                                    }
                                >
                                    <LayerPreview layer=layer.clone() />
                                    <span style="font-size:0.8rem;">{move || {
                                        let binding = project.get().layers.get();
                                        let l = binding.iter().find(|l| l.id == layer.id).unwrap();
                                        l.title.clone()
                                    }}</span>
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
                                        if let Some(index) = project.get().layers.get().iter().position(|l| l.id == layer.id) {
                                            index >= project.get().layers.get().iter().count() - 1
                                        }
                                        else {
                                            true
                                        }
                                    }
                                    on:click = move |_| {
                                        project.get().layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                if index >= layers.len() - 1 {
                                                    return;
                                                }
                                                layers.swap(index, index + 1);
                                                logging::log!("Layer {} moved up", layer.id);
                                            }
                                        });
                                    }>
                                    "▲"
                                    </button>
                                    
                                    <button
                                    on:click = move |_| {
                                        project.get().layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                let original_layer = layers.get(index).unwrap();
                                                let mut layer_cloned = original_layer.clone();
                                                layer_cloned.title = (layer_cloned.title + " (Copy)").to_string();
                                                let layer_id = project.get().next_layer_id.get();
                                                layer_cloned.id = layer_id;
                                                project.get().next_layer_id.set(layer_id + 1);

                                                layers.insert(index + 1, layer_cloned);
                                                logging::log!("Layer {} cloned", layer.id);
                                            }
                                        });
                                    }>
                                    "📄"
                                    </button>

                                    <button
                                    disabled = move || {
                                        if let Some(layer_reactive) = project.get().layers.get().iter().find(|l| l.id == layer.id) {
                                            layer_reactive.is_locked
                                        }
                                        else {
                                            true
                                        }
                                    }
                                    on:click = move |_| {
                                        project.get().layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                layers[index].canvas.clear();
                                                logging::log!("Layer {} cleared", layers[index].id);
                                            }
                                        });
                                    }>
                                    "🧽"
                                    </button>

                                    <button
                                    disabled = move || {
                                        if let Some(index) = project.get().layers.get().iter().position(|l| l.id == layer.id) {
                                            index <= 0
                                        }
                                        else {
                                            true
                                        }
                                    }
                                    on:click = move |_| {
                                        project.get().layers.update(|layers| {
                                            if let Some(index) = layers.iter_mut().position(|l| l.id == layer.id) {
                                                if index <= 0 {
                                                    return;
                                                }
                                                layers.swap(index, index - 1);
                                                logging::log!("Layer {} moved down", layer.id);
                                            }
                                        });
                                    }>
                                    "▼"
                                    </button>                      
                                </div>
                            </div>
                        }
                    }
                />
                </div>

            <Show when = move || id_to_edit.get().is_some() fallback = || ()>
                <EditLayerWindow
                    dialog_ref = edit_layer_window_ref
                    is_open = is_edit_layer_window_open
                    id = id_to_edit.get().unwrap()
                />
            </Show>

            <NewLayerWindow
                dialog_ref = new_layer_window_ref
                is_open = is_new_layer_window_open
            />
            <button
                on:click = move |_| {
                    logging::log!("Add layer button clicked!");
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
