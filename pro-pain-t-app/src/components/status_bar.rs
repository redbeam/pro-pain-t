use leptos::prelude::*;
use pro_pain_t_app::structs::project::Project;

use crate::view_state::ProjectViewState;

#[component]
pub fn StatusBar(is_open: RwSignal<bool>) -> impl IntoView {
    let project = use_context::<RwSignal<Project>>().unwrap().get();
    let view_state = use_context::<ProjectViewState>().expect("ProjectViewState context missing");

    // Precompute these to avoid `<=`/`>=` token confusion in the `view!` macro attribute parser.
    let zoom_out_disabled =
        Memo::new(move |_| view_state.zoom.get() <= ProjectViewState::MIN_ZOOM + 0.000_01);
    let zoom_in_disabled =
        Memo::new(move |_| view_state.zoom.get() >= ProjectViewState::MAX_ZOOM - 0.000_01);

    let zoom_out_style = {
        let zoom_out_disabled = zoom_out_disabled.clone();
        move || {
            let disabled = zoom_out_disabled.get();
            format!(
                "border:none; background:transparent; color:#c0c0c0; cursor:pointer; padding:0 0.25rem; opacity:{};",
                if disabled { "0.35" } else { "1" }
            )
        }
    };
    let zoom_in_style = {
        let zoom_in_disabled = zoom_in_disabled.clone();
        move || {
            let disabled = zoom_in_disabled.get();
            format!(
                "border:none; background:transparent; color:#c0c0c0; cursor:pointer; padding:0 0.25rem; opacity:{};",
                if disabled { "0.35" } else { "1" }
            )
        }
    };

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
                <button
                    on:click=move |_| view_state.zoom_out()
                    title="Zoom out"
                    style=zoom_out_style
                    prop:disabled=move || zoom_out_disabled.get()
                >
                    "-"
                </button>
                <button
                    on:click=move |_| view_state.reset_zoom()
                    title="Reset zoom (100%)"
                    style="border:none; background:transparent; color:#c0c0c0; cursor:pointer; padding:0 0.25rem;"
                >
                    {move || format!("{}%", view_state.zoom_percent())}
                </button>
                <button
                    on:click=move |_| view_state.zoom_in()
                    title="Zoom in"
                    style=zoom_in_style
                    prop:disabled=move || zoom_in_disabled.get()
                >
                    "+"
                </button>
            </div>
        </footer>
    }
}
