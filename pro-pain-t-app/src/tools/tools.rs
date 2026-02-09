use serde::{Deserialize, Serialize};
use web_sys::PointerEvent;

use crate::tools::{context::ToolContext, pan::PanState, pen::PenState};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tool {
    Pen(PenState),
    Pan(PanState),
}

impl Tool {
    pub fn is_pan(&self) -> bool {
        matches!(self, Tool::Pan(_))
    }

    pub fn on_pointer_down(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        match self {
            Tool::Pen(state) => state.on_pointer_down(e, ctx),
            Tool::Pan(state) => state.on_pointer_down(e, ctx),
        }
    }

    pub fn on_pointer_move(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        match self {
            Tool::Pen(state) => state.on_pointer_move(e, ctx),
            Tool::Pan(state) => state.on_pointer_move(e, ctx),
        }
    }

    pub fn on_pointer_up(&mut self, e: &PointerEvent) {
        match self {
            Tool::Pen(state) => state.on_pointer_up(e),
            Tool::Pan(state) => state.on_pointer_up(e),
        }
    }

    pub fn on_pointer_cancel(&mut self) {
        match self {
            Tool::Pen(state) => state.cancel(),
            Tool::Pan(state) => state.cancel(),
        }
    }

    pub fn cursor(&self) -> &'static str {
        match self {
            Tool::Pen(state) => state.cursor(),
            Tool::Pan(state) => state.cursor(),
        }
    }
}
