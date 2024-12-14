mod editor;

use crossterm::{cursor, execute, terminal, Result};
use std::io::{self};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

fn main() -> Result<()> {
    let mut editor = editor::core::TextEditor::new();

    editor.load_file("test.py");

    terminal::enable_raw_mode()?; 
    execute!(io::stdout(), cursor::Hide)?; 

    editor.render()?;


    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    match (key_event.code, key_event.modifiers) {
                        (KeyCode::Char('x'), KeyModifiers::CONTROL) => break,
                        (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                            editor.save_file();
                        }
                        _ => {
                            editor.handle_input(key_event.code);
                            editor.render()?;
                        }
                    }
                }
            }
        }
    }
    execute!(io::stdout(), cursor::Show)?;
    terminal::disable_raw_mode()?; 
    Ok(())
}