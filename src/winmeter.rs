use crossterm::style::Color;

use crate::frame::{Frame,Drawable};

pub struct WinMeter {
	x: usize,
	y: usize,
	amount: u32
}

impl WinMeter {
	pub fn new(x:usize,y:usize) -> Self {
		Self { x, y, amount:0 }
	}

	pub fn set_amount(&mut self,amount:u32) {
		self.amount=amount;
	}
}

impl Drawable for WinMeter {
	fn draw(&self,frame:&mut Frame) {
		frame.print_str(self.x, self.y, format!(" WIN: £{:6.2} ",(self.amount as f64)/100.0),
										Color::Black, Color::White);
	}
}