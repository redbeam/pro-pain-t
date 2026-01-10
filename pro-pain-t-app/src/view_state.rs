use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct ProjectViewState {
    pub zoom: RwSignal<f32>,
}

impl ProjectViewState {
    pub const MIN_ZOOM: f32 = 0.05; // 5%
    pub const MAX_ZOOM: f32 = 32.0; // 3200%
    pub const ZOOM_STEP: f32 = 1.25;

    pub fn new() -> Self {
        Self {
            zoom: RwSignal::new(1.0),
        }
    }

    pub fn set_zoom(&self, zoom: f32) {
        let zoom = if zoom.is_finite() { zoom } else { 1.0 };
        self.zoom.set(zoom.clamp(Self::MIN_ZOOM, Self::MAX_ZOOM));
    }

    pub fn reset_zoom(&self) {
        self.set_zoom(1.0);
    }

    pub fn zoom_in(&self) {
        self.set_zoom(self.zoom.get() * Self::ZOOM_STEP);
    }

    pub fn zoom_out(&self) {
        self.set_zoom(self.zoom.get() / Self::ZOOM_STEP);
    }

    pub fn zoom_percent(&self) -> u32 {
        (self.zoom.get() * 100.0).round().max(1.0) as u32
    }
}
