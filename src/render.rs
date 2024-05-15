use std::io::{Stdout, Write};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool)
{
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x_pos, col) in curr_frame.iter().enumerate()
    {
        for (y_pos, val) in col.iter().enumerate()
        {
            if *val != last_frame[x_pos][y_pos] || force
            {
                stdout.queue(MoveTo(x_pos as u16, y_pos as u16)).unwrap();
                print!("{}", val);
            }
        }
    }
    stdout.flush().unwrap();
}