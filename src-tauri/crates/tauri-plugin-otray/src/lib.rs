use tauri::{
    include_image, menu::{Menu, MenuItem}, image::{Image}, plugin::{Builder, TauriPlugin}, Manager, RunEvent, Runtime
};

use tauri::tray::TrayIconBuilder;

mod error;

pub use error::{Error, Result};

const DEFAULT_TRAY_ICON: Image<'_> = include_image!("./icons/rclone.png");

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("otray")
        .on_event(|app, event| match event {
            tauri::RunEvent::WindowEvent {
                label,
                event: window_event,
                ..
            } => match window_event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    let window = app.get_webview_window(label.as_str()).unwrap();
                    window.hide().unwrap();

                    #[cfg(target_os = "macos")]
                    app.set_activation_policy(tauri::ActivationPolicy::Accessory)
                        .unwrap();

                    api.prevent_close();
                }
                _ => {}
            },

            RunEvent::Ready => {
                let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>).unwrap();
                let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
                let menu = Menu::with_items(app, &[&show_i, &quit_i]).unwrap();

                let _ = TrayIconBuilder::with_id("__ON_RCLONE:TRAY")
                    .icon(DEFAULT_TRAY_ICON)
                    .icon_as_template(true)
                    .menu(&menu)
                    .on_menu_event(move |app, event| match event.id.as_ref() {
                        "show" => {
                            let window = app.get_webview_window("main").expect("no main window");
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        },
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    })
                    .build(app)
                    .unwrap();
            }

            _ => {}
        })
        .build()
}
