use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

/// Creates a system tray menu with options `show`, `hide`, and `quit`
pub(crate) fn create_tray_menu() -> SystemTrayMenu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
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
    match event {
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
                window.show().expect("Couldn't show the main window");
            }
            _ => {}
        },
        _ => {}
    }
}
