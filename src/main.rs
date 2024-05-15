use std::{io, thread};
use std::error::Error;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crossterm::{event, ExecutableCommand, terminal};
use crossterm::cursor::Hide;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::EnterAlternateScreen;
use rusty_audio::Audio;

use invaders::frame::{Drawable, new_frame};
use invaders::invaders::Invaders;
use invaders::player::Player;
use invaders::render;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode","explode.wav");
    audio.add("lose","lose.wav");
    audio.add("move","move.wav");
    audio.add("pew","pew.wav");
    audio.add("startup","startup.wav");
    audio.add("win","win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let (render_tx, render_rx) = mpsc::channel();
    let render_handler = thread::spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            // block on receiver to wait for next frame
            // the next frame will be fed by main event loop
            let curr_frame = match render_rx.recv()
            {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    // game loop
    'gameloop: loop {
        // per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    _ => {}
                }
            }
        }

        player.update(delta);
        if player.detect_hit(&mut invaders)
        {
            audio.play("explode");
        }

        if invaders.all_killed()
        {
            audio.play("win");
            break;
        }

        if invaders.update(delta)
        {
            audio.play("move");
        }

        if invaders.reached_bottom()
        {
            audio.play("lose");
            break;
        }

        // instead of sending before thread started we can
        // just ignore the error returned
        player.draw(&mut curr_frame);
        invaders.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(10))
    }

    // dropping transmitter will cause error in receiver and
    // thread will exit. drop will move the render into a functiin and
    // call destructor
    drop(render_tx);
    render_handler.join().unwrap();

    audio.wait();
    Ok(())
}
