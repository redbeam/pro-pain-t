use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct ProjectViewState {
    pub zoom_factor: RwSignal<f32>,
    pub pan_x: RwSignal<f32>,
    pub pan_y: RwSignal<f32>,
    pub did_center_view: RwSignal<bool>,
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
            pan_x: RwSignal::new(0.0),
            pan_y: RwSignal::new(0.0),
            did_center_view: RwSignal::new(false),
        }
    }

    pub fn ensure_centered_once(
        &self,
        viewport_w_css: f32,
        viewport_h_css: f32,
        project_w: u32,
        project_h: u32,
        zoom: f32,
    ) -> Option<(f32, f32)> {
        if self.did_center_view.get_untracked() {
            return None;
        }

        let target_pan_x = (viewport_w_css - (project_w as f32 * zoom)) / 2.0;
        let target_pan_y = (viewport_h_css - (project_h as f32 * zoom)) / 2.0;

        self.pan_x.set(target_pan_x);
        self.pan_y.set(target_pan_y);
        self.did_center_view.set(true);
        Some((target_pan_x, target_pan_y))
    }

    pub fn pan_by(&self, dx: f32, dy: f32) {
        if !dx.is_finite() || !dy.is_finite() {
            return;
        }
        self.pan_x.update(|x| *x += dx);
        self.pan_y.update(|y| *y += dy);
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
