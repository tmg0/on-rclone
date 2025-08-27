import { invoke } from '@tauri-apps/api/core'

interface RcloneNfsmountOptions extends Record<string, string> {
  remote: string
  remotePath: string
  mountpoint: string
}

export const rclone = {
  nfsmount: (options: RcloneNfsmountOptions) => invoke('plugin:rclone|nfsmount', options),
  stop: () => invoke('plugin:rclone|stop'),
}
