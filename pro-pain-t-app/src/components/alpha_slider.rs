use leptos::{prelude::*};
use pro_pain_t_app::structs::color::Color;

#[component]
pub fn AlphaSlider(
    color: RwSignal<Color>,
) -> impl IntoView {

    let on_input = move |ev| {
        let c = color.get();

        let a: f32 = event_target_value(&ev)
            .parse()
            .unwrap_or(1.0f32)
            .clamp(0.0, 1.0) as f32;

        color.set(Color::new(c.r, c.g, c.b, a));
    };

    let alpha = move || color.get().alpha;

    view! {
        <div style="display:flex; align-items:center; gap:8px; width:100%;">
            <input
                type="number"
                min="0"
                max="1"
                step="0.01"
                prop:value=move || format!("{:.2}", alpha())
                style="width:50px;"
                on:input=on_input
            />

            <div style="width:12px; text-align:center;">
                "A"
            </div>

            <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                prop:value=alpha
                style="width:60px; flex:1;"
                on:input=on_input
            />
        </div>
    }
}