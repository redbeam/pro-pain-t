use leptos::prelude::*;
use pro_pain_t_app::structs::project::Project;

#[component]
pub fn CanvasArea() -> impl IntoView {
    let project = use_context::<RwSignal<Project>>().unwrap().get();

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
                    project.width.get(),
                    project.height.get()
                )
            ></div>
        </section>
    }
}
