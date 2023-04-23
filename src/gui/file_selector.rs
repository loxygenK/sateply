use rfd::AsyncFileDialog;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct FileDialog {
    selected_file: Arc<Mutex<Option<PathBuf>>>,
}

impl FileDialog {
    pub fn show(&mut self) {
        let selected_file = self.selected_file.clone();

        tokio::spawn(async move {
            let selected = AsyncFileDialog::default()
                .add_filter("Lua program", &["lua"])
                .pick_file()
                .await;

            let mut selected_file = selected_file.lock().unwrap();
            *selected_file = selected.as_ref().map(|file| file.path().to_path_buf());
        });
    }

    pub fn get_selected(&self) -> Option<PathBuf> {
        self.selected_file.lock().unwrap().clone()
    }

    pub fn read_selected(&self) -> Option<String> {
        let Some(path) = self.get_selected() else { return None };

        Some(std::fs::read_to_string(path).unwrap())
    }

    pub fn forget_selected(&mut self) {
        (*self.selected_file.lock().unwrap()) = None
    }
}
