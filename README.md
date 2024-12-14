# Vimless

Vimless is a minimalist console-based text editor inspired by **nano**. It is designed to offer a simple, intuitive, and fast alternative for those who want to escape the complexity of Vim.

## Why Vimless?

- **Ease of Use**: A minimalist user interface, simple keyboard shortcuts, and features inspired by nano.
- **Lightweight and Fast**: Built with Rust, it is efficient and scalable, even for large files.
- **Philosophy**: Vimless is for developers who value simplicity while making a bold statement about their dislike for Vim. ;)

## Features

- 🔒 Load and save text files
- 🛏ï Intuitive cursor navigation (arrows, insertion, deletion, etc.)
- ⭕ Nano-like keyboard shortcuts:
  - **Ctrl+O**: Save
  - **Ctrl+X**: Exit
  - **Ctrl+K**: Cut a line
  - **Ctrl+U**: Paste a line
- 🌐 Minimalist interface with clear rendering in the terminal
- 🤖 **Upcoming AI Features**:
  - Autocomplete to enhance productivity
  - Intelligent suggestions for code and text editing

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/zewcro/vimless.git
   cd vimless
   ```

2. Build the project with Cargo (Rust):

   ```bash
   cargo build --release
   ```

3. Add the executable to your PATH for easy access:

   ```bash
   cp target/release/vimless /usr/local/bin/vimless
   ```

4. Run Vimless:

   ```bash
   vimless myfile.txt
   ```

## Contribution

Contributions are welcome! If you want to contribute:

1. Fork the repository
2. Create a branch for your feature: `git checkout -b feature/my-feature`
3. Make your changes and commit: `git commit -m "Add my feature"`
4. Submit a Pull Request

## License

Vimless is distributed under the [Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International (CC BY-NC-SA 4.0)](https://creativecommons.org/licenses/by-nc-sa/4.0/).

This means:

- **Attribution**: You must credit the original author.
- **NonCommercial**: You may not use this project for commercial purposes.
- **ShareAlike**: If you modify or adapt this project, you must distribute it under the same license.

For commercial use or exceptions to this license, please contact me.

---

© 2024 Théo, all rights reserved.
