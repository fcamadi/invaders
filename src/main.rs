use std::error::Error;
use std::{io, thread};
use std::sync::mpsc;
use std::time::Duration;
use crossterm::{event, ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;
use invaders::{frame, render};

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

    //render loop
    // it is better to use crossbeam channels, because they are better performant and have more features,
    // but for this project, mpsc channels are fine (they are built in the standard library).
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move ||  {
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

    'gameloop: loop {
        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop
                    }
                    _ => {}
                }
            }
        }
    }

    //Cleanup
    audio.wait();
    stdout.execute(Show)?;   //reverse order
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode();
    Ok(())

}
