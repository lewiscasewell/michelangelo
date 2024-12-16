use crate::art::ArtHandler;
use signal_hook::consts::SIGWINCH;
use signal_hook::iterator::Signals;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

pub fn run_watch_mode(handler: ArtHandler) -> Result<(), Box<dyn std::error::Error>> {
    let mut signals = Signals::new(&[SIGWINCH])?;
    let (tx, rx): (std::sync::mpsc::Sender<()>, Receiver<()>) = channel();

    thread::spawn(move || {
        for _ in signals.forever() {
            let _ = tx.send(());
        }
    });

    loop {
        if rx.recv_timeout(Duration::from_millis(500)).is_ok() {
            handler.display_with_quote()?;
        }
    }
}
