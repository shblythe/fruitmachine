use crossterm::style::Color;

use crate::{NUM_COLS,NUM_ROWS};

#[derive(PartialEq)]
pub struct Pixel {
	pub character:String,
	pub foreground:Color,
	pub background:Color
}

pub struct Frame {
	frame:Vec<Vec<Pixel>>,
}

impl Frame {
  pub fn new(background:Color) -> Self {
		let mut cols:Vec<Vec<Pixel>> = Vec::with_capacity(NUM_COLS);
		for _ in 0..NUM_COLS {
			let mut col:Vec<Pixel>=Vec::with_capacity(NUM_ROWS);
			for _ in 0..NUM_ROWS {
				col.push(Pixel { character:" ".to_string(), foreground:Color::White, background:background});
			}
			cols.push(col);
		}
		Self { frame:cols }
	}

	pub fn set_pixel(&mut self,x:usize,y:usize,ch:String,fg:Color,bg:Color) {
		self.frame[x][y]=Pixel { character: ch, foreground: fg, background: bg }
	}

	pub fn get_pixel(&self,x:usize,y:usize) -> &Pixel {
		&self.frame[x][y]
	}

	pub fn print_str(&mut self,x:usize,y:usize,str:String,fg:Color,bg:Color) {
		for (i,ch) in str.chars().enumerate() {
			let cs=ch.to_string();
			if x+i<NUM_COLS {
				self.set_pixel(x+i,y,cs, fg, bg);
			}
		}
	}
}

pub trait Drawable {
	fn draw(&self, frame:&mut Frame);
}