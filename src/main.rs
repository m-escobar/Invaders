use std::error::Error;
use std::io;
use rusty_audio::Audio;

use crossterm::{ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};


fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    //Set audio files
    audio.add("explode", "audio/original/explode.wav");
    audio.add("lose", "audio/original/lose.wav");
    audio.add("move", "audio/original/move.wav");
    audio.add("pew", "audio/original/pew.wav");
    audio.add("startup", "audio/original/startup.wav");
    audio.add("win", "audio/original/win.wav");

    // Play startup sound
    audio.play("startup");

    //Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Cleanup to exit
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    
    Ok(())
}
