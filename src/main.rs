mod editor;

use crossterm::{cursor, execute, terminal, Result};
use std::io::{self};

fn main() -> Result<()> {
    let mut editor = editor::TextEditor::new();

    editor.load_file("example.txt");

    terminal::enable_raw_mode()?;
    execute!(io::stdout(), cursor::Hide)?;

    loop {
        editor.render()?;
        if let crossterm::event::Event::Key(event) = crossterm::event::read()? {
            if let crossterm::event::KeyCode::Char('q') = event.code {
                break; // Exit the program
            }
            if let crossterm::event::KeyCode::Char('s') = event.code {
                editor.save_file("example.txt"); 
            }
            editor.handle_input(event.code);
        }
    }

    execute!(io::stdout(), cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
