use std::{time::Duration};

use crate::{reel::Reel, NUM_COLS, fruits::{Fruit, FRUIT_WIDTH}, frame::Drawable};
use crossterm::style::Color;
use random_number::random;
use rusty_audio::Audio;
use rusty_time::prelude::Timer;

pub struct Reels {
	reels:Vec<Reel>,
	spinning:bool,
	stop_timer:Timer,
	force:[&'static str;3],
}

impl Reels {
	pub fn new() -> Self {
		let mut reels=Vec::new();
		reels.push(Reel::new(NUM_COLS*3/10-FRUIT_WIDTH/2-1,1,0));
		reels.push(Reel::new(NUM_COLS*1/2-FRUIT_WIDTH/2-1,1,120));
		reels.push(Reel::new(NUM_COLS*7/10-FRUIT_WIDTH/2-1,1,110));
		for _ in 0..30 { reels[0].push_symbol(Fruit::new_cherry()); }
		for _ in 0..50 { reels[0].push_symbol(Fruit::new_bell()); }
		for _ in 0..30 { reels[0].push_symbol(Fruit::new_lemon()); }
		for _ in 0..20 { reels[0].push_symbol(Fruit::new_orange()); }
		for _ in 0..11 { reels[0].push_symbol(Fruit::new_star()); }
		for _ in 0..9 { reels[0].push_symbol(Fruit::new_seven()); }

		for _ in 0..30 { reels[1].push_symbol(Fruit::new_cherry()); }
		for _ in 0..50 { reels[1].push_symbol(Fruit::new_bell()); }
		for _ in 0..30 { reels[1].push_symbol(Fruit::new_lemon()); }
		for _ in 0..20 { reels[1].push_symbol(Fruit::new_orange()); }
		for _ in 0..11 { reels[1].push_symbol(Fruit::new_star()); }
		for _ in 0..9 { reels[1].push_symbol(Fruit::new_seven()); }

		for _ in 0..30 { reels[2].push_symbol(Fruit::new_cherry()); }
		for _ in 0..10 { reels[2].push_symbol(Fruit::new_bell()); }
		for _ in 0..31 { reels[2].push_symbol(Fruit::new_lemon()); }
		for _ in 0..26 { reels[2].push_symbol(Fruit::new_orange()); }
		for _ in 0..11 { reels[2].push_symbol(Fruit::new_star()); }
		for _ in 0..10 { reels[2].push_symbol(Fruit::new_seven()); }

		for r in &mut reels {
			r.shuffle_symbols();
		}
		Self {
			reels,
			spinning:false,
			stop_timer:Timer::from_millis(1000),
			force:["","",""],
		}
	}

	pub fn spin(&mut self, fast_test:bool,force:&[&'static str;3]) {
		self.force=*force;
		if fast_test {
			for reel in &mut self.reels {
				reel.fast_stop(random!(0,reel.num_symbols()-1));
			}
		} else {
			for reel in &mut self.reels {
				reel.start_spin();
			}
			self.stop_timer.reset();
			self.spinning=true;
		}
	}

	// Should be called only if a spin is in progress, really
	// Returns false when the spin has finished, true otherwise
	pub fn update_spin(&mut self, delta: Duration, audio:&mut Audio, fast_test:bool) -> bool {
		for reel in &mut self.reels {
			reel.update_spin(delta);
		}
		if self.spinning {
			self.stop_timer.update(delta);
			'reels: for (i,reel) in &mut self.reels.iter_mut().enumerate() {
				if reel.spinning() {
					if reel.stopping() {
						if reel.check_hit_target() {
							if !fast_test {
								audio.play("stop");
							}
							self.stop_timer.reset();
						}
					} else if self.stop_timer.ready {
						if self.force[i].len()>0 {
							reel.force_stop(self.force[i]);
						}
						else {
							reel.stop(random!(0,reel.num_symbols()-1));
						}
					}
					self.spinning=self.reels.iter().fold(false,|a,r| {a||r.spinning()});
					break 'reels;
				}
			}
		}
		self.spinning
	}

	pub fn calculate_win(&self)->u32 {
		if self.reels[0].current_symbol_name() == self.reels[1].current_symbol_name() {
			if self.reels[1].current_symbol_name() == self.reels[2].current_symbol_name() {
				if self.reels[0].current_symbol_name()=="Bell" {
					return 5_00
				}
				return 1_00
			}
			return 0_50
		}
		0_00
	}
}

impl Drawable for Reels {
	fn draw(&self, frame:&mut crate::frame::Frame) {
		let x0=self.reels[0].get_left_x()-1;
		let x1=self.reels.last().unwrap().get_right_x()+1;
		let y=self.reels[0].get_centre_y();
		for x in x0..=x1 {
			frame.set_pixel(x, y, "-".to_string(), Color::White, Color::Black);
		}
		for reel in &self.reels {
			reel.draw(frame);
		}
	}
}

