use anyhow::Result;
use arboard::Clipboard;

pub struct ClipboardManager {
    clipboard: Clipboard,
}

impl ClipboardManager {
    pub fn new() -> Result<Self> {
        let clipboard = Clipboard::new()?;
        Ok(Self { clipboard })
    }

    /// Copy text to system clipboard
    pub fn copy_text(&mut self, text: &str) -> Result<()> {
        self.clipboard.set_text(text)?;
        log::info!("Copied {} chars to clipboard", text.len());
        Ok(())
    }

    /// Get text from system clipboard
    pub fn get_text(&mut self) -> Result<String> {
        let text = self.clipboard.get_text()?;
        Ok(text)
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self::new().expect("Failed to initialize clipboard")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_copy_paste() {
        let mut clipboard = ClipboardManager::new().unwrap();
        let test_text = "Hello, ValleyFlow!";
        clipboard.copy_text(test_text).unwrap();
        let retrieved = clipboard.get_text().unwrap();
        assert_eq!(retrieved, test_text);
    }
}
