use leptos::prelude::*;

#[component]
pub fn StatusBar(
    canvas_width: RwSignal<u32>,
    canvas_height: RwSignal<u32>,
    on_open_canvas_size: impl Fn() + 'static + Clone,
) -> impl IntoView {
    let on_open = move |_| {
        on_open_canvas_size();
    };

    view! {
        <footer
            style="
                height:24px;
                background:#181818;
                color:#c0c0c0;
                font-size:0.75rem;
                font-family:system-ui, sans-serif;
                display:flex;
                align-items:center;
                justify-content:space-between;
                padding:0 0.75rem;
                box-sizing:border-box;
            "
        >
            <div style="display:flex; gap:1.5rem; align-items:center;">
                <button
                    on:click=on_open
                    style="
                        border:none;
                        background:transparent;
                        color:#c0c0c0;
                        padding:0;
                        cursor:pointer;
                        font-size:0.75rem;
                    "
                    title="Change canvas size"
                >
                    {move || format!("{}×{} px", canvas_width.get(), canvas_height.get())}
                </button>
                <span>"x = 412, y = 238"</span>
            </div>
            <div style="display:flex; align-items:center; gap:0.35rem;">
                <button style="border:none; background:transparent; color:#c0c0c0;">"-"</button>
                <span>"100%"</span>
                <button style="border:none; background:transparent; color:#c0c0c0;">"+"</button>
            </div>
        </footer>
    }
}
