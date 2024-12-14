mod editor;

use crossterm::{cursor, execute, terminal, Result};
use std::io::{self};

fn main() -> Result<()> {
    let mut editor = editor::core::TextEditor::new();

    editor.load_file("example.txt");

    terminal::enable_raw_mode()?; // Active le mode "raw"
    execute!(io::stdout(), cursor::Hide)?; // Masque le curseur

    loop {
        editor.render()?; // Rend l'écran

        if let crossterm::event::Event::Key(event) = crossterm::event::read()? {
            match event.code {
                crossterm::event::KeyCode::Char('q') => break, // Quitte l'application
                crossterm::event::KeyCode::Char('s') => {
                    editor.save_file("example.txt"); // Sauvegarde le fichier
                }
                _ => editor.handle_input(event.code), // Gère les autres entrées utilisateur
            }
        }
    }

    execute!(io::stdout(), cursor::Show)?;
    terminal::disable_raw_mode()?; 
    Ok(())
}