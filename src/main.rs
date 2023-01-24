use std::error::Error;
use std::{io, thread};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use crossterm::{event, ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;
use invaders::{frame, render};
use invaders::frame::{Drawable, new_frame};
use invaders::invaders::{Invader, Invaders};
use invaders::player::Player;

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
                                                     // to immediately execute something
    stdout.execute(Hide)?;

    //render loop
    // it is better to use crossbeam channels, because they are better performant and have more features,
    // but for this project, mpsc channels are fine (they are built in the standard library).
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move ||  {
        let mut last_frame = new_frame();
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

    let mut player  = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    'gameloop: loop {

        //Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();

        let mut curr_frame = new_frame();

        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Up => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop
                    }
                    _ => {}
                }
            }
        }

        // update timers
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }

        //Draw & render
        //player.draw(&mut curr_frame);
        //invaders.draw(&mut curr_frame); adding this line would work, but let's try generics
        let drawables : Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        let _ = render_tx.send(curr_frame); // no needed to get the result
        thread::sleep(Duration::from_millis(1));
    }

    //Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;   //reverse order
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())

}
