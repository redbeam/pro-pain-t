use leptos::prelude::*;
use pro_pain_t_app::{structs::project::Project, tools::{pen::PenState, tools::Tool}};

use crate::components::color_picker::ColorPicker;

#[component]
pub fn ToolPalette() -> impl IntoView {
    let project = use_context::<RwSignal<Project>>().unwrap();

    view! {
        <nav
            style="
                width:160px;
                background:#262626;
                color:#f5f5f5;
                display:flex;
                flex-direction:column;
                padding:0.5rem;
                gap:0.75rem;
                box-sizing:border-box;
                font-size:0.75rem;
                font-family:system-ui, sans-serif;
            "
        >
            <div
                style="
                    display:grid;
                    grid-template-columns:repeat(4, 1fr);
                    gap:0.25rem;
                    cursor: pointer;
                "
            >
                <div
                    // TODO: show pen icon
                    style="width:24px; height:24px; background:#ffff00; border-radius:2px;"
                    on:click=move |_| {
                        project.get().current_tool.update(|_| {Tool::Pen(PenState::default());});
                    }
                />
                { (0..11).map(|_| view! { <div style="width:24px; height:24px; background:#3a3a3a; border-radius:2px;"></div> }).collect_view() }
            </div>

            <ColorPicker color=project.get().current_color />
        </nav>
    }
}
