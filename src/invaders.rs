use std::cmp::max;
use std::fmt::{Display, Formatter, Write};
use std::time::Duration;
use rusty_time::prelude::Timer;
use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};

pub struct Invader
{
    pub x: usize,
    pub y: usize,
}

impl Display for Invader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.x, self.y))
    }
}

pub struct Invaders
{
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders
{
    pub fn new() -> Self
    {
        let mut army = Vec::new();
        for x in 0..NUM_COLS
        {
            for y in 0..NUM_ROWS
            {
                if (x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (y < 9)
                    && (x % 2 == 0)
                    && (y % 2 == 0)
                {
                    army.push(Invader { x, y });
                }
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(1000),
            direction: 1,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;
            if self.direction == -1 {
                let min_x = self.army.iter()
                    .map(|inv| inv.x)
                    .min()
                    .unwrap_or(0);
                if 0 == min_x
                {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter()
                    .map(|inv| inv.x)
                    .max()
                    .unwrap_or(0);
                if NUM_COLS - 1 == max_x
                {
                    self.direction = -1;
                    downwards = true;
                }
            }
            if downwards
            {
                let new_duration = max
                    (self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }
        false
    }

    pub fn all_killed(&self) -> bool
    {
        self.army.is_empty()
    }
    pub fn reached_bottom(&self) -> bool
    {
        self.army.iter().map(|inv| inv.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }
    pub fn kill_invader(&mut self, x: usize, y: usize) -> bool
    {
        let ct = self.army.len();
        self.army.retain(|inv| {
            inv.x != x || inv.y != y
        });
        ct != self.army.len()
    }
}

impl Drawable for Invaders
{
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            if 0 == invader.x % 2
            {
                frame[invader.x][invader.y] = "X";
            }
            else {
                frame[invader.x][invader.y] = "+";
            }
        }
    }
}