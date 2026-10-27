use arboard::Clipboard;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

pub fn main() -> rustyline::Result<()> {
    // Keep a single clipboard handle alive for the whole session.
    // On X11/Wayland, the copied contents are lost once the handle is dropped.
    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => Some(clipboard),
        Err(cause) => {
            eprintln!("warning: Unable to access clipboard ({cause}), only printing output");
            None
        }
    };
    let mut editor = DefaultEditor::new()?;
    loop {
        match editor.readline(">>> ") {
            Ok(line) => {
                let latex = line.trim();
                if latex.is_empty() {
                    continue;
                }
                editor.add_history_entry(latex)?;
                let unicode = unicodeit::replace(latex);
                println!("{unicode}");
                if let Some(Err(cause)) = clipboard.as_mut().map(|c| c.set_text(&unicode)) {
                    eprintln!("warning: Failed to copy to clipboard: {cause}");
                }
            }
            // Ctrl-C discards the current line, like most shells
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(cause) => return Err(cause),
        }
    }
    Ok(())
}
