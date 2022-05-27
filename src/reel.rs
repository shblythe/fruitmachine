use std::time::Duration;

use crossterm::style::Color;
use random_number::rand::{prelude::SliceRandom, thread_rng};
use rusty_time::prelude::Timer;

use crate::{fruits::{FruitDrawable, FRUIT_HEIGHT, FRUIT_WIDTH },
						frame::{Drawable, Frame},
						};

pub struct Reel {
	x:usize,
	y:usize,
	symbols: Vec<Box<dyn FruitDrawable>>,
	current_pos:usize,
	spinning:bool,
	stopping:bool,
	target_pos:usize,
	spin_timer:Timer,
	line:usize,
}

impl Reel {
	pub fn new(x:usize,y:usize,start_pos:usize)->Self {
		Self {
			x, y, symbols:Vec::new(), current_pos:start_pos, spinning:false, spin_timer:Timer::from_millis(20), line:0, stopping:false, target_pos:start_pos
		}
	}

	pub fn push_symbol(&mut self, symbol:Box<dyn FruitDrawable>) {
		self.symbols.push(symbol);
	}

	pub fn shuffle_symbols(&mut self) {
		let mut rng=thread_rng();
		self.symbols.shuffle(&mut rng);
	}

	pub fn start_spin(&mut self) {
		self.spinning=true;
	}

	pub fn stop(&mut self,pos:usize) {
		self.target_pos=pos;
		self.stopping=true;
	}

	pub fn force_stop(&mut self,force:&str) {
		for (i,s) in self.symbols.iter().enumerate() {
			if s.name() == force {
				self.target_pos=i;
				self.stopping=true;
			}
		}
	}

	fn stopped(&mut self) {
		self.line=0;
		self.stopping=false;
		self.spinning=false;
	}

	pub fn fast_stop(&mut self,pos:usize) {
		self.current_pos=pos;
		self.stopped();
	}

	pub fn check_hit_target(&mut self) -> bool {
		if self.symbols[self.current_pos].name() == self.symbols[self.target_pos].name() {
			self.stopped();
			return true;
		}
		false
	}
	
	pub fn spinning(&self) -> bool {
		self.spinning
	}

	pub fn stopping(&self) -> bool {
		self.stopping
	}

	pub fn num_symbols(&self)->usize {
		self.symbols.len()
	}

	pub fn current_symbol_name(&self)->&'static str {
		self.symbols[self.current_pos].name()
	}

	pub fn get_centre_y(&self)->usize {
		self.y+FRUIT_HEIGHT*3/2+1
	}

	pub fn get_left_x(&self)->usize {
		self.x
	}

	pub fn get_right_x(&self)->usize {
		self.x+FRUIT_WIDTH-1
	}

	pub fn update_spin(&mut self,delta:Duration) {
		if self.spinning {
			self.spin_timer.update(delta);
			if self.spin_timer.ready {
				self.line+=1;
				if self.line==FRUIT_HEIGHT+1 {
					self.line=0;
					if self.current_pos==0 {
						self.current_pos=self.symbols.len()-1;
					} else {
						self.current_pos-=1;
					}
				}
				self.spin_timer.reset();
			}
		}
	}
}

impl Drawable for Reel {
	fn draw(&self,frame:&mut Frame) {
		if false /*self.spinning*/ {
			// let blur=Blur::new(0,0);
			// blur.draw_fruit_at(self.x, self.y, frame);
			// blur.draw_fruit_at(self.x, self.y+FRUIT_HEIGHT+1, frame);
			// blur.draw_fruit_at(self.x, self.y+(FRUIT_HEIGHT+1)*2, frame);
		} else {
			if self.line>0 {
				// Display extra symbol at top
				self.symbols[(self.current_pos+self.symbols.len()-2)%self.symbols.len()]
					.draw_fruit_at(self.x, (self.y as i32)-(FRUIT_HEIGHT as i32)- 1, (self.line).try_into().unwrap(), FRUIT_HEIGHT-self.line+1, 0, frame)
			}

			self.symbols[
				if self.current_pos>0 { self.current_pos-1 } else { self.symbols.len()-1 }
			].draw_fruit_at(self.x,(self.y).try_into().unwrap(),(self.line).try_into().unwrap(),0,0,frame);

			self.symbols[self.current_pos].draw_fruit_at(self.x,(self.y+FRUIT_HEIGHT+1).try_into().unwrap(),(self.line).try_into().unwrap(),0,0,frame);

			self.symbols[
				if self.current_pos<self.symbols.len()-1 { self.current_pos+1 } else { 0 }
			].draw_fruit_at(self.x,(self.y+(FRUIT_HEIGHT+1)*2).try_into().unwrap(),(self.line).try_into().unwrap(),0,self.line,frame);
		}
		for x in 0..FRUIT_WIDTH {
			if self.line>0 {
				frame.set_pixel(self.x+x,self.y-1+(self.line as usize)," ".to_string(),Color::Black,Color::White);
			}
			frame.set_pixel(self.x+x,self.y+FRUIT_HEIGHT+(self.line as usize)," ".to_string(),Color::Black,Color::White);
			frame.set_pixel(self.x+x,self.y+FRUIT_HEIGHT*2+1+(self.line as usize)," ".to_string(),Color::Black,Color::White);
		}
	}
}