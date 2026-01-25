use crate::structs::{color::Color, history::History, layer::Layer};
use leptos::prelude::{Get, RwSignal, Set, Update};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub width: RwSignal<u32>,
    pub height: RwSignal<u32>,
    pub background_color: Color,
    pub layers: RwSignal<Vec<Layer>>,
    pub history: History,
    pub next_layer_id: RwSignal<usize>, // best approach for serializing ids
    pub selected_layer_id: RwSignal<Option<usize>>,
}

impl Project {
    pub fn new(
        name: String,
        width: u32,
        height: u32,
        background_color: Color,
    ) -> Self {
        Self {
            name,
            width: RwSignal::new(width),
            height: RwSignal::new(height),
            background_color,
            layers: RwSignal::new(vec![Layer::new(
                0,
                "Layer 0".to_string(),
                width,
                height,
                background_color,
            )]),
            history: History::new(10),
            next_layer_id: RwSignal::new(1),
            selected_layer_id: RwSignal::new(None),
        }
    }

    pub fn default() -> Self {
        Self::new(
            "Unnamed project".to_string(),
            300,
            300,
            Color::default_white(),
        )
    }

    pub fn from_file_data(data: Vec<u8>) -> Self {
        ron::de::from_bytes(&*data).expect("Failed to deserialize project")
    }

    pub fn replace_project_with_blank(
        &mut self,
        name: String,
        width: u32,
        height: u32,
        background_color: Color,
    ) {
        self.name = name;
        self.width.set(width);
        self.height.set(height);
        self.background_color = background_color;
        self.layers.set(vec![Layer::new(
            0,
            "Layer 0".to_string(),
            width,
            height,
            background_color,
        )]);
        self.history = History::new(10);
        self.next_layer_id.set(1);
    }

    pub fn replace_project_with(&mut self, new_project: Project) {
        self.name = new_project.name;
        self.width.set(new_project.width.get());
        self.height.set(new_project.height.get());
        self.background_color = new_project.background_color;
        self.layers.set(new_project.layers.get());
        self.history = new_project.history;
        self.next_layer_id.set(new_project.next_layer_id.get());
    }

    pub fn add_new_layer(&self, layer: Layer) {
        self.layers.update(|layers| {
            layers.push(layer);
        });
        self.next_layer_id.set(self.next_layer_id.get() + 1);
    }

    pub fn layer_count(&self) -> usize {
        self.layers.get().len()
    }

    pub fn serialize(&self) -> String {
        ron::ser::to_string(self).expect("Couldn't serialize project")
    }
}
