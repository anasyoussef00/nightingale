use tauri::api::dialog;
use tauri::{CustomMenuItem, Menu, Submenu};

pub fn build_menu() -> Menu {
    let open = CustomMenuItem::new("open-single-file".to_string(), "Open");
    let open_folder = CustomMenuItem::new("open-folder".to_string(), "Open Folder");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let submenu = Submenu::new(
        "File",
        Menu::new()
            .add_item(open)
            .add_item(open_folder)
            .add_item(quit),
    );

    Menu::new().add_submenu(submenu)
}

pub fn handle_open_single_file() {
    dialog::FileDialogBuilder::default().pick_file(|path_buf| match path_buf {
        Some(p) => println!("{:?}", p),
        _ => {}
    })
}
