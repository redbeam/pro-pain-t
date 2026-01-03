use leptos::{prelude::*, *};
use crate::components::color_picker::Channel;

#[component]
pub fn RGBSlider(
    channel: Channel,
    color: RwSignal<(u8, u8, u8, f32)>,
) -> impl IntoView {

    let on_input = move |ev| {
        let (r, g, b, a) = color.get();

        match channel {
            Channel::R => {
                let red: u8 = event_target_value(&ev)
                    .parse::<u8>()
                    .unwrap_or(color.get().0)
                    .clamp(0, 255) as u8;

                color.set((red, g, b, a));
            }

            Channel::G => {
                let green: u8 = event_target_value(&ev)
                    .parse::<u8>()
                    .unwrap_or(color.get().1)
                    .clamp(0, 255) as u8;

                color.set((r, green, b, a));
            }

            Channel::B => {
                let blue: u8 = event_target_value(&ev)
                    .parse::<u8>()
                    .unwrap_or(color.get().2)
                    .clamp(0, 255) as u8;

                color.set((r, g, blue, a));
            }

            _ => {}
        }
    };


    let ch = move || match channel {
        Channel::R => color.get().0,
        Channel::G => color.get().1,
        Channel::B => color.get().2,
        _ => 0,
    };

    let label = match channel {
        Channel::R => "R",
        Channel::G => "G",
        Channel::B => "B",
        _ => "",
    };
        
            view! {
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
                        prop:value=move || ch
                        style="width:50px;"
                        on:input=on_input.clone()
                    />

                    <div style="width:12px; text-align:center;">
                        {label}
                    </div>

                    <input
                        type="range"
                        min="0"
                        max="255"
                        step="1"
                        prop:value=ch
                        style="width:60px; flex:1;"
                        on:input=on_input
                    />
                </div>
            }
}
