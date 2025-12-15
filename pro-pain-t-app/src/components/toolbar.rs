use leptos::prelude::*;

#[component]
pub fn Toolbar() -> impl IntoView {
    view! {
        <header
            style="
                display:flex;
                align-items:center;
                justify-content:space-between;
                height:28px;
                padding:0 0.75rem;
                background:#2b2b2b;
                color:#f5f5f5;
                font-family:system-ui, sans-serif;
                font-size:0.8rem;
                box-shadow:0 1px 0 #1a1a1a;
            "
        >
            <nav style="display:flex; gap:0.5rem;">
                <span style="cursor:pointer;">"File"</span>
                <span style="opacity:0.4;">"|"</span>
                <span style="cursor:pointer;">"Layer"</span>
                <span style="opacity:0.4;">"|"</span>
                <span style="cursor:pointer;">"Undo"</span>
                <span style="opacity:0.4;">"|"</span>
                <span style="cursor:pointer;">"Redo"</span>
            </nav>
        </header>
    }
}
