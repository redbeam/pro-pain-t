use leptos::prelude::*;

#[component]
pub fn BrushSizeSlider(brush_size: RwSignal<f32>) -> impl IntoView {
    let on_input = move |ev| {
        let value = event_target_value(&ev)
            .parse()
            .unwrap_or(1.0f32)
            .clamp(0.1, 3.0);

        brush_size.set(value);
    };

    view! {
        <div style="width:100%; text-align:center; font-size:0.85rem; letter-spacing:0.06em; color:white;">
            "Brush size"
        </div>
        <div style="display:flex; align-items:center; gap:8px; width:100%;">
            <input
                type="number"
                min="0.1"
                max="3"
                step="0.01"
                prop:value=move || format!("{:.2}", brush_size.get())
                style="width:50px;"
                on:input=on_input
            />
            <input
                type="range"
                min="0.1"
                max="3"
                step="0.01"
                prop:value=brush_size.get()
                style="width:60px; flex:1;"
                on:input=on_input
            />
        </div>
    }
}
