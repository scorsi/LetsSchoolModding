import { invoke } from '@tauri-apps/api/tauri'

export const game = {
    check: function(gamePath?: string): Promise<boolean> {
        return invoke('check_game', { gamePath });
    }
}

export const modLoader = {
    check: function(): Promise<"valid" | "outdated" | "not-installed"> {
        return invoke('check_modloader');
    },
    install: function(): Promise<boolean> {
        return invoke('install_modloader');
    },
    download: function(): Promise<boolean> {
        return invoke('download_modloader');
    },
    cleanDownload: function(): Promise<boolean> {
        return invoke('clean_download_modloader');
    }
}

export async function checkModLoader(): Promise<boolean> {
    return await invoke('check_modloader');
}

export async function installModLoader(): Promise<boolean> {
    return await invoke('install_modloader');
}

export async function downloadModLoader(): Promise<boolean> {
    return await invoke('download_modloader');
}

export async function cleanDownloadModLoader(): Promise<boolean> {
    return await invoke('clean_download_modloader');
}
