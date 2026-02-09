use crate::structs::project::Project;
use crate::view_state::ProjectViewState;
use leptos::prelude::*;

#[component]
pub fn StatusBar(is_open: RwSignal<bool>) -> impl IntoView {
    let project = use_context::<RwSignal<Project>>().expect("Project context missing");
    let view_state = use_context::<ProjectViewState>().expect("ProjectViewState context missing");

    let zoom_out_disabled = Memo::new(move |_| {
        view_state.zoom_factor.get()
            <= ProjectViewState::MIN_ZOOM_FACTOR + ProjectViewState::ZOOM_EPSILON_FACTOR
    });
    let zoom_in_disabled = Memo::new(move |_| {
        view_state.zoom_factor.get()
            >= ProjectViewState::MAX_ZOOM_FACTOR - ProjectViewState::ZOOM_EPSILON_FACTOR
    });

    view! {
        <footer class="status-bar">
            <div class="status-bar-left">
                <button
                    on:click=move |_| is_open.set(true)
                    class="status-bar-button status-bar-button--canvas"
                    title="Change canvas size"
                >
                    {move || project.with(|project| format!("{}×{} px", project.width.get(), project.height.get()))}
                </button>
                <span>"x = 0, y = 0"</span>
            </div>
            <div class="status-bar-right">
                <button
                    on:click=move |_| view_state.zoom_out_by_step()
                    title="Zoom out"
                    class="status-bar-button"
                    prop:disabled=move || zoom_out_disabled.get()
                >
                    "-"
                </button>
                <button
                    on:click=move |_| view_state.reset_zoom_to_100()
                    title="Reset zoom (100%)"
                    class="status-bar-button"
                >
                    "="
                </button>
                <button
                    on:click=move |_| view_state.zoom_in_by_step()
                    title="Zoom in"
                    class="status-bar-button"
                    prop:disabled=move || zoom_in_disabled.get()
                >
                    "+"
                </button>
                <input
                    type="number"
                    min="5"
                    max="3200"
                    step="10"
                    title="Zoom (%)"
                    class="status-bar-zoom-input"
                    prop:value=move || view_state.zoom_percent().to_string()
                    on:change=move |ev| {
                        let s = event_target_value(&ev);
                        if let Ok(pct) = s.trim().parse::<f32>() {
                            view_state.set_zoom_percent(pct);
                        }
                    }
                />
                <span class="status-bar-percent-label">"%"</span>
            </div>
        </footer>
    }
}
