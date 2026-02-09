use web_sys::HtmlCanvasElement;

pub fn screen_to_canvas(
    canvas: &HtmlCanvasElement,
    client_x: f64,
    client_y: f64,
    zoom: f32,
    pan_x: f32,
    pan_y: f32,
) -> (i32, i32) {
    let rect = canvas.get_bounding_client_rect();
    let x = ((client_x - rect.left() - pan_x as f64) / zoom as f64).floor() as i32;
    let y = ((client_y - rect.top() - pan_y as f64) / zoom as f64).floor() as i32;
    (x, y)
}
