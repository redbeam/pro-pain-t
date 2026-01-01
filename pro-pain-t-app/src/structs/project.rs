use leptos::prelude::{Get, RwSignal, Set, Update};

use crate::structs::{color::Color, history::History, layer::Layer};
#[allow(dead_code, unused_variables)]
pub struct Project {
    name: String,
    pub width: u32,
    pub height: u32,
    background_color: Color,
    pub layers: RwSignal<Vec<Layer>>,
    history: History,
    pub next_layer_id: RwSignal<usize>, // best approach for seriliazing ids
}

impl Project {
    pub fn new(name: String, width: u32, height: u32, background_color: Color) -> Self {
        Self {
            name,
            width,
            height,
            background_color,
            layers: RwSignal::new(vec![Layer::new(0, "Layer 0".to_string(), width, height, background_color)]),
            history: History::new(10),
            next_layer_id: RwSignal::new(1),
        }
    }

    pub fn add_new_layer(&mut self) {
        self.layers.update(|layers| {
            layers.push(Layer::new(
            self.next_layer_id.get(),
            self.name.clone(),
            self.width,
            self.height,
            self.background_color,
        ));});

        self.next_layer_id.set(self.next_layer_id.get() + 1);
    }
}
