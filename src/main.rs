use std::error::Error;
use std::{io, thread};
use std::sync::mpsc;
use std::time::Duration;
use rusty_audio::Audio;

use crossterm::{ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::event;
use crossterm::event::{Event, KeyCode};
// use crossterm::event::MediaKeyCode::Play;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use space_invaders::{frame, render};
use space_invaders::frame::{Drawable, Frame, new_frame};
use space_invaders::player::Player;
// use space_invaders::render::render;

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

    // Render loop in a separate thread
    // using mpsc but move to crossbeam channel
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };

            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });
    

    // Game loop
    let mut player = Player::new();
    
    'gameloop: loop {
        // Per frame init
        let mut curr_frame = new_frame();
        
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
        
        //Draw and render
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }
    
    
    // Cleanup to exit
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    
    Ok(())
}
