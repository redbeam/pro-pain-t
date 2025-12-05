pub struct Project {
    name: String,
    canvas_list: Vec<Canvas>
}

impl Project {
    pub fn new(name: String, width: u32, height: u32, background_color: Color) -> Self {
        Project {
            name,
            canvas_list: Vec::new(Layer::new(width, height, background_color)),
        }
    }
}