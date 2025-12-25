use leptos::{html::Dialog, prelude::*};

use crate::components::{color_picker::ColorPicker};

#[component]
pub fn NewLayerWindow(dialog_ref: NodeRef<Dialog>, is_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <dialog
            node_ref = dialog_ref
            style = move || {format!("align-items:center;
                justify-content:center;
                background:#3f3f3f;
                width:400px;
                flex-direction:column;
                display:{};", if is_open.get() {"block"} else {"none"})}>
            <h1 style="color:white; text-align:center;">"New layer"</h1>
            <table>
                <tr>
                    <td style:color="white">"Title:"</td>
                    <td>
                        <input type="text" color="black" text="Title" id="new-layer-title"/>
                    </td>
                </tr>
                <tr>
                    <td style:color="white">"Background color:"</td>
                    <td><ColorPicker /></td>
                </tr>
            </table>
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
                    text-align:left;
                "
            >"Cancel"</button>
            <button
                on:click = move |_| {
                    is_open.set(false);
                    dialog_ref.get().unwrap().close();
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
                    text-align:left;
                "
            >"Ok"</button>
        </dialog>
    }
}
