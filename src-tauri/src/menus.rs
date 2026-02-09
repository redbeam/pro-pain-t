use crate::events::handlers::{error_dialog, export_project_handler, import_as_layer_handler, open_file_handler, project_overwrite_confirmation, save_project_handler};
use pro_pain_t_shared::events::events::{EVENT_MENU_CANVAS_SIZE, EVENT_MENU_NEW_PROJECT, EVENT_MENU_REDO, EVENT_MENU_UNDO};
use tauri::menu::{MenuBuilder, SubmenuBuilder};
use tauri::{App, AppHandle, Emitter};
use tauri_plugin_dialog::DialogExt;

pub fn setup_menus(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let dummy_menu = SubmenuBuilder::new(app, "Pro PainT").build()?;

    let file_menu = SubmenuBuilder::new(app, "File")
        .text("new_project", "New project")
        .text("open_project", "Open project")
        .text("save_project", "Save project")
        .separator()
        .text("import_as_layer", "Import image as Layer")
        .separator()
        .text("export_project", "Export project")
        .separator()
        .text("quit", "Quit")
        .build()?;

    let edit_menu = SubmenuBuilder::new(app, "Edit")
        .text("undo", "Undo")
        .text("redo", "Redo")
        .separator()
        .text("canvas_size", "Canvas size...")
        .build()?;

    let menu = MenuBuilder::new(app)
        .items(&[&dummy_menu, &file_menu, &edit_menu])
        .build()?;

    app.set_menu(menu)?;

    app.on_menu_event(move |handle: &AppHandle, event| {
        let app_handle = handle.clone();
        match event.id().0.as_str() {
            // ===== File =====
            "new_project" => {
                if !project_overwrite_confirmation(&app_handle) {
                    return;
                }
                if app_handle.emit(EVENT_MENU_NEW_PROJECT, ()).is_err() {
                    error_dialog(&app_handle, "Failed to emit menubar action");
                    return;
                }
                println!("emitted new_project");
            }

            "open_project" => {
                if !project_overwrite_confirmation(&app_handle) {
                    return;
                }
                app_handle.dialog().file()
                    .add_filter("ProPainTProject", &["ppp"])
                    .pick_file(move |file_path| open_file_handler(&app_handle, file_path));
            }

            "save_project" => {
                app_handle.dialog().file()
                    .add_filter("ProPainTProject", &["ppp"])
                    .set_file_name("unnamed.ppp")
                    .set_can_create_directories(true)
                    .save_file(move |file_path| save_project_handler(&app_handle, file_path));
            }

            "import_as_layer" => {
                app_handle.dialog().file()
                    .add_filter("PNG or JPG images", &["png", "jpg", "jpeg"])
                    .pick_file(move |file_path| import_as_layer_handler(&app_handle, file_path));
            }

            "export_project" => {
                app_handle.dialog().file()
                    .add_filter("PNG or JPG images", &["png", "jpg", "jpeg"])
                    .set_file_name("unnamed_export")
                    .set_can_create_directories(true)
                    .save_file(move |file_path| export_project_handler(&app_handle, file_path));
            }

            "quit" => {
                app_handle.exit(0);
            }

            // ===== Edit =====
            "undo" => {
                if app_handle.emit(EVENT_MENU_UNDO, ()).is_err() {
                    error_dialog(&app_handle, "Failed to emit menubar action");
                    return;
                }
                println!("emitted undo");
            }

            "redo" => {
                if app_handle.emit(EVENT_MENU_REDO, ()).is_err() {
                    error_dialog(&app_handle, "Failed to emit menubar action");
                    return;
                }
                println!("emitted redo");
            }

            "canvas_size" => {
                if app_handle.emit(EVENT_MENU_CANVAS_SIZE, ()).is_err() {
                    error_dialog(&app_handle, "Failed to emit menubar action");
                    return;
                }
                println!("emitted canvas_size");
            }

            _ => {
                println!("Unhandled menu event: {:?}", event.id());
            }
        }
    });

    Ok(())
}
