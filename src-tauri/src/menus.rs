use tauri::App;
use tauri::menu::{MenuBuilder, SubmenuBuilder};

pub fn setup_menus(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let dummy_menu = SubmenuBuilder::new(app, "Pro PainT")
        .build()?;

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
        .text("canvas_size", "Canvas Size...")
        .build()?;

    let menu = MenuBuilder::new(app)
        .items(&[&dummy_menu, &file_menu, &edit_menu])
        .build()?;

    app.set_menu(menu)?;

    Ok(())
}
