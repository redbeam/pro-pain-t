use crate::components::color_picker::ColorPicker;
use crate::tools::eraser::EraserState;
use crate::tools::eyedropper::EyedropperState;
use crate::{
    state::workspace_state::WorkspaceState,
    structs::project::Project,
    tools::{pan::PanState, pen::PenState, select::SelectState, tools::Tool, bucket::BucketState},
};
use leptos::prelude::*;
use crate::components::brush_size_slider::BrushSizeSlider;

#[component]
pub fn ToolPalette() -> impl IntoView {
    let project = use_context::<RwSignal<Project>>().expect("Project context missing");
    let workspace_state = use_context::<WorkspaceState>().expect("WorkspaceState context missing");
    let current_color = project.with_untracked(|p| p.current_color);
    let current_tool = workspace_state.current_tool;

    view! {
        <nav class="tool-palette">
            <div class="tool-palette-grid">
                <div
                    class="tool-button"
                    class=("tool-button--active", move || current_tool.get().is_pan())
                    on:click=move |_| {
                        workspace_state.set_tool(Tool::Pan(PanState::default()), &project);
                    }
                    title="Pan tool"
                >
                "🤚🏻"
                </div>
                <div
                    class="tool-button"
                    class=("tool-button--active", move || matches!(current_tool.get(), Tool::Select(_)))
                    on:click=move |_| {
                        workspace_state.set_tool(Tool::Select(SelectState::default()), &project);
                    }
                    title="Select tool"
                >
                "🔲"
                </div>
                <div
                    class="tool-button"
                    class=("tool-button--active", move || matches!(current_tool.get(), Tool::Pen(_)))
                    on:click=move |_| {
                        workspace_state.set_tool(Tool::Pen(PenState::default()), &project);
                    }
                    title="Pen tool"
                >
                "🖊️"
                </div>
                <div
                    class="tool-button"
                    class=("tool-button--active", move || matches!(current_tool.get(), Tool::Bucket(_)))
                    on:click=move |_| {
                        workspace_state.set_tool(Tool::Bucket(BucketState::default()), &project);
                    }
                    title="Bucket tool"
                >
                "🧺"
                </div>
                <div
                    class="tool-button"
                    class=("tool-button--active", move || matches!(current_tool.get(), Tool::EyeDropper(_)))
                    on:click=move |_| {
                        workspace_state.set_tool(Tool::EyeDropper(EyedropperState::default()), &project);
                    }
                    title="EyeDropper tool"
                >
                "🩸"
                </div>
                <div
                    class="tool-button"
                    class=("tool-button--active", move || matches!(current_tool.get(), Tool::Eraser(_)))
                    on:click=move |_| {
                        workspace_state.set_tool(Tool::Eraser(EraserState::default()), &project);
                    }
                    title="Eraser tool"
                >
                "🧽"
                </div>
                { (0..6).map(|_| view! { <div class="tool-button tool-button--placeholder"></div> }).collect_view() }
            </div>

            <ColorPicker color = current_color />
            <BrushSizeSlider brush_size = workspace_state.brush_size />
        </nav>
    }
}
