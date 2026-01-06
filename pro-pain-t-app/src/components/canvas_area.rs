use leptos::prelude::*;

#[component]
pub fn CanvasArea(canvas_width: RwSignal<u32>, canvas_height: RwSignal<u32>) -> impl IntoView {
    view! {
        <section
            style="
                flex:1;
                display:flex;
                align-items:center;
                justify-content:center;
                min-width:0;
                min-height:0;
                overflow:auto;
                background:#3f3f3f;
            "
        >
            <div
                style=move || format!(
                    "width:{}px; height:{}px; background:#dcdcdc; box-shadow:0 0 0 1px #777, 0 10px 24px rgba(0,0,0,0.6);",
                    canvas_width.get(),
                    canvas_height.get()
                )
            ></div>
        </section>
    }
}
