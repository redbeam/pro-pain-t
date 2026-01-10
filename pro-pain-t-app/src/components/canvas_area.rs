use leptos::prelude::*;
use pro_pain_t_app::structs::project::Project;

use crate::view_state::ProjectViewState;

#[component]
pub fn CanvasArea() -> impl IntoView {
    let project = use_context::<RwSignal<Project>>().unwrap().get();
    let view_state = use_context::<ProjectViewState>().expect("ProjectViewState context missing");

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
                    "width:{:.2}px; height:{:.2}px; background:#dcdcdc; box-shadow:0 0 0 1px #777, 0 10px 24px rgba(0,0,0,0.6); image-rendering:pixelated;",
                    project.width.get() as f32 * view_state.zoom_factor.get(),
                    project.height.get() as f32 * view_state.zoom_factor.get()
                )
            ></div>
        </section>
    }
}
