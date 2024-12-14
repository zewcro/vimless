use crossterm::{
  cursor,
  terminal::{self, ClearType},
  execute,
  Result,
};
use std::io::{self, Write};

pub fn render_text(text: &Vec<String>, file_path: &str) -> Result<()> {
  execute!(
      io::stdout(),
      terminal::Clear(ClearType::All),
      cursor::MoveTo(0, 0)
  )?;

  writeln!(io::stdout(), "{}\r", file_path)?;

  for line in text {
      write!(io::stdout(), "{}\r\n", line)?;
  }

  io::stdout().flush()?;
  Ok(())
}

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
  execute!(
      io::stdout(),
      cursor::MoveTo(cursor_x as u16, (cursor_y + 1) as u16)
  )
}