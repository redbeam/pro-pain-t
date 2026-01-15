use tauri::menu::{MenuBuilder, SubmenuBuilder};
use tauri::{App, Emitter};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

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

    app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {
        match event.id().0.as_str() {
            // ===== File =====
            "new_project" => {
                let answer = app_handle.dialog()
                    .message("This will overwrite the currently opened project. Do you want to continue?")
                    .title("Warning")
                    .buttons(MessageDialogButtons::OkCancelCustom("Yes".to_string(), "No".to_string()))
                    .blocking_show();
                if !answer {
                    return;
                }
                app_handle
                    .emit("menu-new-project", ())
                    .expect("Failed to emit menu-new-project");
                println!("emitted new_project");
            }

            "open_project" => {
                let answer = app_handle.dialog()
                    .message("This will overwrite the currently opened project. Do you want to continue?")
                    .title("Warning")
                    .buttons(MessageDialogButtons::OkCancelCustom("Yes".to_string(), "No".to_string()))
                    .blocking_show();
                if !answer {
                    return;
                }

                let handle = app_handle.clone();
                app_handle.dialog().file()
                    .add_filter("ProPainTProject", &["ppp"])
                    .pick_file(move |file_path| {
                    if let Some(path) = file_path {
                        handle
                            .emit("menu-open-project", path.to_string())
                            .expect("Failed to emit menu-open-project");
                        println!("emitted open_project");
                    }
                });
            }

            "save_project" => {
                let handle = app_handle.clone();
                app_handle.dialog().file()
                    .add_filter("ProPainTProject", &["ppp"])
                    .set_file_name("unnamed.ppp")
                    .set_can_create_directories(true)
                    .save_file(move |file_path| {
                        if let Some(path) = file_path {
                            handle
                                .emit("menu-save-project", path.to_string())
                                .expect("Failed to emit menu-save-project");
                            println!("emitted save_project");
                        }
                    });
            }

            "import_as_layer" => {
                app_handle
                    .emit("menu-import-as-layer", ())
                    .expect("Failed to emit menu-import-as-layer");
                println!("emitted import_as_layer");
            }

            "export_project" => {
                app_handle
                    .emit("menu-export-project", ())
                    .expect("Failed to emit menu-export-project");
                println!("emitted export_project");
            }

            "quit" => {
                app_handle.exit(0);
            }

            // ===== Edit =====
            "undo" => {
                app_handle
                    .emit("menu-undo", ())
                    .expect("Failed to emit menu-undo");
                println!("emitted undo");
            }

            "redo" => {
                app_handle
                    .emit("menu-redo", ())
                    .expect("Failed to emit menu-redo");
                println!("emitted redo");
            }

            "canvas_size" => {
                app_handle
                    .emit("menu-canvas-size", ())
                    .expect("Failed to emit menu-canvas-size");
                println!("emitted canvas_size");
            }

            _ => {
                println!("Unhandled menu event: {:?}", event.id());
            }
        }
    });

    Ok(())
}
