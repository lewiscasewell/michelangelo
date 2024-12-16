use std::fs;
use std::io::{self, Write};
use terminal_size::{terminal_size, Width};
pub struct ArtHandler {
    quote: String,
}

impl ArtHandler {
    pub fn new() -> Self {
        let quote = crate::quotes::get_random_quote().to_string();
        ArtHandler { quote }
    }

    pub fn display_with_quote(&self) -> Result<(), ArtError> {
        let art = self.get_art_for_terminal_size()?;

        Self::clear_screen()?;
        println!("{}", art);
        println!("\n\n{}", self.quote);

        Ok(())
    }

    fn get_art_for_terminal_size(&self) -> Result<String, ArtError> {
        let width = Self::get_terminal_width()?;
        let step = 25;
        let min_width = 50;
        let max_width = 400;
        let file_base = "creation_of_ascii";

        let file_path = format!(
            "{}/{}.txt",
            file_base,
            ((width.max(min_width) - min_width) / step * step + min_width).min(max_width)
        );

        fs::read_to_string(file_path).map_err(ArtError::IoError)
    }

    fn get_terminal_width() -> Result<usize, ArtError> {
        if let Some((Width(width), _)) = terminal_size() {
            Ok(width as usize)
        } else {
            Err(ArtError::TerminalSizeError)
        }
    }

    fn clear_screen() -> Result<(), ArtError> {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().map_err(|e| ArtError::IoError(e))
    }
}

#[derive(Debug)]
pub enum ArtError {
    IoError(std::io::Error),
    TerminalSizeError,
}

impl std::fmt::Display for ArtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtError::IoError(e) => write!(f, "I/O Error: {}", e),
            ArtError::TerminalSizeError => write!(f, "Could not determine terminal size"),
        }
    }
}

impl std::error::Error for ArtError {}
