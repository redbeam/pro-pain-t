use leptos::prelude::*;

use crate::components::color_picker::ColorPicker;

#[component]
pub fn ToolPalette() -> impl IntoView {

    let current_color = RwSignal::new((255, 255, 255, 255));

    view! {
        <nav
            style="
                width:140px;
                background:#262626;
                color:#f5f5f5;
                display:flex;
                flex-direction:column;
                padding:0.5rem;
                gap:0.75rem;
                box-sizing:border-box;
                font-size:0.75rem;
                font-family:system-ui, sans-serif;
            "
        >
            <div
                style="
                    display:grid;
                    grid-template-columns:repeat(4, 1fr);
                    gap:0.25rem;
                "
            >
                { (0..12).map(|_| view! { <div style="width:24px; height:24px; background:#3a3a3a; border-radius:2px;"></div> }).collect_view() }
            </div>

            <ColorPicker color=current_color />
        </nav>
    }
}
