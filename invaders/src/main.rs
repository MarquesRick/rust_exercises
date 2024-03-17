use crossterm::{
    cursor::{Hide, Show}, event, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand
};
use rusty_audio::Audio;
use std::{error::Error, io, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "src/sounds/explode.wav");
    audio.add("lose", "src/sounds/lose.wav");
    audio.add("move", "src/sounds/move.wav");
    audio.add("pew", "src/sounds/pew.wav");
    audio.add("startup", "src/sounds/startup.wav");
    audio.add("win", "src/sounds/win.wav");
    audio.play("startup");

    //terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //game loop
    'gameloop: loop {
        //input
        while event::poll(Duration::default())? {
            
        }
    }

    //cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
