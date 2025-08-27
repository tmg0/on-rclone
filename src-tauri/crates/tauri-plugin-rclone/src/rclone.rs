use std::collections::HashMap;

use crate::{Result, fse};
use tauri::{AppHandle, Runtime};
use tauri_plugin_shell::process::CommandChild;
use tauri_plugin_shell::ShellExt;

use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

pub struct RcloneState {
    process: HashMap<String, Option<CommandChild>>,
}

impl RcloneState {
    pub(crate) fn new() -> Self {
        RcloneState {
            process: HashMap::new(),
        }
    }

    async fn run_rclone_command<R: Runtime>(
        &mut self,
        app: &AppHandle<R>,
        remote: &str,
        command: &str,
        args: Vec<&str>,
    ) -> Result<()> {
        let key = format!("{}:{}", remote, command);

        if self.process.get(&key).is_none() {
            let mut params = vec![command];
            params.extend(args);
            let cmd = app.shell().sidecar("rclone").unwrap();
            let (_, child) = cmd.args(params).spawn().unwrap();
            self.process.insert(key, Some(child));
        }

        Ok(())
    }

    pub(crate) async fn nfsmount<R: Runtime>(
        &mut self,
        app: &AppHandle<R>,
        remote: String,
        remote_path: String,
        mountpoint: String,
    ) -> Result<()> {
        let remote_path = format!("{}:{}", &remote, &remote_path);
        fse::ensure_dir(&mountpoint).await?;

        self.run_rclone_command(
            app,
            &remote,
            "nfsmount",
            vec![
                &remote_path,
                &mountpoint,
                "--vfs-cache-mode",
                "full"
            ],
        ).await?;

        Ok(())
    }

    pub(crate) fn stop(&mut self) -> Result<()> {
        for cmd in self.process.values_mut() {
            if let Some(value) = cmd {
                let pid = value.pid();
                kill(Pid::from_raw(pid as i32), Signal::SIGINT).unwrap();
            }
        }

        self.process.clear();
        Ok(())
    }
}
