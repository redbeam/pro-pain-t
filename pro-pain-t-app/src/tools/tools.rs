use serde::{Deserialize, Serialize};
use web_sys::PointerEvent;

use crate::tools::{brush::BrushState, bucket::BucketState, context::ToolContext, eraser::EraserState, eyedropper::EyedropperState, pan::PanState, pen::PenState, select::SelectState};

#[derive(Clone, Serialize, Deserialize)]
pub enum Tool {
    Pen(PenState),
    Pan(PanState),
    Select(SelectState),
    Bucket(BucketState),
    EyeDropper(EyedropperState),
    Eraser(EraserState),
    Brush(BrushState),
}

impl Tool {
    pub fn is_pan(&self) -> bool {
        matches!(self, Tool::Pan(_))
    }

    pub fn on_pointer_down(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        match self {
            Tool::Pen(state) => state.on_pointer_down(e, ctx),
            Tool::Pan(state) => state.on_pointer_down(e, ctx),
            Tool::Select(state) => state.on_pointer_down(e, ctx),
            Tool::Bucket(state) => state.on_pointer_down(e, ctx),
            Tool::EyeDropper(state) => state.on_pointer_down(e, ctx),
            Tool::Eraser(state) => state.on_pointer_down(e, ctx),
            Tool::Brush(state) => state.on_pointer_down(e, ctx),
        }
    }

    pub fn on_pointer_move(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        match self {
            Tool::Pen(state) => state.on_pointer_move(e, ctx),
            Tool::Pan(state) => state.on_pointer_move(e, ctx),
            Tool::Select(state) => state.on_pointer_move(e, ctx),
            Tool::Bucket(state) => state.on_pointer_move(e, ctx),
            Tool::EyeDropper(state) => state.on_pointer_move(e, ctx),
            Tool::Eraser(state) => state.on_pointer_move(e, ctx),
            Tool::Brush(state) => state.on_pointer_move(e, ctx),
        }
    }

    pub fn on_pointer_up(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        match self {
            Tool::Pen(state) => state.on_pointer_up(e, ctx),
            Tool::Pan(state) => state.on_pointer_up(e, ctx),
            Tool::Select(state) => state.on_pointer_up(e, ctx),
            Tool::Bucket(state) => state.on_pointer_up(e, ctx),
            Tool::EyeDropper(state) => state.on_pointer_up(e, ctx),
            Tool::Eraser(state) => state.on_pointer_up(e, ctx),
            Tool::Brush(state) => state.on_pointer_up(e, ctx),
        }
    }

    pub fn on_pointer_cancel(&mut self) {
        match self {
            Tool::Pen(state) => state.cancel(),
            Tool::Pan(state) => state.cancel(),
            Tool::Select(state) => state.cancel(),
            Tool::Bucket(state) => state.cancel(),
            Tool::EyeDropper(state) => state.cancel(),
            Tool::Eraser(state) => state.cancel(),
            Tool::Brush(state) => state.cancel(),
        }
    }

    pub fn cursor(&self) -> &'static str {
        match self {
            Tool::Pen(state) => state.cursor(),
            Tool::Pan(state) => state.cursor(),
            Tool::Select(state) => state.cursor(),
            Tool::Bucket(state) => state.cursor(),
            Tool::EyeDropper(state) => state.cursor(),
            Tool::Eraser(state) => state.cursor(),
            Tool::Brush(state) => state.cursor(),
        }
    }
}
