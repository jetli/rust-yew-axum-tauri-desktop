const invoke = window.__TAURI_INVOKE__
// const invoke = window.__TAURI__.invoke

export async function getPort() {
    return await invoke("get_port", {});
}