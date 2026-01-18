use crate::components::color_picker::ColorPicker;
use leptos::{html::Dialog, logging, prelude::*};
use pro_pain_t_app::structs::project::Project;
use pro_pain_t_app::structs::{color::Color, layer::Layer};

#[component]
pub fn NewLayerWindow(dialog_ref: NodeRef<Dialog>, is_open: RwSignal<bool>) -> impl IntoView {
    let (title, set_title) = signal(String::from("New layer"));
    let color = RwSignal::new(Color::default_white());

    let project = use_context::<RwSignal<Project>>().unwrap();

    let create_layer = move || {
        let layer_id = project.get().next_layer_id.get();
        let layer = Layer::new(layer_id, title.get(), project.get().width.get(), project.get().height.get(), color.get());
        project.get().add_new_layer(layer);

        logging::log!("new_layer: {}, {}, {}, {}, count: {}", layer_id, project.get().width.get(), project.get().height.get(), title.get(), project.get().layer_count());
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
                "New layer"
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
                            on:input = move |value| { set_title.set(event_target_value(&value)) }
                            style:color="black"
                            style:text="Title"
                            id="new-layer-title"
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
                >
                    "Cancel"
                </button>
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
                >
                    "Ok"
                </button>
            </div>
        </dialog>
    }
}
