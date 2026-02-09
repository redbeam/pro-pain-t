use crate::structs::history::{PixelDiff, StrokeDiff};
use crate::structs::{color::Color, layer::Layer, project::Project};
use crate::tools::context::ToolContext;
use crate::tools::geometry::screen_to_canvas;
use crate::structs::pixel::Pixel;
use leptos::prelude::{RwSignal, Set, Update, With};
use serde::{Deserialize, Serialize};
use web_sys::PointerEvent;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResizeHandle {
    Nw,
    Ne,
    Se,
    Sw,
    N,
    E,
    S,
    W,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct SelectionRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl SelectionRect {
    pub fn is_empty(&self) -> bool {
        self.w <= 0 || self.h <= 0
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.w && y < self.y + self.h
    }

    pub fn from_points(a: (i32, i32), b: (i32, i32)) -> Self {
        let x0 = a.0.min(b.0);
        let y0 = a.1.min(b.1);
        let x1 = a.0.max(b.0);
        let y1 = a.1.max(b.1);
        Self {
            x: x0,
            y: y0,
            w: x1 - x0 + 1,
            h: y1 - y0 + 1,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SelectionBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SelectionState {
    pub layer_id: usize,
    pub rect: SelectionRect,
    pub buffer: Option<SelectionBuffer>,
    pub original_pixels: Vec<PixelDiff>,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum SelectMode {
    Idle,
    Creating { start: (i32, i32) },
    Moving { start: (i32, i32), orig: SelectionRect },
    Resizing {
        start: (i32, i32),
        orig: SelectionRect,
        handle: ResizeHandle,
    },
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SelectState {
    pointer_id: Option<i32>,
    mode: SelectMode,
    hover_handle: Option<ResizeHandle>,
    hover_inside: bool,
}

impl Default for SelectState {
    fn default() -> Self {
        Self {
            pointer_id: None,
            mode: SelectMode::Idle,
            hover_handle: None,
            hover_inside: false,
        }
    }
}

impl SelectState {
    pub fn on_pointer_down(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        if self.pointer_id.is_some() {
            return;
        }

        let Some(layer_id) = ctx.selected_layer else {
            return;
        };

        if !layer_is_editable(ctx, layer_id) {
            return;
        }

        let (x, y) = screen_to_canvas(
            ctx.canvas,
            e.client_x() as f64,
            e.client_y() as f64,
            ctx.zoom,
            ctx.pan_x,
            ctx.pan_y,
        );

        let mut clear_selection = false;
        ctx.workspace_state.selection.with(|selection| {
            if let Some(sel) = selection {
                if sel.layer_id != layer_id {
                    commit_selection(ctx.project, sel);
                    clear_selection = true;
                }
            }
        });
        if clear_selection {
            ctx.workspace_state.selection.set(None);
        }

        let mut existing_rect: Option<SelectionRect> = None;
        let mut existing_has_buffer = false;
        ctx.workspace_state.selection.with(|selection| {
            if let Some(sel) = selection {
                if sel.layer_id == layer_id {
                    existing_rect = Some(sel.rect);
                    existing_has_buffer = sel.buffer.is_some();
                }
            }
        });

        if let Some(rect) = existing_rect {
            let handle = handle_at(rect, x, y, ctx.zoom);
            if handle.is_some() || rect.contains(x, y) {
                self.pointer_id = Some(e.pointer_id());
                if !existing_has_buffer {
                    let (buffer, diffs) = cut_buffer(ctx, layer_id, rect);
                    ctx.workspace_state.selection.update(|sel| {
                        if let Some(sel) = sel.as_mut() {
                            sel.buffer = Some(buffer);

                            if !diffs.is_empty() {
                                sel.original_pixels.clear();
                                sel.original_pixels = diffs;
                            }
                        }
                    });
                }
                if let Some(handle) = handle {
                    self.mode = SelectMode::Resizing {
                        start: (x, y),
                        orig: rect,
                        handle,
                    };
                } else {
                    self.mode = SelectMode::Moving {
                        start: (x, y),
                        orig: rect,
                    };
                }
                return;
            }

            ctx.workspace_state.selection.with(|selection| {
                if let Some(sel) = selection {
                    commit_selection(ctx.project, sel);
                }
            });
        }

        let rect = SelectionRect::from_points((x, y), (x, y));
        ctx.workspace_state.selection.set(Some(SelectionState {
            layer_id,
            rect,
            buffer: None,
            original_pixels: Vec::new(),
        }));

        self.pointer_id = Some(e.pointer_id());
        self.mode = SelectMode::Creating { start: (x, y) };
    }

    pub fn on_pointer_move(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        let (x, y) = screen_to_canvas(
            ctx.canvas,
            e.client_x() as f64,
            e.client_y() as f64,
            ctx.zoom,
            ctx.pan_x,
            ctx.pan_y,
        );

        match self.mode {
            SelectMode::Creating { start } => {
                if self.pointer_id != Some(e.pointer_id()) {
                    return;
                }
                let rect = SelectionRect::from_points(start, (x, y));
                ctx.workspace_state.selection.update(|sel| {
                    if let Some(sel) = sel.as_mut() {
                        sel.rect = rect;
                    }
                });
            }
            SelectMode::Moving { start, orig } => {
                if self.pointer_id != Some(e.pointer_id()) {
                    return;
                }
                let dx = x - start.0;
                let dy = y - start.1;
                let rect = SelectionRect {
                    x: orig.x + dx,
                    y: orig.y + dy,
                    w: orig.w,
                    h: orig.h,
                };
                ctx.workspace_state.selection.update(|sel| {
                    if let Some(sel) = sel.as_mut() {
                        sel.rect = rect;
                    }
                });
            }
            SelectMode::Resizing { start, orig, handle } => {
                if self.pointer_id != Some(e.pointer_id()) {
                    return;
                }
                let dx = x - start.0;
                let dy = y - start.1;
                let rect = resize_rect(orig, handle, dx, dy);
                ctx.workspace_state.selection.update(|sel| {
                    if let Some(sel) = sel.as_mut() {
                        sel.rect = rect;
                    }
                });
            }
            SelectMode::Idle => {
                self.update_hover(ctx, x, y);
            }
        }
    }

    pub fn on_pointer_up(&mut self, e: &PointerEvent, ctx: &ToolContext) {
        if self.pointer_id != Some(e.pointer_id()) {
            return;
        }

        match self.mode {
            SelectMode::Creating { .. } => {
                let mut should_clear = false;
                ctx.workspace_state.selection.with(|selection| {
                    if let Some(sel) = selection {
                        if sel.rect.is_empty() {
                            should_clear = true;
                        }
                    }
                });
                if should_clear {
                    ctx.workspace_state.selection.set(None);
                }
            }
            SelectMode::Moving { .. } => {}
            SelectMode::Resizing { .. } => {
                let mut scaled: Option<SelectionBuffer> = None;
                ctx.workspace_state.selection.with(|selection| {
                    if let Some(sel) = selection {
                        if let Some(buffer) = sel.buffer.as_ref() {
                            scaled = Some(scale_buffer(buffer, sel.rect.w as u32, sel.rect.h as u32));
                        }
                    }
                });
                if let Some(scaled) = scaled {
                    ctx.workspace_state.selection.update(|sel| {
                        if let Some(sel) = sel.as_mut() {
                            sel.buffer = Some(scaled);
                        }
                    });
                }
            }
            SelectMode::Idle => {}
        }

        self.pointer_id = None;
        self.mode = SelectMode::Idle;
    }

    pub fn cancel(&mut self) {
        self.pointer_id = None;
        self.mode = SelectMode::Idle;
    }

    pub fn cursor(&self) -> &'static str {
        if let SelectMode::Resizing { handle, .. } = self.mode {
            return cursor_for_handle(handle);
        }
        if let SelectMode::Moving { .. } = self.mode {
            return "move";
        }
        if let Some(handle) = self.hover_handle {
            return cursor_for_handle(handle);
        }
        if self.hover_inside {
            return "move";
        }
        "crosshair"
    }

    fn update_hover(&mut self, ctx: &ToolContext, x: i32, y: i32) {
        let mut handle = None;
        let mut inside = false;
        ctx.workspace_state.selection.with(|selection| {
            if let Some(sel) = selection {
                handle = handle_at(sel.rect, x, y, ctx.zoom);
                inside = sel.rect.contains(x, y);
            }
        });
        self.hover_handle = handle;
        self.hover_inside = inside;
    }
}

fn layer_is_editable(ctx: &ToolContext, layer_id: usize) -> bool {
    let mut editable = false;
    ctx.project.with(|project| {
        project.layers.with(|layers| {
            if let Some(layer) = layers.iter().find(|l| l.id == layer_id) {
                editable = !layer.is_locked && layer.is_visible;
            }
        });
    });
    editable
}

fn handle_at(rect: SelectionRect, x: i32, y: i32, zoom: f32) -> Option<ResizeHandle> {
    if rect.is_empty() {
        return None;
    }
    let tol = (6.0 / zoom.max(0.01)) as f32;
    let xf = x as f32;
    let yf = y as f32;
    let left = rect.x as f32;
    let right = (rect.x + rect.w) as f32;
    let top = rect.y as f32;
    let bottom = (rect.y + rect.h) as f32;

    let near_left = (xf - left).abs() <= tol;
    let near_right = (xf - right).abs() <= tol;
    let near_top = (yf - top).abs() <= tol;
    let near_bottom = (yf - bottom).abs() <= tol;

    if near_left && near_top {
        return Some(ResizeHandle::Nw);
    }
    if near_right && near_top {
        return Some(ResizeHandle::Ne);
    }
    if near_right && near_bottom {
        return Some(ResizeHandle::Se);
    }
    if near_left && near_bottom {
        return Some(ResizeHandle::Sw);
    }
    let between_h = yf > top + tol && yf < bottom - tol;
    let between_w = xf > left + tol && xf < right - tol;
    if near_left && between_h {
        return Some(ResizeHandle::W);
    }
    if near_right && between_h {
        return Some(ResizeHandle::E);
    }
    if near_top && between_w {
        return Some(ResizeHandle::N);
    }
    if near_bottom && between_w {
        return Some(ResizeHandle::S);
    }
    None
}

fn cursor_for_handle(handle: ResizeHandle) -> &'static str {
    match handle {
        ResizeHandle::Nw | ResizeHandle::Se => "nwse-resize",
        ResizeHandle::Ne | ResizeHandle::Sw => "nesw-resize",
        ResizeHandle::N | ResizeHandle::S => "ns-resize",
        ResizeHandle::E | ResizeHandle::W => "ew-resize",
    }
}

fn cut_buffer(ctx: &ToolContext, layer_id: usize, rect: SelectionRect) -> (SelectionBuffer, Vec<PixelDiff>) {
    let mut buffer = SelectionBuffer {
        width: rect.w.max(1) as u32,
        height: rect.h.max(1) as u32,
        pixels: Vec::new(),
    };

    let mut diffs = Vec::new();

    ctx.project.update(|project| {
        project.layers.update(|layers| {
            let Some(layer) = layers.iter_mut().find(|l| l.id == layer_id) else {
                return;
            };

            buffer = extract_buffer_from_layer(layer, rect);

            clear_rect(layer, rect, &mut diffs);
        });
    });

    (buffer, diffs)
}

fn extract_buffer_from_layer(layer: &mut Layer, rect: SelectionRect) -> SelectionBuffer {
    let width = rect.w.max(1) as u32;
    let height = rect.h.max(1) as u32;
    let mut pixels = Vec::with_capacity((width * height) as usize);

    for y in 0..height {
        for x in 0..width {
            let px = rect.x + x as i32;
            let py = rect.y + y as i32;
            if px < 0 || py < 0 {
                pixels.push(transparent_color());
                continue;
            }
            if px as u32 >= layer.canvas.width || py as u32 >= layer.canvas.height {
                pixels.push(transparent_color());
                continue;
            }
            let idx = (py as u32 * layer.canvas.width + px as u32) as usize;
            let color = layer.canvas.content[idx].color;
            pixels.push(color);
        }
    }

    SelectionBuffer {
        width,
        height,
        pixels,
    }
}

pub fn commit_selection(project: &RwSignal<Project>, selection: &SelectionState) {
    let Some(buffer) = selection.buffer.as_ref() else { return; };
    let rect = selection.rect;
    let layer_id = selection.layer_id;
    let sel = selection.clone();

    project.update(|project| {
        project.layers.update(|layers| {
            let Some(layer) = layers.iter_mut().find(|l| l.id == layer_id) else {
                return;
            };

            let mut diffs = Vec::new();

            clear_rect(layer, rect, &mut diffs);

            apply_buffer(layer, rect, buffer, &mut diffs);

            diffs.extend(sel.original_pixels.clone());

            if !diffs.is_empty() {
                project.history.add(StrokeDiff {
                    layer_id,
                    pixels: diffs,
                });
            }
        });
    });
}

fn clear_rect(
    layer: &mut Layer,
    rect: SelectionRect,
    diffs: &mut Vec<PixelDiff>,
) {
    let width = rect.w.max(1) as u32;
    let height = rect.h.max(1) as u32;

    for y in 0..height {
        for x in 0..width {
            let px = rect.x + x as i32;
            let py = rect.y + y as i32;
            if px < 0 || py < 0 { continue; }
            if px as u32 >= layer.canvas.width || py as u32 >= layer.canvas.height { continue; }

            let ux = px as u32;
            let uy = py as u32;
            let idx = (uy * layer.canvas.width + ux) as usize;

            let before = layer.canvas.content[idx].clone();
            let after = Color { r:0,g:0,b:0, alpha:0.0 };

            if before.color == after { continue; }

            diffs.push(PixelDiff {
                before: before.clone().into(),
                after: Pixel::new(ux, uy, after),
            });

            layer.canvas.content[idx].color = after;
        }
    }
}

fn apply_buffer(
    layer: &mut Layer,
    rect: SelectionRect,
    buffer: &SelectionBuffer,
    diffs: &mut Vec<PixelDiff>,
) {
    let width = rect.w.max(1) as u32;
    let height = rect.h.max(1) as u32;

    for y in 0..height {
        for x in 0..width {
            let px = rect.x + x as i32;
            let py = rect.y + y as i32;
            if px < 0 || py < 0 { continue; }
            if px as u32 >= layer.canvas.width || py as u32 >= layer.canvas.height { continue; }

            let ux = px as u32;
            let uy = py as u32;
            let idx = (uy * layer.canvas.width + ux) as usize;
            let src_idx = (y * buffer.width + x) as usize;

            let new_color = buffer.pixels.get(src_idx).copied().unwrap_or_else(transparent_color);
            let before = layer.canvas.content[idx].clone();

            if before.color == new_color { continue; }

            diffs.push(PixelDiff {
                before: before.clone().into(),
                after: Pixel::new(ux, uy, new_color),
            });

            layer.canvas.content[idx].color = new_color;
        }
    }
}

fn scale_buffer(buffer: &SelectionBuffer, new_w: u32, new_h: u32) -> SelectionBuffer {
    let new_w = new_w.max(1);
    let new_h = new_h.max(1);
    let mut pixels = Vec::with_capacity((new_w * new_h) as usize);

    for y in 0..new_h {
        for x in 0..new_w {
            let src_x = (x as u64 * buffer.width as u64) / new_w as u64;
            let src_y = (y as u64 * buffer.height as u64) / new_h as u64;
            let src_idx = (src_y as u32 * buffer.width + src_x as u32) as usize;
            let color = buffer.pixels.get(src_idx).copied().unwrap_or_else(transparent_color);
            pixels.push(color);
        }
    }

    SelectionBuffer {
        width: new_w,
        height: new_h,
        pixels,
    }
}

fn resize_rect(orig: SelectionRect, handle: ResizeHandle, dx: i32, dy: i32) -> SelectionRect {
    let mut left = orig.x;
    let mut right = orig.x + orig.w - 1;
    let mut top = orig.y;
    let mut bottom = orig.y + orig.h - 1;

    match handle {
        ResizeHandle::Nw => {
            left += dx;
            top += dy;
        }
        ResizeHandle::Ne => {
            right += dx;
            top += dy;
        }
        ResizeHandle::Se => {
            right += dx;
            bottom += dy;
        }
        ResizeHandle::Sw => {
            left += dx;
            bottom += dy;
        }
        ResizeHandle::N => {
            top += dy;
        }
        ResizeHandle::E => {
            right += dx;
        }
        ResizeHandle::S => {
            bottom += dy;
        }
        ResizeHandle::W => {
            left += dx;
        }
    }

    if right < left {
        right = left;
    }
    if bottom < top {
        bottom = top;
    }

    SelectionRect {
        x: left,
        y: top,
        w: right - left + 1,
        h: bottom - top + 1,
    }
}

fn transparent_color() -> Color {
    Color {
        r: 0,
        g: 0,
        b: 0,
        alpha: 0.0,
    }
}
