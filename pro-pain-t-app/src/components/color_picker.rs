use leptos::prelude::*;

#[component]
pub fn ColorPicker() -> impl IntoView {
    view! {
        <div
            style="
                margin-top:0.75rem;
                display:flex;
                flex-direction:column;
                align-items:center;
                gap:0.5rem;
            "
        >
            <div
                style="
                    width:90px;
                    height:90px;
                    border-radius:50%;
                    background:conic-gradient(from 0deg, red, yellow, lime, cyan, blue, magenta, red);
                    box-shadow:0 0 0 3px #111;
                "
            ></div>
            <div style="width:100%; display:flex; flex-direction:column; gap:0.25rem;">
                <span style="font-size:0.7rem; opacity:0.7;">"Gamma"</span>
                <input
                    style="width:100%;"
                    type="range"
                    min="0"
                    max="100"
                    value="50"
                />
            </div>
        </div>
    }
}
