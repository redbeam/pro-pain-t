use crate::components::color_picker::ColorPicker;
use crate::{
    state::workspace_state::WorkspaceState,
    structs::project::Project,
    tools::{pan::PanState, pen::PenState, select::SelectState, tools::Tool},
};
use leptos::prelude::*;

#[component]
pub fn ToolPalette() -> impl IntoView {
    let project = use_context::<RwSignal<Project>>().unwrap();
    let workspace_state = use_context::<WorkspaceState>().expect("WorkspaceState context missing");
    let current_color = project.with_untracked(|p| p.current_color);

    view! {
        <nav class="tool-palette">
            <div class="tool-palette-grid">
                <div
                    class="tool-button"
                    class=("tool-button--active", move || workspace_state.current_tool.get().is_pan())
                    on:click=move |_| {
                        workspace_state.current_tool.set(Tool::Pan(PanState::default()));
                    }
                    title="Pan tool"
                >
                "🤚🏻"
                </div>
                <div
                    class="tool-button"
                    class=("tool-button--active", move || matches!(workspace_state.current_tool.get(), Tool::Select(_)))
                    on:click=move |_| {
                        workspace_state.current_tool.set(Tool::Select(SelectState::default()));
                    }
                    title="Select tool"
                >
                "🔲"
                </div>
                <div
                    class="tool-button"
                    class=("tool-button--active", move || matches!(workspace_state.current_tool.get(), Tool::Pen(_)))
                    on:click=move |_| {
                        workspace_state.current_tool.set(Tool::Pen(PenState::default()));
                    }
                    title="Pen tool"
                >
                "🖊️"
                </div>
                { (0..10).map(|_| view! { <div class="tool-button tool-button--placeholder"></div> }).collect_view() }
            </div>

            <ColorPicker color=current_color />
        </nav>
    }
}
