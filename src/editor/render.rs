use crossterm::{
  cursor,
  terminal::{self, ClearType},
  execute,
  style::{Color, SetForegroundColor, ResetColor},
  Result,
};
use std::io::{self, Write};
use std::path::Path;
use chrono::Local;

pub fn render_status_bar(cursor_x: usize, cursor_y: usize) -> Result<()> {
  let status = format!(
      "File: example.txt | Cursor: {}:{}",
      cursor_y + 1,
      cursor_x + 1
  );

  execute!(
      io::stdout(),
      cursor::MoveTo(0, terminal::size()?.1 - 1),
      terminal::Clear(ClearType::CurrentLine)
  )?;

  io::stdout().write_all(status.as_bytes())?;
  io::stdout().flush()?;
  Ok(())
}

pub fn move_cursor(cursor_x: usize, cursor_y: usize) -> Result<()> {
  let header_lines = 3;
  execute!(
      io::stdout(),
      cursor::MoveTo(cursor_x as u16, (cursor_y + header_lines) as u16)
  )
}

pub fn render_text(text: &Vec<String>, file_path: &str) -> Result<()> {
  let path = Path::new(file_path);
  let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown");
  let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
  let icon = match extension {
      "rs" => "🦀", // Rust
      "toml" => "🧾", // TOML
      "json" => "📜", // JSON
      "py" => "🐍", // Python
      "txt" => "📄", // Text file
      "md" => "📝", // Markdown
      _ => "📁", // Default icon
  };

  // Obtenir la taille du terminal
  let (width, _) = terminal::size()?;

  execute!(
      io::stdout(),
      terminal::Clear(ClearType::All),
      cursor::MoveTo(0, 0),
      SetForegroundColor(Color::Blue)
  )?;

  // En-tête stylisé avec des bordures
  let header_top = "╔".to_string() + &"═".repeat(width as usize - 2) + "╗";
  let header_bottom = "╚".to_string() + &"═".repeat(width as usize - 2) + "╝";

  writeln!(io::stdout(), "{}", header_top)?;
  
  // Ligne de titre principale
  let title_line = format!(
      "║ VimLess Editor v0.0.1 | {} {} {} | {}",
      filename, 
      icon, 
      extension.to_uppercase(),
      Local::now().format("%Y-%m-%d %H:%M:%S")
  );
  
  // Compléter la ligne avec des espaces jusqu'à la largeur du terminal
  let padding = " ".repeat(width as usize - title_line.len() - 1);
  writeln!(io::stdout(), "{}{padding}║", title_line)?;

  writeln!(io::stdout(), "{}", header_bottom)?;

  execute!(io::stdout(), ResetColor)?;

  for line in text {
      write!(io::stdout(), "{}\r\n", line)?;
  }

  io::stdout().flush()?;
  Ok(())
}