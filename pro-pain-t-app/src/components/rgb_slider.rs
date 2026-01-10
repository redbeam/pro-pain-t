use crate::components::color_picker::Channel;
use leptos::prelude::*;
use pro_pain_t_app::structs::color::Color;

#[component]
pub fn RGBSlider(channel: Channel, color: RwSignal<Color>) -> impl IntoView {
    let on_input = move |ev| {
        let c = color.get();

        match channel {
            Channel::R => {
                let red: u8 = event_target_value(&ev)
                    .parse::<u8>()
                    .unwrap_or(color.get().r)
                    .clamp(0, 255) as u8;

                color.set(Color::new(red, c.g, c.b, c.alpha));
            }

            Channel::G => {
                let green: u8 = event_target_value(&ev)
                    .parse::<u8>()
                    .unwrap_or(color.get().g)
                    .clamp(0, 255) as u8;

                color.set(Color::new(c.r, green, c.b, c.alpha));
            }

            Channel::B => {
                let blue: u8 = event_target_value(&ev)
                    .parse::<u8>()
                    .unwrap_or(color.get().b)
                    .clamp(0, 255) as u8;

                color.set(Color::new(c.r, c.g, blue, c.alpha));
            }
        }
    };

    let ch = move || match channel {
        Channel::R => color.get().r,
        Channel::G => color.get().g,
        Channel::B => color.get().b,
    };

    let label = match channel {
        Channel::R => "R",
        Channel::G => "G",
        Channel::B => "B",
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
                on:input=on_input
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
