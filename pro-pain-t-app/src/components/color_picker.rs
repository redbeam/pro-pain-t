use leptos::html::Canvas;
use leptos::prelude::*;
use leptos::web_sys;
use pro_pain_t_app::structs::color::Color;
use web_sys::{ CanvasRenderingContext2d };
use leptos::wasm_bindgen::JsCast;

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h {
        h if h < 60.0  => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _              => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}


#[component]
pub fn ColorPicker(
    #[prop(into)] color: RwSignal<Color>
) -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();

    let hue = RwSignal::new(0.0f32);
    let sat = RwSignal::new(1.0f32);
    let val = RwSignal::new(1.0f32);
    let alpha = RwSignal::new(1.0f32);
    let red = RwSignal::new(255u8);
    let green = RwSignal::new(255u8);
    let blue = RwSignal::new(255u8);

    Effect::new(move |_| {
        let canvas = canvas_ref.get().unwrap();
        let ctx = canvas
            .get_context("2d").unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>().unwrap();

        let size = canvas.width() as f32;
        let r = size / 2.0;

        for y in 0..size as i32 {
            for x in 0..size as i32 {
                let dx = x as f32 - r;
                let dy = y as f32 - r;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist <= r {
                    let s = dist / r;
                    let h = dy.atan2(dx).to_degrees().rem_euclid(360.0);
                    let (rr, gg, bb) = hsv_to_rgb(h, s, val.get());
                    ctx.set_fill_style_str(&format!("rgb({},{},{})", rr, gg, bb));
                    ctx.fill_rect(x as f64, y as f64, 1.0, 1.0);
                }
            }
        }
    });

    let on_click = move |ev: web_sys::MouseEvent| {
        let canvas = canvas_ref.get().unwrap();

        let rect = canvas
            .dyn_ref::<web_sys::HtmlElement>()
            .unwrap()
            .get_bounding_client_rect();

        let x = ev.client_x() as f32 - rect.left() as f32;
        let y = ev.client_y() as f32 - rect.top() as f32;

        let r = canvas.width() as f32 / 2.0;
        let dx = x - r;
        let dy = y - r;

        let dist = (dx * dx + dy * dy).sqrt();
        if dist > r {
            return;
        }

        let s = dist / r;
        let h = dy.atan2(dx).to_degrees().rem_euclid(360.0);

        hue.set(h);
        sat.set(s);

        let (rr, gg, bb) = hsv_to_rgb(h, s, val.get());
        red.set(rr);
        green.set(gg);
        blue.set(bb);
        color.set(Color::new(rr, gg, bb, color.get().alpha));
    };

    let on_value = move |ev: web_sys::Event| {
        let v: f32 = event_target_value(&ev).parse().unwrap();
        val.set(v);
        let (rr, gg, bb) = hsv_to_rgb(hue.get(), sat.get(), val.get());
        red.set(rr);
        green.set(gg);
        blue.set(bb);
        color.set(Color::new(rr, gg, bb, color.get().alpha));
    };

    let on_alpha = move |ev: web_sys::Event| {
        let a: f32 = event_target_value(&ev).parse().unwrap();
        alpha.set(a.clamp(0.0, 1.0));
        let c = color.get();
        color.set(Color::new(c.r, c.g, c.b, a));
    };

    let on_red = move |ev: web_sys::Event| {
        let r: u8 = event_target_value(&ev).parse().unwrap();
        red.set(r.clamp(0, 255));
        let c = color.get();
        color.set(Color::new(r, c.g, c.b, c.alpha));
    };

    let on_green = move |ev: web_sys::Event| {
        let g: u8 = event_target_value(&ev).parse().unwrap();
        green.set(g.clamp(0, 255));
        let c = color.get();
        color.set(Color::new(c.r, green.get(), c.b, c.alpha));
    };

    let on_blue = move |ev: web_sys::Event| {
        let b: u8 = event_target_value(&ev).parse().unwrap();
        blue.set(b.clamp(0, 255));
        let c = color.get();
        color.set(Color::new(c.r, c.g, b, c.alpha));
    };

    view! {
    <div style="display:flex; gap:16px; align-items:flex-start;">
        <div style="display:flex; flex-direction:column; align-items:center; gap:8px;">

            <div
                style=move || {
                    let c = color.get();
                    format!(
                        "width:28px;\
                         height:28px;\
                         border:1px solid #333;\
                         background:rgba({},{},{},{:.3});",
                        c.r, c.g, c.b, c.alpha
                    )
                }
            />

            <div
                style="
                    display:flex;
                    flex-direction:row;
                    align-items:center;
                    gap:8px;
                    width:100%;
                "
            >

                <input
                    type="number"
                    min="0"
                    max="1"
                    step="0.01"
                    prop:value=move || format!("{:.2}", red.get())
                    style="width:50px;"
                    on:input=move |ev| {
                        let r: u8 = event_target_value(&ev)
                            .parse()
                            .unwrap_or(red.get());
                        red.set(r.clamp(0, 255));
                    }
                />

                <div>
                    "R"
                </div>

                <input
                    type="range"
                    min="0"
                    max="255"
                    step="1"
                    prop:value=red
                    style="width:60px; flex:1;"
                    on:input=on_red
                />
            </div>

            <div
                style="
                    display:flex;
                    flex-direction:row;
                    align-items:center;
                    gap:8px;
                    width:100%;
                "
            >

                <input
                    type="number"
                    min="0"
                    max="255"
                    step="1"
                    prop:value=move || format!("{:.2}", green.get())
                    style="width:50px;"
                    on:input=move |ev| {
                        let g: u8 = event_target_value(&ev)
                            .parse()
                            .unwrap_or(red.get());
                        green.set(g.clamp(0, 255));
                    }
                />

                <div>
                    "G"
                </div>

                <input
                    type="range"
                    min="0"
                    max="255"
                    step="1"
                    prop:value=green
                    style="width:60px; flex:1;"
                    on:input=on_green
                />
            </div>

            <div
                style="
                    display:flex;
                    flex-direction:row;
                    align-items:center;
                    gap:8px;
                    width:100%;
                "
            >

                <input
                    type="number"
                    min="0"
                    max="255"
                    step="1"
                    prop:value=move || format!("{:.2}", blue.get())
                    style="width:50px;"
                    on:input=move |ev| {
                        let b: u8 = event_target_value(&ev)
                            .parse()
                            .unwrap_or(blue.get());
                        blue.set(b.clamp(0, 255));
                    }
                />

                <div>
                    "B"
                </div>

                <input
                    type="range"
                    min="0"
                    max="255"
                    step="1"
                    prop:value=blue
                    style="width:60px; flex:1;"
                    on:input=on_blue
                />
            </div>

            <div
                style="
                    display:flex;
                    flex-direction:row;
                    align-items:center;
                    gap:8px;
                    width:100%;
                "
            >

                <input
                    type="number"
                    min="0"
                    max="1"
                    step="0.01"
                    prop:value=move || format!("{:.2}", alpha.get())
                    style="width:50px;"
                    on:input=move |ev| {
                        let a: f32 = event_target_value(&ev)
                            .parse()
                            .unwrap_or(alpha.get());
                        alpha.set(a.clamp(0.0, 1.0));
                    }
                />

                <div>
                    "A"
                </div>

                <input
                    type="range"
                    min="0"
                    max="1"
                    step="0.01"
                    prop:value=alpha
                    style="width:60px; flex:1;"
                    on:input=on_alpha
                />
            </div>

            

            <canvas
                node_ref=canvas_ref
                width=125
                height=125
                style="border-radius:50%; cursor:crosshair;"
                on:click=on_click
            />

            <div style="font-size:0.85rem; letter-spacing:0.06em; color:white; align-self:flex-start; padding:0 0 0 6px;">
                "Brightness"
            </div>

            <input
                type="range"
                min="0"
                max="1.0"
                step="0.01"
                prop:value=val
                style="writing-mode: bt-lr; height:10px;"
                on:input=on_value
            />
    
            
        </div>
        
    </div>
    }
}