use crossterm::style::{Color};

use crate::frame::{Frame};

pub const FRUIT_WIDTH:usize = 8;
pub const FRUIT_HEIGHT:usize = 4;

pub struct Fruit {
	name: &'static str,
	color: Color,
	bg_color: Color,
	pattern: Vec<&'static str>,

}

impl Fruit {

	pub fn name(&self) -> &'static str { &self.name }

	pub fn draw_fruit_at(&self, x:usize,y:i32,y_offset:i32,y_clip_top:usize,y_clip_bottom:usize,frame:&mut Frame) {
		for (row,row_pattern) in self.pattern.iter().enumerate() {
			if row>=y_clip_top && row<self.pattern.len()-y_clip_bottom {
				for (col,c) in row_pattern.bytes().enumerate() {
					frame.set_pixel(x+col,(y+(row as i32)+y_offset).try_into().unwrap()," ".to_string(),Color::Black,
													if c==b'1' { self.color } else { self.bg_color });
				}
			}
		}
	}

  pub fn new_cherry() -> Self
	{
		Self {
			name: "Cherry",
			color:Color::Red, bg_color:Color::White,
			pattern:vec!(
				"00010000",
				"00111100",
				"01111110",
				"00111100",
			)
		}
	}

	pub fn new_lemon() -> Self {
		Self {
			name: "Lemon",
			color:Color::DarkYellow, bg_color:Color::White,
			pattern:vec!(
				"00011110",
				"00111110",
				"01111100",
				"01111000"
			)
		} 
	}

	pub fn new_bell() -> Self {
		Self {
			name: "Bell",
			color:Color::Yellow, bg_color:Color::DarkGrey,
			pattern:vec!(
				"00011000",
				"00111100",
				"00111100",
				"01111110"
			)
		} 
	}

	pub fn new_orange() -> Self {
		Self {
			name: "Orange",
			color:Color::Rgb { r:242, g:98, b:46 }, bg_color:Color::White,
			pattern:vec!(
				"00111100",
				"01110110",
				"01111110",
				"00111100"
			)
		}
	}

	pub fn new_star() -> Self {
		Self {
			name: "Star",
			color:Color::DarkGrey, bg_color:Color::White,
			pattern:vec!(
				"00011000",
				"01111110",
				"00111100",
				"01100110"
			)
		}
	}

	pub fn new_seven() -> Self {
		Self {
			name: "Seven",
			color:Color::Magenta, bg_color:Color::White,
			pattern:vec!(
				"01111110",
				"00001110",
				"00111000",
				"01110000"
			)
		} 
	}
}
