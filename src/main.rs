use std::error::Error;
use std::io;
use crossterm::{ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>> {

    let mut audio  = Audio::new();

    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("win", "sounds/win.wav");
    audio.add("startup", "startup.wav");

    audio.play("startup");

    //terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;   //to get input from keyboard. The "?" is to crash if an error occurs
    stdout.execute(EnterAlternateScreen)?;  // "execute" is an extension provided by crossterm
                                                     // to ommediatelly execute something
    stdout.execute(Hide)?;

    //Cleanup
    audio.wait();
    stdout.execute(Show)?;   //reverse order
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode();
    Ok(())

}
