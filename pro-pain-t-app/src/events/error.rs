use leptos::task::spawn_local;
use tauri_sys::core::invoke;
use pro_pain_t_shared::dtos::error_message::ErrorMessage;

pub fn show_error_dialog(message: String) {
    spawn_local(async move {
        invoke::<()>("error_dialog_command", ErrorMessage::new(message)).await;
    });
}
