use crate::components::color_picker::ColorPicker;
use leptos::prelude::{NodeRef, RwSignal};
use leptos::{IntoView, component, view};
use leptos::{html::Dialog, prelude::*};
use pro_pain_t_app::structs::color::Color;
use pro_pain_t_app::structs::project::Project;

#[component]
pub fn NewProjectWindow(dialog_ref: NodeRef<Dialog>, is_open: RwSignal<bool>) -> impl IntoView {
    let (title, set_title) = signal(String::from("New project"));
    let (dim_width, set_dim_width) = signal(String::from("Width"));
    let (dim_height, set_dim_height) = signal(String::from("Height"));
    let color = RwSignal::new(Color::default_white());

    let project = use_context::<RwSignal<Project>>().unwrap();

    let create_project = move || {
        let width = u32::from_str_radix(dim_width.get().as_str(), 10).unwrap();
        let height = u32::from_str_radix(dim_height.get().as_str(), 10).unwrap();
        project.update(|project| {
            project.replace_project_with_blank(title.get(), width, height, color.get());
        });
    };

    view! {
        <dialog
            node_ref = dialog_ref
            style = move || {
                format!("
                    align-items:center;
                    justify-content:space-between;
                    background:#3f3f3f;
                    flex-direction:row;
                    display:{};",
                if is_open.get() {"block"} else {"none"})
            }
        >
            <h1 style="color:white; text-align:center;">
                "Create new project"
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
                    <td style:color="white">
                        "Width:"
                    </td>
                    <td>
                        <input
                            type="number"
                            prop:value="0"
                            on:input = move |value| { set_dim_width.set(event_target_value(&value)) }
                            style:color="black"
                            style:text="X:"
                            id="new-project-width"
                        />
                    </td>
                </tr>
                <tr>
                    <td style:color="white">
                        "Height:"
                    </td>
                    <td>
                        <input
                            type="number"
                            prop:value="0"
                            on:input = move |value| { set_dim_height.set(event_target_value(&value)) }
                            style:color="black"
                            style:text="Y:"
                            id="new-project-height"
                        />
                    </td>
                </tr>
                <tr>
                    <td style:color="white">"Background color:"</td>
                    <td>
                        <ColorPicker
                            color = color
                        />
                    </td>
                </tr>
            </table>
            <div style="display:flex; justify-content:space-between;">
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
                        create_project();
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
