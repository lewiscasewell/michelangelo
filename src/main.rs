mod art;
mod quotes;
mod watch;

use art::ArtHandler;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let watch_mode = args.contains(&String::from("--watch"));
    let quote_only = args.contains(&String::from("--quote"));

    if quote_only {
        println!("{}", quotes::get_random_quote());
        return;
    }

    let art_handler = ArtHandler::new();

    if watch_mode {
        if let Err(e) = watch::run_watch_mode(art_handler) {
            eprintln!("Error in watch mode: {}", e);
        }
    } else {
        if let Err(e) = art_handler.display_with_quote() {
            eprintln!("Error displaying art: {}", e);
        }
    }
}
