use crossterm::event::KeyCode;
use crate::editor::core::TextEditor;

pub fn handle_keypress(editor: &mut TextEditor, key: KeyCode) {
    match key {
        KeyCode::Char(c) => {
            if editor.cursor_y < editor.text.len() {
                editor.text[editor.cursor_y].insert(editor.cursor_x, c);
                editor.cursor_x += 1;
            }
        }
        KeyCode::Backspace => {
            if editor.cursor_y < editor.text.len() && editor.cursor_x > 0 {
                editor.text[editor.cursor_y].remove(editor.cursor_x - 1);
                editor.cursor_x -= 1;
            }
        }
        KeyCode::Enter => {
            if editor.cursor_y < editor.text.len() {
                let remaining_text = editor.text[editor.cursor_y].split_off(editor.cursor_x);
                editor.text.insert(editor.cursor_y + 1, remaining_text);
                editor.cursor_x = 0;
                editor.cursor_y += 1;
            }
        }
        KeyCode::Left => {
            if editor.cursor_x > 0 {
                editor.cursor_x -= 1;
            }
        }
        KeyCode::Right => {
            if editor.cursor_y < editor.text.len() && editor.cursor_x < editor.text[editor.cursor_y].len() {
                editor.cursor_x += 1;
            }
        }
        KeyCode::Up => {
            if editor.cursor_y > 0 {
                editor.cursor_y -= 1;
                editor.cursor_x = editor.cursor_x.min(editor.text[editor.cursor_y].len());
            }
        }
        KeyCode::Down => {
            if editor.cursor_y + 1 < editor.text.len() {
                editor.cursor_y += 1;
                editor.cursor_x = editor.cursor_x.min(editor.text[editor.cursor_y].len());
            }
        }
        _ => {}
    }
}