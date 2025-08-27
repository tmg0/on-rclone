use crate::{Error, RcloneState};
use tauri::{command, AppHandle, State};
use tokio::{runtime::Runtime, sync::RwLock};

#[command]
pub async fn nfsmount(
    app: AppHandle,
    state: State<'_, RwLock<RcloneState>>,
    remote: String,
    remote_path: String,
    mountpoint: String,
) -> Result<(), Error> {
    state
        .write()
        .await
        .nfsmount(&app, remote, remote_path, mountpoint)
        .await
}

#[command]
pub fn stop(state: State<'_, RwLock<RcloneState>>) -> Result<(), Error> {
    Runtime::new().unwrap().block_on(state.write()).stop()
}
