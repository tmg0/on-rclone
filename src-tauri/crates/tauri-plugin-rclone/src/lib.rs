use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, RunEvent,
};

use tokio::sync::RwLock;

mod commands;
mod error;
mod rclone;
mod fse;

pub use error::{Error, Result};
pub use rclone::RcloneState;

/// Initializes the plugin.
pub fn init() -> TauriPlugin<tauri::Wry> {
    Builder::new("rclone")
        .invoke_handler(tauri::generate_handler![commands::nfsmount, commands::stop,])
        .setup(|app, _| {
            app.manage(RwLock::new(RcloneState::new()));
            Ok(())
        })
        .on_event(|app, event| match event {
            RunEvent::Exit => {
                let state = app.state::<RwLock<RcloneState>>();
                let _ = commands::stop(state);
            }
            _ => {}
        })
        .build()
}
