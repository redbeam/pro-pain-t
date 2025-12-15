use leptos::prelude::*;

#[component]
pub fn CanvasArea() -> impl IntoView {
    view! {
        <section
            style="
                flex:1;
                display:flex;
                align-items:center;
                justify-content:center;
                background:#3f3f3f;
            "
        >
            <div
                style="
                    width:75%;
                    height:75%;
                    max-width:1100px;
                    max-height:700px;
                    background:#dcdcdc;
                    box-shadow:0 0 0 1px #777, 0 10px 24px rgba(0,0,0,0.6);
                "
            ></div>
        </section>
    }
}
