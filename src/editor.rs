use crossterm::{
  cursor,
  terminal::{self, ClearType},
  execute,
  event::KeyCode,
  Result,
};
use std::fs;
use std::io::{self, Write};

pub struct TextEditor {
  text: Vec<String>,
  cursor_x: usize,
  cursor_y: usize,
}

impl TextEditor {
  pub fn new() -> Self {
      Self {
          text: vec![String::new()],
          cursor_x: 0,
          cursor_y: 0,
      }
  }

  pub fn load_file(&mut self, file_path: &str) {
      if let Ok(contents) = fs::read_to_string(file_path) {
          self.text = contents.lines().map(|line| line.to_string()).collect();
      } else {
          self.text = vec![String::new()];
      }
  }

  pub fn save_file(&self, file_path: &str) {
      let contents = self.text.join("\n");
      fs::write(file_path, contents).unwrap();
  }

  pub fn render(&self) -> Result<()> {
      execute!(
          io::stdout(),
          terminal::Clear(ClearType::All),
          cursor::MoveTo(0, 0)
      )?;

      for line in &self.text {
          println!("{}", line);
      }
      execute!(io::stdout(), cursor::MoveTo(self.cursor_x as u16, self.cursor_y as u16))?;
      Ok(())
  }

  pub fn handle_input(&mut self, key: KeyCode) {
      match key {
          KeyCode::Char(c) => {
              self.text[self.cursor_y].insert(self.cursor_x, c);
              self.cursor_x += 1;
          }
          KeyCode::Backspace => {
              if self.cursor_x > 0 {
                  self.text[self.cursor_y].remove(self.cursor_x - 1);
                  self.cursor_x -= 1;
              }
          }
          KeyCode::Enter => {
              let remaining_text = self.text[self.cursor_y].split_off(self.cursor_x);
              self.text.insert(self.cursor_y + 1, remaining_text);
              self.cursor_x = 0;
              self.cursor_y += 1;
          }
          KeyCode::Up => {
              if self.cursor_y > 0 {
                  self.cursor_y -= 1;
                  self.cursor_x = self.cursor_x.min(self.text[self.cursor_y].len());
              }
          }
          KeyCode::Down => {
              if self.cursor_y + 1 < self.text.len() {
                  self.cursor_y += 1;
                  self.cursor_x = self.cursor_x.min(self.text[self.cursor_y].len());
              }
          }
          KeyCode::Left => {
              if self.cursor_x > 0 {
                  self.cursor_x -= 1;
              }
          }
          KeyCode::Right => {
              if self.cursor_x < self.text[self.cursor_y].len() {
                  self.cursor_x += 1;
              }
          }
          _ => {}
      }
  }
}
