use leptos::prelude::RwSignal;
use web_sys::HtmlCanvasElement;

use crate::{structs::project::Project, view_state::ProjectViewState};

/// Shared context passed to tools on input events.
/// keep CanvasArea as a thin glue layer and keep tool logic inside tool modules
pub struct ToolContext<'a> {
    pub canvas: &'a HtmlCanvasElement,
    pub project: &'a RwSignal<Project>,
    pub view_state: &'a ProjectViewState,

    pub viewport_w: f32,
    pub viewport_h: f32,

    pub zoom: f32,
    
    pub pan_x: f32,
    pub pan_y: f32,

    pub selected_layer: Option<usize>,
}
