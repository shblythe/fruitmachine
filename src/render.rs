use std::io::Stdout;
use std::io::Write;
use crate::NUM_COLS;
use crate::NUM_ROWS;
use crate::frame::Frame;
use crossterm::QueueableCommand;
use crossterm::style::Colors;
use crossterm::style::SetColors;
use crossterm::style::{SetBackgroundColor,Color};
use crossterm::terminal::{Clear,ClearType};
use crossterm::cursor::MoveTo;

pub fn render(stdout:&mut Stdout, last_frame:&Frame, curr_frame:&Frame, force:bool) {
	if force {
		stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
		stdout.queue(Clear(ClearType::All)).unwrap();
		stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
	}
	for x in 0..NUM_COLS {
		for y in 0..NUM_ROWS {
			let p=curr_frame.get_pixel(x, y);
			if p != last_frame.get_pixel(x, y) || force {
				stdout.queue(MoveTo(x as u16,y as u16)).unwrap();
				stdout.queue(SetColors(Colors::new(p.foreground,p.background))).unwrap();
				print!("{}",p.character);
			}
		}
	}
	stdout.flush().unwrap();
}