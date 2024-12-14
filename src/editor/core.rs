use crate::editor::{file, render, input};
use crossterm::event::KeyCode;
use crossterm::{cursor, execute, Result};
use std::io::{self};

pub struct TextEditor {
    pub text: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub status_message: String,
    pub file_path: String,
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            text: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
            status_message: "Welcome to vimless! Press Ctrl+Q to quit.".to_string(),
            file_path: String::new(),
        }
    }

    pub fn load_file(&mut self, file_path: &str) {
        self.text = file::load_file(file_path);
        self.file_path = std::fs::canonicalize(file_path)
            .expect("Unable to get full path")
            .to_str()
            .unwrap()
            .to_string();
    }

    pub fn save_file(&self) {
        file::save_file(&self.file_path, &self.text);
    }

        pub fn render(&self) -> crossterm::Result<()> {
            render::render_text(&self.text, &self.file_path)?; 
            render::render_status_bar(self.cursor_x, self.cursor_y)?; 
            render::move_cursor(self.cursor_x, self.cursor_y)?; 
            execute!(io::stdout(), cursor::Show)?;
            Ok(())
        }
        
    pub fn handle_input(&mut self, key: KeyCode) {
        input::handle_keypress(self, key);
    }
}