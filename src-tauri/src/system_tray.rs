use log::error;
use tauri::{
    AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, WindowEvent,
};
use tauri_plugin_positioner::{Position, WindowExt};

use crate::state::State;

/// Creates a system tray menu with options `show`, `hide`, and `quit`
pub(crate) fn create_tray_menu() -> SystemTrayMenu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit)
}

/// A system tray event handler that executes system tray events,
/// e.g., quits the process
pub(crate) fn system_tray_event_handler(app: &AppHandle, event: SystemTrayEvent) {
    if let SystemTrayEvent::MenuItemClick { id, .. } = &event {
        match id.as_str() {
            "quit" => {
                let state: tauri::State<State> = app.state();
                // TODO: a more thorough investigation of why tauri doesn't drop
                // its managed state is needed
                if let Err(err) = state.get_process_manager().terminate_all_processes() {
                    error!("Couldn't terminate all processes: {:?}", err);
                }
                app.exit(0);
            }
            "hide" => {
                let window = app
                    .get_window("main")
                    .expect("Couldn't not get the main window.");
                window.hide().expect("Couldn't hide the main window");
            }
            "show" => {
                let window = app
                    .get_window("main")
                    .expect("Couldn't not get the main window.");
                if let Err(err) = window.move_window(Position::TopRight) {
                    error!("Couldn't center the main window: {:?}", err);
                }

                window.show().expect("Couldn't show the main window");
            }
            _ => {}
        }
    };
    tauri_plugin_positioner::on_tray_event(app, &event);
}

/// Handles window events, such as clicks outside the window
pub(crate) fn window_event_handler(event: GlobalWindowEvent) {
    if let WindowEvent::Focused(_is_focused) = event.event() {
        // TODO: it would be nice to "auto-hide" the window on focus loss,
        // but this also hides the window when interacting with file picker
        // let _ = event.window().hide();
    }
}
