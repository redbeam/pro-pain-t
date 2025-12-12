use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <h1>{"Pro PainT"}</h1>
            <p>{"Leptos + Tauri skeleton"}</p>
        </main>
    }
}

fn main() {
    leptos::mount::mount_to_body(|| view! { <App /> });
}
