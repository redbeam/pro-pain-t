use crate::components::color_picker::ColorPicker;
use crate::structs::project::Project;
use leptos::{html::Dialog, logging, prelude::*};

#[component]
pub fn EditLayerWindow(
    dialog_ref: NodeRef<Dialog>,
    is_open: RwSignal<bool>,
    id: usize,
) -> impl IntoView {
    let project = use_context::<RwSignal<Project>>().unwrap().get();

    let layers = project.layers.get();
    let layer = layers.iter().find(|l| l.id == id).unwrap();

    let title = RwSignal::new(layer.title.clone());
    let color = RwSignal::new(layer.canvas.background_color);

    let edit_layer = move || {
        project.layers.update(|layers| {
            if let Some(index) = layers.iter_mut().position(|l| l.id == id) {
                layers[index].canvas.background_color = color.get();
                layers[index].title = title.get();
                logging::log!("Layer {} edited", id);
            }
        });
    };

    view! {
        <dialog
            class="dialog"
            node_ref = dialog_ref
            style = move || {format!("display:{};", if is_open.get() {"block"} else {"none"})}
        >
            <div
                class="layer-dialog-layout">
                <h1 style="color:white; text-align:center;">
                    "Edit layer"
                </h1>
                        "Title:"
                        <input
                            type="text"
                            prop:value = move || title.get()
                            on:input = move |value| { title.set(event_target_value(&value)) }
                            style:text="Title"
                            id="edit-layer-title"
                        />
                        Background color:
                        <ColorPicker color=color style:color = "#ffffff" style:margin="2px" style:padding="2px"/>
                <div
                style="display:flex; justify-content:space-between;">
                    <button
                        class="dialog-button"
                        style:width="80px"
                        on:click = move |_| {
                            is_open.set(false);
                            dialog_ref.get().unwrap().close();
                        }
                        id="cancel-edit-layer-window"
                    >
                        "Cancel"
                    </button>
                    <button
                        class="dialog-button-ok"
                        style:width="80px"
                        on:click = move |_| {
                            is_open.set(false);
                            dialog_ref.get().unwrap().close();
                            edit_layer();
                        }
                        id="confirm-edit-layer-window"
                    >
                        "Ok"
                    </button>
                </div>
            </div>
        </dialog>
    }
}
