use crate::components::color_picker::ColorPicker;
use leptos::{html::Dialog, logging, prelude::*};
use pro_pain_t_app::structs::project::Project;

#[component]
pub fn EditLayerWindow(dialog_ref: NodeRef<Dialog>, is_open: RwSignal<bool>, id: usize) -> impl IntoView {
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
            node_ref = dialog_ref
            style = move || {format!("align-items:center;
                justify-content:space-between;
                background:#3f3f3f;
                flex-direction:row;
                display:{};", if is_open.get() {"block"} else {"none"})}
        >
            <h1 style="color:white; text-align:center;">
                "Edit layer"
            </h1>
            <table>
                <tr>
                    <td style:color="white">
                        "Title:"
                    </td>
                    <td>
                        <input
                            type="text"
                            prop:value = move || title.get()
                            on:input = move |value| { title.set(event_target_value(&value)) }
                            style:color="black"
                            style:text="Title"
                            id="edit-layer-title"
                        />
                    </td>
                </tr>
                <tr>
                    <td style:color="white" style="vertical-align:top; padding-top:0.5rem;">"Background color:"</td>
                    <td><ColorPicker color=color /></td>
                </tr>
            </table>
            <div
            style="display:flex; justify-content:space-between;">
                <button
                    on:click = move |_| {
                        is_open.set(false);
                        dialog_ref.get().unwrap().close();
                    }
                    id="cancel-edit-layer-window"
                    style="
                        margin-top:0.25rem;
                        padding:0.25rem 0.5rem;
                        border-radius:2px;
                        border:none;
                        background:#3a3a3a;
                        color:#f5f5f5;
                        font-size:0.8rem;
                        text-align:center;
                        width:70px;
                    "
                >
                    "Cancel"
                </button>
                <button
                    on:click = move |_| {
                        is_open.set(false);
                        dialog_ref.get().unwrap().close();
                        edit_layer();
                    }
                    id="confirm-edit-layer-window"
                    style="
                        margin-top:0.25rem;
                        padding:0.25rem 0.5rem;
                        border-radius:2px;
                        border:none;
                        background:#3a3a3a;
                        color:#f5f5f5;
                        font-size:0.8rem;
                        text-align:center;
                        width:70px;
                    "
                >
                    "Ok"
                </button>
            </div>
        </dialog>
    }
}
