use web_sys::HtmlCanvasElement;

pub fn screen_to_canvas(
    canvas: &HtmlCanvasElement,
    client_x: f64,
    client_y: f64,
    zoom: f32,
) -> Option<(i32, i32)> {
    let rect = canvas.get_bounding_client_rect();

    let x = ((client_x - rect.left()) / zoom as f64).floor() as i32;
    let y = ((client_y - rect.top()) / zoom as f64).floor() as i32;

    if x < 0 || y < 0 ||
       x >= canvas.width() as i32 ||
       y >= canvas.height() as i32 {
        None
    } else {
        Some((x, y))
    }
}
