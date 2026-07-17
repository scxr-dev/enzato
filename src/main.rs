mod buffer;
mod clipboard;
mod engine;
mod history;
mod viewer;

use std::io::{self, stdout};
use std::panic;
use std::time::Duration;
use crossterm::{
    event::{self, Event, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use engine::Engine;
use viewer::Viewer;

struct RawModeGuard;

impl RawModeGuard {
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = stdout().execute(LeaveAlternateScreen);
    }
}

fn main() -> io::Result<()> {
    let _guard = RawModeGuard::new()?;

    panic::set_hook(Box::new(|info| {
        let _ = disable_raw_mode();
        let _ = stdout().execute(LeaveAlternateScreen);
        eprintln!("Fatal error: {:?}", info);
    }));

    let mut engine = Engine::new();
    if let Some(filepath) = std::env::args().nth(1) {
        let _ = engine.load_file(&filepath);
    }

    let mut viewer = Viewer::new()?;
    viewer.render(&engine)?;

    loop {
        if event::poll(Duration::from_millis(8))? {
            match event::read()? {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Release {
                        continue;
                    }
                    if engine.handle_key(key_event) {
                        break;
                    }
                    viewer.render(&engine)?;
                }
                Event::Resize(width, height) => {
                    viewer.resize(width, height);
                    viewer.render(&engine)?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}