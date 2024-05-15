use std::time::Duration;
use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};
use crate::invaders::{Invader, Invaders};
use crate::shot::Shot;

pub struct Player
{
    x_pos: usize,
    y_pos: usize,
    shots: Vec<Shot>,
}

impl Player
{
    pub fn new() -> Player
    {
        Player {
            x_pos: NUM_COLS / 2,
            y_pos: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self)
    {
        if self.x_pos > 0 {
            self.x_pos -= 1;
        }
    }

    pub fn move_right(&mut self)
    {
        if self.x_pos < NUM_COLS - 1 {
            self.x_pos += 1;
        }
    }

    pub fn shoot(&mut self) -> bool
    {
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x_pos, self.y_pos - 1));
            true
        }
        else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration)
    {
        self.shots.iter_mut().for_each(|shot| shot.update(delta));
        self.shots.retain(|shot| !shot.dead());
    }

    pub fn detect_hit(&mut self, invaders: &mut Invaders) -> bool
    {
        let mut hit_something = false;
        self.shots.iter_mut().for_each(|shot| {
            if !shot.exploding
            {
                if invaders.kill_invader(shot.x, shot.y)
                {
                    hit_something = true;
                    shot.explode();
                }
            }
        });
        return hit_something;
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x_pos][self.y_pos] = "A";
        for shot in self.shots.iter() {
           shot.draw(frame);
        }
    }
}