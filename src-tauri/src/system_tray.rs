use log::error;
use tauri::{
    AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, WindowEvent,
};
use tauri_plugin_positioner::{Position, WindowExt};

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

/// A system tray event handler that executes system tray events, e.g., quits the process
pub(crate) fn system_tray_event_handler(app: &AppHandle, event: SystemTrayEvent) {
    match &event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
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
                    error!("Couldn't center the main window: {}", err);
                }

                window.show().expect("Couldn't show the main window");
            }
            _ => {}
        },
        _ => {}
    }
    tauri_plugin_positioner::on_tray_event(app, &event);
}

/// Handles window events, such as clicks outside the window
pub(crate) fn window_event_handler(event: GlobalWindowEvent) {
    match event.event() {
        WindowEvent::Focused(is_focused) => {
            if !is_focused {
                let _ = event.window().hide();
            }
        }
        _ => {}
    }
}
