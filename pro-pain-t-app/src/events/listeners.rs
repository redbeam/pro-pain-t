use crate::structs::project::Project;
use futures::StreamExt;
use leptos::prelude::RwSignal;

use crate::render::canvas_renderer::composite_layers;
use crate::structs::color::Color;
use crate::structs::layer::Layer;
use image::RgbImage;
use leptos::prelude::{Get, Set, Update};
use leptos::task::spawn_local;
use pro_pain_t_shared::dtos::export_object::ExportObjectDto;
use pro_pain_t_shared::dtos::image::ImageDto;
use pro_pain_t_shared::dtos::path::PathDto;
use pro_pain_t_shared::dtos::project::ProjectDto;
use pro_pain_t_shared::dtos::save_object::SaveObjectDto;
use pro_pain_t_shared::events::events::{
    EVENT_MENU_CANVAS_SIZE, EVENT_MENU_EXPORT_PROJECT, EVENT_MENU_IMPORT_AS_LAYER,
    EVENT_MENU_NEW_PROJECT, EVENT_MENU_OPEN_PROJECT, EVENT_MENU_SAVE_PROJECT,
};
use tauri_sys::core::invoke;
use tauri_sys::event::listen;
use crate::events::error::show_error_dialog;

pub fn create_new_project_listener(new_project_window_signal: RwSignal<bool>) {
    spawn_local(async move {
        let mut listener = listen::<()>(EVENT_MENU_NEW_PROJECT).await.unwrap();
        while let Some(_) = listener.next().await {
            new_project_window_signal.set(true);
        }
    });
}

pub fn open_project_listener(project: RwSignal<Project>) {
    spawn_local(async move {
        let mut listener = listen::<ProjectDto>(EVENT_MENU_OPEN_PROJECT).await.unwrap();
        while let Some(data) = listener.next().await {
            let project_loaded = Project::from_file_data(data.payload.data);
            project.update(|project| {
                project.replace_project_with(project_loaded);
            });
        }
    });
}

pub fn save_project_listener(project: RwSignal<Project>) {
    spawn_local(async move {
        let mut listener = listen::<PathDto>(EVENT_MENU_SAVE_PROJECT).await.unwrap();
        while let Some(data) = listener.next().await {
            let project_serialized = project.get().serialize();
            invoke::<()>(
                "save_project_command",
                SaveObjectDto::new(data.payload.path, project_serialized),
            )
            .await;
        }
    });
}

pub fn import_as_layer_listener(project: RwSignal<Project>) {
    spawn_local(async move {
        let mut listener = listen::<ImageDto>(EVENT_MENU_IMPORT_AS_LAYER)
            .await
            .unwrap();
        while let Some(data) = listener.next().await {
            project.update(|project| {
                let image = RgbImage::from_raw(
                    data.payload.width,
                    data.payload.height,
                    data.payload.raw_data,
                );
                if image.is_none() {
                    show_error_dialog("Unable to create image buffer".to_string());
                    return;
                }
                let layer_id = project.next_layer_id.get();
                let new_layer =
                    Layer::from_image(&image.unwrap(), layer_id, "Imported image", Color::default_black());
                project.add_new_layer(new_layer);
            });
        }
    });
}

pub fn export_project_listener(project: RwSignal<Project>) {
    spawn_local(async move {
        let mut listener = listen::<PathDto>(EVENT_MENU_EXPORT_PROJECT).await.unwrap();
        while let Some(data) = listener.next().await {
            let layers = project.get().layers.get();
            if layers.is_empty() {
                continue;
            }

            let raw_image = composite_layers(&layers).0;
            let image_dto = ImageDto::new(
                project.get().width.get(),
                project.get().height.get(),
                raw_image,
            );
            invoke::<()>(
                "export_project_command",
                ExportObjectDto::new(data.payload.path, image_dto),
            )
            .await;
        }
    });
}

pub fn canvas_size_listener(canvas_size_window_signal: RwSignal<bool>) {
    spawn_local(async move {
        let mut listener = listen::<()>(EVENT_MENU_CANVAS_SIZE).await.unwrap();
        while let Some(_) = listener.next().await {
            canvas_size_window_signal.set(true);
        }
    });
}
