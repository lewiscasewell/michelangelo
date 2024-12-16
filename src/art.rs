use std::collections::HashMap;
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

        let size_key =
            ((width.max(min_width) - min_width) / step * step + min_width).min(max_width);

        // Use a HashMap to embed and retrieve ASCII art
        let ascii_map: HashMap<usize, &str> = HashMap::from([
            (50, include_str!("../creation_of_ascii/50.txt")),
            (75, include_str!("../creation_of_ascii/75.txt")),
            (100, include_str!("../creation_of_ascii/100.txt")),
            (125, include_str!("../creation_of_ascii/125.txt")),
            (150, include_str!("../creation_of_ascii/150.txt")),
            (175, include_str!("../creation_of_ascii/175.txt")),
            (200, include_str!("../creation_of_ascii/200.txt")),
            (225, include_str!("../creation_of_ascii/225.txt")),
            (250, include_str!("../creation_of_ascii/250.txt")),
            (275, include_str!("../creation_of_ascii/275.txt")),
            (300, include_str!("../creation_of_ascii/300.txt")),
            (325, include_str!("../creation_of_ascii/325.txt")),
            (350, include_str!("../creation_of_ascii/350.txt")),
            (375, include_str!("../creation_of_ascii/375.txt")),
            (400, include_str!("../creation_of_ascii/400.txt")),
        ]);

        // Retrieve the ASCII art for the given size
        ascii_map
            .get(&size_key)
            .map(|&art| art.to_string())
            .ok_or_else(|| {
                ArtError::IoError(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "painting not found",
                ))
            })
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
            ArtError::IoError(e) => write!(f, "io error: {}", e),
            ArtError::TerminalSizeError => write!(f, "could not determine terminal size"),
        }
    }
}

impl std::error::Error for ArtError {}
