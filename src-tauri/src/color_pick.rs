use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
use windows::Win32::Foundation::{COLORREF, POINT};
use windows::Win32::Graphics::Gdi::{GetDC, GetPixel, ReleaseDC};
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

#[tauri::command]
pub async fn start_color_pick(app: AppHandle) -> Result<(), String> {
    let monitors = app.available_monitors().map_err(|e| e.to_string())?;

    for (index, monitor) in monitors.into_iter().enumerate() {
        let pos = monitor.position();
        let size = monitor.size();
        let label = format!("picker-overlay-{}", index);

        if app.get_webview_window(&label).is_none() {
            let window = WebviewWindowBuilder::new(
                &app,
                &label,
                WebviewUrl::App("index.html#/overlay".into()),
            )
            .position(pos.x as f64, pos.y as f64)
            .inner_size(size.width as f64, size.height as f64)
            .transparent(true)
            .decorations(false)
            .always_on_top(true)
            .skip_taskbar(true)
            .shadow(false)
            .resizable(false)
            .focused(true)
            .build()
            .map_err(|e| e.to_string())?;

            // Forcer le focus pour que la touche Échap fonctionne immédiatement
            let _ = window.set_focus();
        }
    }
    Ok(())
}

#[tauri::command]
pub fn read_pixel_at() -> Result<String, String> {
    unsafe {
        let mut point = POINT { x: 0, y: 0 };
        GetCursorPos(&mut point).map_err(|e| e.to_string())?;

        let hdc = GetDC(None);
        let color: COLORREF = GetPixel(hdc, point.x, point.y);
        ReleaseDC(None, hdc);

        let r = (color.0 & 0xFF) as u8;
        let g = ((color.0 >> 8) & 0xFF) as u8;
        let b = ((color.0 >> 16) & 0xFF) as u8;

        Ok(format!("{:02X}{:02X}{:02X}", r, g, b))
    }
}

#[tauri::command]
pub fn confirm_color_pick(app: AppHandle, hex: String) -> Result<(), String> {
    // Émettre la couleur choisie à la fenêtre principale
    let _ = app.emit("color-selected", hex);

    // Fermer tous les overlays
    for (label, window) in app.webview_windows() {
        if label.starts_with("picker-overlay-") {
            let _ = window.close();
        }
    }

    // Remettre le focus sur la fenêtre principale
    if let Some(main) = app.get_webview_window("main") {
        let _ = main.set_focus();
    }

    Ok(())
}

#[tauri::command]
pub fn cancel_color_pick(app: AppHandle) -> Result<(), String> {
    for (label, window) in app.webview_windows() {
        if label.starts_with("picker-overlay-") {
            let _ = window.close();
        }
    }
    if let Some(main) = app.get_webview_window("main") {
        let _ = main.set_focus();
    }
    Ok(())
}
