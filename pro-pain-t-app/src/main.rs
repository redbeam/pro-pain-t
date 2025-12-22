use leptos::prelude::*;

mod app;
mod components;

fn main() {
    leptos::mount::mount_to_body(|| view! { <app::App /> });
}
