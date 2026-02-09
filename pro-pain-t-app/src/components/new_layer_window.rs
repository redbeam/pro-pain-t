use crate::components::color_picker::ColorPicker;
use crate::structs::project::Project;
use crate::structs::{color::Color, layer::Layer};
use leptos::{html::Dialog, logging, prelude::*};

#[component]
pub fn NewLayerWindow(dialog_ref: NodeRef<Dialog>, is_open: RwSignal<bool>) -> impl IntoView {
    let title = RwSignal::new(String::from("New layer"));
    let color = RwSignal::new(Color::default_white());

    let project = use_context::<RwSignal<Project>>().unwrap();

    let create_layer = move || {
        let layer_id = project.get().next_layer_id.get();
        let layer = Layer::new(
            layer_id,
            title.get(),
            project.get().width.get(),
            project.get().height.get(),
            color.get(),
        );
        project.get().add_new_layer(layer);

        logging::log!(
            "new_layer: {}, {}, {}, {}, count: {}",
            layer_id,
            project.get().width.get(),
            project.get().height.get(),
            title.get(),
            project.get().layer_count()
        );
    };

    let reset = move || {
        title.set(String::from("New layer"));
        color.set(Color::new(255, 255, 255, 1.0f32));
    };

    view! {
        <dialog
            node_ref = dialog_ref
            open = move || {
                reset();
                is_open.get()
            }
            class="dialog"
        >
            <div
                class="layer-dialog-layout">
                <h1 style="color:white; text-align:center;">
                    "New layer"
                </h1>
                    "Title:"
                    <input
                        type="text"
                        prop:value = move || title.get()
                        on:input = move |value| { title.set(event_target_value(&value)) }
                        style:text="Title"
                        id="new-layer-title"
                    />
                    Background color:
                    <ColorPicker color=color style:color = "#ffffff" style:margin="2px" style:padding="2px" />
                <div
                style="display:flex; justify-content:space-between;">
                    <button
                        class="dialog-button"
                        style:width="80px"
                        on:click = move |_| {
                            is_open.set(false);
                        }
                        id="cancel-add-layer-window"
                    >
                        "Cancel"
                    </button>
                    <button
                        class="dialog-button-ok"
                        style:width="80px"
                        on:click = move |_| {
                            is_open.set(false);
                            dialog_ref.get().unwrap().close();
                            create_layer();
                        }
                        id="confirm-add-layer-window"
                    >
                        "Ok"
                    </button>
                </div>
            </div>
        </dialog>
    }
}
