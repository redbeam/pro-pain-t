use leptos::prelude::*;
use pro_pain_t_app::structs::project::Project;

#[component]
pub fn StatusBar(is_open: RwSignal<bool>) -> impl IntoView {
    let project = use_context::<RwSignal<Project>>().unwrap().get();

    view! {
        <footer
            style="
                height:24px;
                background:#181818;
                color:#c0c0c0;
                font-size:0.75rem;
                font-family:system-ui, sans-serif;
                display:flex;
                align-items:center;
                justify-content:space-between;
                padding:0 0.75rem;
                box-sizing:border-box;
            "
        >
            <div style="display:flex; gap:1.5rem; align-items:center;">
                <button
                    on:click=move |_| is_open.set(true)
                    style="
                        border:none;
                        background:transparent;
                        color:#c0c0c0;
                        padding:0;
                        cursor:pointer;
                        font-size:0.75rem;
                    "
                    title="Change canvas size"
                >
                    {move || format!("{}×{} px", project.width.get(), project.height.get())}
                </button>
                <span>"x = 0, y = 0"</span>
            </div>
            <div style="display:flex; align-items:center; gap:0.35rem;">
                <button style="border:none; background:transparent; color:#c0c0c0;">"-"</button>
                <span>"100%"</span>
                <button style="border:none; background:transparent; color:#c0c0c0;">"+"</button>
            </div>
        </footer>
    }
}
