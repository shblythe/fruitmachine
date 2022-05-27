use crossterm::style::Color;

use crate::frame::{Frame,Drawable};

pub struct Credit {
	x: usize,
	y: usize,
	amount: u32
}

impl Credit {
	pub fn new(x:usize,y:usize,amount:u32) -> Self {
		Self { x, y, amount }
	}

	pub fn take_credit(&mut self,stake:u32) {
		// TODO: Error handling!
		self.amount-=stake;
	}

	pub fn add_win(&mut self,win:u32) {
		self.amount+=win;
	}

	pub fn enough_credit(&self,stake:u32) -> bool {
		self.amount>=stake
	}
}

impl Drawable for Credit {
	fn draw(&self,frame:&mut Frame) {
		frame.print_str(self.x, self.y, format!(" CREDIT: £{:6.2} ",(self.amount as f64)/100.0),
										Color::Black, Color::White);
	}
}