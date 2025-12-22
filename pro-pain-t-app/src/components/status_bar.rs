use leptos::prelude::*;

#[component]
pub fn StatusBar() -> impl IntoView {
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
            <div style="display:flex; gap:1.5rem;">
                <span>"933×627 px"</span>
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
