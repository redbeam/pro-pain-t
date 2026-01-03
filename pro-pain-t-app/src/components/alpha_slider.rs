use leptos::{prelude::*, *};

#[component]
pub fn AlphaSlider(
    color: RwSignal<(u8, u8, u8, f32)>,
) -> impl IntoView {

    // Update alpha
    let on_input = move |ev| {
        let (r, g, b, _) = color.get();

        let a: f32 = event_target_value(&ev)
            .parse()
            .unwrap_or(1.0f32)
            .clamp(0.0, 1.0) as f32;

        color.set((r, g, b, a));
    };

    let alpha = move || color.get().3;

    view! {
        <div style="display:flex; align-items:center; gap:8px; width:100%;">
            <input
                type="number"
                min="0"
                max="1"
                step="0.01"
                prop:value=move || format!("{:.2}", alpha())
                style="width:50px;"
                on:input=on_input.clone()
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