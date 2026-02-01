use crate::components::color_picker::ColorPicker;
use crate::{
    state::workspace_state::WorkspaceState,
    structs::project::Project,
    tools::{pen::PenState, tools::Tool},
};
use leptos::prelude::*;

#[component]
pub fn ToolPalette() -> impl IntoView {
    let project = use_context::<RwSignal<Project>>().unwrap();
    let workspace_state = use_context::<WorkspaceState>().expect("WorkspaceState context missing");

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
                    style=move || format!(
                        "width:24px; height:24px; background:{}; font-size:1rem; display: flex; align-items: center; justify-content: center; border-radius: 2px; border: 1px solid {};",
                        if workspace_state.current_tool.get().is_pan() { "#2f3e66" } else { "#3a3a3a" },
                        if workspace_state.current_tool.get().is_pan() { "#4a7cff" } else { "transparent" }
                    )
                    on:click=move |_| {
                        use crate::tools::{pan::PanState, tools::Tool};
                        workspace_state.current_tool.set(Tool::Pan(PanState::default()));
                    }
                    title="Pan tool"
                >
                "🤚🏻"
                </div>
                <div
                    style=move || format!(
                        "width:24px; height:24px; background:{}; font-size:1rem; display: flex; align-items: center; justify-content: center; border-radius: 2px; border: 1px solid {};",
                        if !workspace_state.current_tool.get().is_pan() { "#2f3e66" } else { "#3a3a3a" },
                        if !workspace_state.current_tool.get().is_pan() { "#4a7cff" } else { "transparent" }
                    )
                    on:click=move |_| {
                        workspace_state.current_tool.set(Tool::Pen(PenState::default()));
                    }
                    title="Pen tool"
                >
                "🖊️"
                </div>
                { (0..10).map(|_| view! { <div style="width:24px; height:24px; background:#3a3a3a; border-radius:2px;"></div> }).collect_view() }
            </div>

            <ColorPicker color=project.get().current_color />
        </nav>
    }
}
