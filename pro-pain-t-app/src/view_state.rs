use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct ProjectViewState {
    pub zoom_factor: RwSignal<f32>,
}

impl ProjectViewState {
    /// Minimum zoom scaling factor (e.g. `0.05` = 5%).
    pub const MIN_ZOOM_FACTOR: f32 = 0.05;
    /// Maximum zoom scaling factor (e.g. `32.0` = 3200%).
    pub const MAX_ZOOM_FACTOR: f32 = 32.0;
    pub const ZOOM_STEP_PERCENT_POINTS: f32 = 10.0;
    pub const ZOOM_EPSILON_FACTOR: f32 = 0.000_01;

    pub fn new() -> Self {
        Self {
            zoom_factor: RwSignal::new(1.0),
        }
    }

    pub fn set_zoom_factor(&self, factor: f32) {
        let factor = if factor.is_finite() { factor } else { 1.0 };
        self.zoom_factor
            .set(factor.clamp(Self::MIN_ZOOM_FACTOR, Self::MAX_ZOOM_FACTOR));
    }

    pub fn set_zoom_percent(&self, percent: f32) {
        let percent = if percent.is_finite() { percent } else { 100.0 };
        self.set_zoom_factor(percent / 100.0);
    }

    pub fn reset_zoom_to_100(&self) {
        self.set_zoom_factor(1.0);
    }

    pub fn zoom_in_by_step(&self) {
        let next = self.zoom_percent() as f32 + Self::ZOOM_STEP_PERCENT_POINTS;
        self.set_zoom_percent(next);
    }

    pub fn zoom_out_by_step(&self) {
        let next = self.zoom_percent() as f32 - Self::ZOOM_STEP_PERCENT_POINTS;
        self.set_zoom_percent(next);
    }

    pub fn zoom_percent(&self) -> u32 {
        (self.zoom_factor.get() * 100.0).round().max(1.0) as u32
    }
}
