//! Pan tool implementation.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PanOffset {
	pub x: f32,
	pub y: f32,
}

impl PanOffset {
	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}

	pub fn translate_by(&mut self, dx: f32, dy: f32) {
		if !dx.is_finite() || !dy.is_finite() {
			return;
		}
		self.x += dx;
		self.y += dy;
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanAction {
	Started,
	Delta { dx: f32, dy: f32 },
	Stopped,
}

#[derive(Debug, Default, Clone)]
pub struct PanTool {
	is_panning: bool,
	pointer_id: Option<i32>,
	last_pos: Option<(f32, f32)>,
	allowed_buttons: [bool; 3],
}

impl PanTool {
	pub fn new() -> Self {
		Self {
			is_panning: false,
			pointer_id: None,
			last_pos: None,
			allowed_buttons: [true, false, false],
		}
	}

	pub fn set_allowed_buttons(&mut self, primary: bool, middle: bool, secondary: bool) {
		self.allowed_buttons = [primary, middle, secondary];
	}

	pub fn is_panning(&self) -> bool {
		self.is_panning
	}

	pub fn cursor(&self) -> &'static str {
		if self.is_panning { "grabbing" } else { "grab" }
	}

	pub fn on_pointer_down(
		&mut self,
		button: i16,
		pointer_id: i32,
		client_x: f32,
		client_y: f32,
	) -> Option<PanAction> {
		if self.is_panning {
			return None;
		}

		let allow = match button {
			0 => self.allowed_buttons[0],
			1 => self.allowed_buttons[1],
			2 => self.allowed_buttons[2],
			_ => false,
		};

		if !allow {
			return None;
		}

		self.is_panning = true;
		self.pointer_id = Some(pointer_id);
		self.last_pos = Some((client_x, client_y));
		Some(PanAction::Started)
	}

	pub fn on_pointer_move(
		&mut self,
		pointer_id: i32,
		client_x: f32,
		client_y: f32,
	) -> Option<PanAction> {
		if !self.is_panning || self.pointer_id != Some(pointer_id) {
			return None;
		}

		let (lx, ly) = self.last_pos?;
		let dx = client_x - lx;
		let dy = client_y - ly;
		self.last_pos = Some((client_x, client_y));

		if !dx.is_finite() || !dy.is_finite() {
			return None;
		}

		if dx.abs() < 0.000_1 && dy.abs() < 0.000_1 {
			return None;
		}

		Some(PanAction::Delta { dx, dy })
	}

	pub fn on_pointer_up(&mut self, pointer_id: i32) -> Option<PanAction> {
		if !self.is_panning || self.pointer_id != Some(pointer_id) {
			return None;
		}

		self.is_panning = false;
		self.pointer_id = None;
		self.last_pos = None;
		Some(PanAction::Stopped)
	}

	pub fn cancel(&mut self) {
		self.is_panning = false;
		self.pointer_id = None;
		self.last_pos = None;
	}
}

