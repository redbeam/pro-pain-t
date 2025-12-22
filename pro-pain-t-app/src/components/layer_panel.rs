use leptos::prelude::*;

#[component]
pub fn LayerPanel() -> impl IntoView {
    view! {
        <aside
            style="
                width:210px;
                min-width:200px;
                max-width:240px;
                background:#2a2a2a;
                color:#f5f5f5;
                padding:0.5rem 0.5rem 0.75rem 0.5rem;
                box-sizing:border-box;
                font-family:system-ui, sans-serif;
                display:flex;
                flex-direction:column;
                gap:0.5rem;
            "
        >
            <h2 style="font-size:0.85rem; margin:0 0 0.25rem 0; text-transform:uppercase; letter-spacing:0.06em;">
                "Layers"
            </h2>
            <div
                style="
                    flex:1;
                    border-radius:2px;
                    background:#1f1f1f;
                    padding:0.35rem;
                    box-sizing:border-box;
                    display:flex;
                    flex-direction:column;
                    gap:0.4rem;
                    font-size:0.8rem;
                "
            >
                {[
                    "Layer 1",
                    "Layer 2",
                    "Layer 3",
                ]
                    .into_iter()
                    .map(|name| {
                        view! {
                            <div
                                style="
                                    display:flex;
                                    align-items:center;
                                    gap:0.35rem;
                                    background:#2c2c2c;
                                    padding:0.25rem 0.3rem;
                                    border-radius:2px;
                                "
                            >
                                <div
                                    style="
                                        display:flex;
                                        flex-direction:column;
                                        gap:0.75rem;
                                        font-size:0.7rem;
                                        align-items:center;
                                        color:#d0d0d0;
                                    "
                                >
                                    <span>"👁"</span>
                                    <span>"🔒"</span>
                                    <span>"🗑"</span>
                                </div>
                                <div
                                    style="
                                        flex:1;
                                        display:flex;
                                        flex-direction:column;
                                        gap:0.15rem;
                                    "
                                >
                                    <div
                                        style="
                                            height:50px;
                                            background:#d8d8d8;
                                            border-radius:2px;
                                        "
                                    ></div>
                                    <span style="font-size:0.8rem;">{name}</span>
                                </div>
                                <div
                                    style="
                                        display:flex;
                                        flex-direction:column;
                                        gap:0.15rem;
                                        font-size:0.7rem;
                                        color:#d0d0d0;
                                    "
                                >
                                    <span>"▲"</span>
                                    <span>"▼"</span>
                                </div>
                            </div>
                        }
                    })
                    .collect_view()
                }
            </div>
            <button
                style="
                    margin-top:0.25rem;
                    padding:0.25rem 0.5rem;
                    border-radius:2px;
                    border:none;
                    background:#3a3a3a;
                    color:#f5f5f5;
                    font-size:0.8rem;
                    text-align:left;
                "
            >"+ Add layer"</button>
        </aside>
    }
}
