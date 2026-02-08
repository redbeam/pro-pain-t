//! Pan tool implementation.
use serde::{Deserialize, Serialize};

use crate::tools::context::ToolContext;
use web_sys::PointerEvent;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PanState {
    is_panning: bool,
    pointer_id: Option<i32>,
    last_pos: Option<(f32, f32)>,
}

impl Default for PanState {
    fn default() -> Self {
        Self {
            is_panning: false,
            pointer_id: None,
            last_pos: None,
        }
    }
}

impl PanState {
    pub fn on_pointer_down(&mut self, e: &PointerEvent, _ctx: &ToolContext) {
        if self.is_panning {
            return;
        }

        self.is_panning = true;
        self.pointer_id = Some(e.pointer_id());
        self.last_pos = Some((e.client_x() as f32, e.client_y() as f32));
    }

    pub fn on_pointer_move(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        if !self.is_panning {
            return;
        }
        if self.pointer_id != Some(e.pointer_id()) {
            return;
        }

        let client_x = e.client_x() as f32;
        let client_y = e.client_y() as f32;

        let Some((lx, ly)) = self.last_pos else {
            self.last_pos = Some((client_x, client_y));
            return;
        };

        let dx = client_x - lx;
        let dy = client_y - ly;
        if !dx.is_finite() || !dy.is_finite() {
            return;
        }

        self.last_pos = Some((client_x, client_y));
        ctx.view_state.pan_by(dx, dy);
    }

    pub fn on_pointer_up(&mut self, e: &PointerEvent) {
        if self.pointer_id != Some(e.pointer_id()) {
            return;
        }
        self.is_panning = false;
        self.pointer_id = None;
        self.last_pos = None;
    }

    pub fn cancel(&mut self) {
        self.is_panning = false;
        self.pointer_id = None;
        self.last_pos = None;
    }

    pub fn cursor(&self) -> &'static str {
        if self.is_panning { "grabbing" } else { "grab" }
    }
}
