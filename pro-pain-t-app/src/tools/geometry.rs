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

// Bresenham line helper
pub fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, mut plot: impl FnMut(i32, i32)) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        plot(x, y);

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
