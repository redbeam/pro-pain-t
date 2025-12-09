use crate::structs::{color::Color, history::History, layer::Layer};
#[allow(dead_code, unused_variables)]
pub struct Project {
    name: String,
    width: u32,
    height: u32,
    background_color: Color,
    layers: Vec<Layer>,
    history: History,
    next_layer_id: usize, // best approach for seriliazing ids
}

impl Project {
    pub fn new(name: String, width: u32, height: u32, background_color: Color) -> Self {
        Self {
            name,
            width,
            height,
            background_color,
            layers: vec![Layer::new(0, width, height, background_color)],
            history: History::new(10),
            next_layer_id: 1,
        }
    }

    pub fn add_new_layer(&mut self) {
        self.layers.push(Layer::new(
            self.next_layer_id,
            self.width,
            self.height,
            self.background_color,
        ));
        self.next_layer_id += 1;
    }
}
