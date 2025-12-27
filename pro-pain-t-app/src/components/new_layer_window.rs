use leptos::{html::Dialog, logging, prelude::*};
use pro_pain_t_app::structs::layer::Layer;

use crate::components::{color_picker::ColorPicker};

#[component]
pub fn NewLayerWindow(dialog_ref: NodeRef<Dialog>, is_open: RwSignal<bool>, width: u32, height: u32, layers: RwSignal<Vec<pro_pain_t_app::structs::layer::Layer>>, id: RwSignal<usize>) -> impl IntoView {
    let background_color = pro_pain_t_app::structs::color::Color::new(0, 0, 0, 0);
    let (title, set_title) = signal(String::from("New layer"));

    let create_layer = move || {
        let layer_id = id.get();
        let mut layers_vector = layers.get();
        let layer = Layer::new(layer_id, title.get(), width, height, background_color);
        layers_vector.push(layer);
        let count = layers_vector.iter().count();
        layers.set(layers_vector);
        logging::log!("new_layer: {}, {}, {}, {}, count: {}", layer_id, width, height, title.get(), count);
        id.set(layer_id + 1);
    };

    view! {
        <dialog
            node_ref = dialog_ref
            style = move || {format!("align-items:center;
                justify-content:space-between;
                background:#3f3f3f;
                flex-direction:row;
                display:{};", if is_open.get() {"block"} else {"none"})}>
            <h1 style="color:white; text-align:center;">"New layer"</h1>
            <table>
                <tr>
                    <td style:color="white">"Title:"</td>
                    <td >
                        <input
                            type="text"
                            prop:value = move || title.get()
                            on:input = move |value| { set_title.set(event_target_value(&value)) }
                            style:color="black"
                            style:text="Title"
                            id="new-layer-title"/>
                    </td>
                </tr>
                <tr>
                    <td style:color="white">"Background color:"</td>
                    <td><ColorPicker /></td>
                </tr>
            </table>
            <div
            style="display:flex; justify-content:space-between;">
                <button
                    on:click = move |_| {
                        is_open.set(false);
                        dialog_ref.get().unwrap().close();
                    }
                    id="cancel-add-layer-window"
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
                >"Cancel"</button>
                <button
                    on:click = move |_| {
                        is_open.set(false);
                        dialog_ref.get().unwrap().close();
                        create_layer();
                    }
                    id="confirm-add-layer-window"
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
                >"Ok"</button>
            </div>
        </dialog>
    }
}
