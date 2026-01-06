use leptos::prelude::*;

#[component]
pub fn CanvasSizeWindow<F>(
    is_open: RwSignal<bool>,
    canvas_width: RwSignal<u32>,
    canvas_height: RwSignal<u32>,
    on_confirm: F,
) -> impl IntoView
where
    F: Fn(u32, u32) + 'static + Clone + Send + Sync,
{
    let (local_width, set_local_width) = signal(canvas_width.get());
    let (local_height, set_local_height) = signal(canvas_height.get());

    Effect::new(move |_| {
        if is_open.get() {
            set_local_width.set(canvas_width.get());
            set_local_height.set(canvas_height.get());
        }
    });

    let on_width_input = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        if let Ok(parsed) = value.parse::<u32>() {
            set_local_width.set(parsed);
        }
    };

    let on_height_input = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        if let Ok(parsed) = value.parse::<u32>() {
            set_local_height.set(parsed);
        }
    };

    view! {
        <div
            style=move || format!(
                "position:fixed; inset:0; background:rgba(0,0,0,0.4); display:{}; align-items:center; justify-content:center; z-index:1000;",
                if is_open.get() { "flex" } else { "none" }
            )
        >
            <div
                style="
                    background:#2b2b2b;
                    padding:1rem 1.25rem;
                    border-radius:4px;
                    color:#f5f5f5;
                    min-width:260px;
                    font-family:system-ui, sans-serif;
                    box-shadow:0 12px 30px rgba(0,0,0,0.7);
                "
            >
                <h2 style="margin:0 0 0.75rem 0; font-size:0.95rem;">"Canvas Size"</h2>
                <table style="width:100%; font-size:0.8rem;">
                            <tr>
                                <td style="padding:0.15rem 0.5rem 0.15rem 0;">"Width (px)"</td>
                                <td style="padding:0.15rem 0;">
                                    <input
                                        type="number"
                                        min="1"
                                        prop:value=move || local_width.get().to_string()
                                        on:input=on_width_input
                                        style="width:100%; box-sizing:border-box;"
                                    />
                                </td>
                            </tr>
                            <tr>
                                <td style="padding:0.15rem 0.5rem 0.15rem 0;">"Height (px)"</td>
                                <td style="padding:0.15rem 0;">
                                    <input
                                        type="number"
                                        min="1"
                                        prop:value=move || local_height.get().to_string()
                                        on:input=on_height_input
                                        style="width:100%; box-sizing:border-box;"
                                    />
                                </td>
                            </tr>
                </table>
                <div
                            style="
                                margin-top:0.75rem;
                                display:flex;
                                justify-content:flex-end;
                                gap:0.5rem;
                            "
                        >
                    <button
                        on:click=move |_| {
                            is_open.set(false);
                        }
                                style="
                                    padding:0.25rem 0.6rem;
                                    border-radius:2px;
                                    border:none;
                                    background:#3a3a3a;
                                    color:#f5f5f5;
                                    font-size:0.8rem;
                            "
                        >"Cancel"</button>
                    <button
                        on:click=move |_| {
                            let w = local_width.get();
                            let h = local_height.get();
                            on_confirm(w, h);
                            is_open.set(false);
                        }
                                style="
                                    padding:0.25rem 0.75rem;
                                    border-radius:2px;
                                    border:none;
                                    background:#4a7cff;
                                    color:#f5f5f5;
                                    font-size:0.8rem;
                            "
                        >"OK"</button>
                </div>
            </div>
        </div>
    }
}
