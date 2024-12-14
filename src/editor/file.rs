use std::fs;

pub fn load_file(file_path: &str) -> Vec<String> {
    if let Ok(contents) = fs::read_to_string(file_path) {
        let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
        if lines.is_empty() {
            vec![String::new()]
        } else {
            lines
        }
    } else {
        vec![String::new()]
    }
}

pub fn save_file(file_path: &str, text: &[String]) {
    let contents = text.join("\n");
    fs::write(file_path, contents).expect("Unable to save file.");
}