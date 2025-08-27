<script setup lang="ts">
import { homeDir } from '@tauri-apps/api/path'
import { join } from 'pathe'
import { rclone } from './src/utils/rclone'

async function execute() {
  const home = await homeDir()
  const mountpoint = join(home, 'OneDrive')
  rclone.nfsmount({ remote: 'OneDrive', remotePath: '/', mountpoint })
}

execute()
</script>

<template>
  <div class="flex flex-col items-center justify-center select-none">
    <div class="mt-6 mb-4">
      <img src="/rclone.svg" class="w-16 cursor-pointer">
    </div>

    <div class="font-semibold text-zinc-600">
      OnRclone
    </div>

    <div class="text-xs text-zinc-400">
      version 0.1.0
    </div>
  </div>
</template>
