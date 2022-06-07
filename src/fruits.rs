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

pub struct Cherry { fruit: Fruit }
impl FruitDrawable for Cherry { fn fruit(&self) -> &Fruit { &self.fruit } }

impl Cherry {
  pub fn new() -> Self
	{
		Self {
			fruit: Fruit {
				name: "Cherry",
				color:Color::Red, bg_color:Color::White,
				pattern:vec!(
					"00010000",
					"00111100",
					"01111110",
					"00111100",
				) },
		}
	}
}

pub struct Lemon { fruit: Fruit }
impl FruitDrawable for Lemon { fn fruit(&self) -> &Fruit {&self.fruit} }

impl Lemon {
	pub fn new() -> Self {
		Self { fruit: Fruit {
			name: "Lemon",
			color:Color::DarkYellow, bg_color:Color::White,
			pattern:vec!(
				"00011110",
				"00111110",
				"01111100",
				"01111000"
			)
		} }
	}
}


pub struct Bell { fruit: Fruit }
impl FruitDrawable for Bell { fn fruit(&self) -> &Fruit {&self.fruit} }
impl Bell {
	pub fn new() -> Self {
		Self { fruit: Fruit {
			name: "Bell",
			color:Color::Yellow, bg_color:Color::DarkGrey,
			pattern:vec!(
				"00011000",
				"00111100",
				"00111100",
				"01111110"
			)
		} }
	}
}

pub struct Orange { fruit: Fruit }
impl FruitDrawable for Orange { fn fruit(&self) -> &Fruit {&self.fruit} }
impl Orange {
	pub fn new() -> Self {
		Self { fruit: Fruit {
			name: "Orange",
			color:Color::Rgb { r:242, g:98, b:46 }, bg_color:Color::White,
			pattern:vec!(
				"00111100",
				"01110110",
				"01111110",
				"00111100"
			)
		} }
	}
}

pub struct Star { fruit: Fruit }
impl FruitDrawable for Star { fn fruit(&self) -> &Fruit {&self.fruit} }
impl Star {
	pub fn new() -> Self {
		Self { fruit: Fruit {
			name: "Star",
			color:Color::DarkGrey, bg_color:Color::White,
			pattern:vec!(
				"00011000",
				"01111110",
				"00111100",
				"01100110"
			)
		} }
	}
}

pub struct Seven { fruit: Fruit }
impl FruitDrawable for Seven { fn fruit(&self) -> &Fruit {&self.fruit} }
impl Seven {
	pub fn new() -> Self {
		Self { fruit: Fruit {
			name: "Seven",
			color:Color::Magenta, bg_color:Color::White,
			pattern:vec!(
				"01111110",
				"00001110",
				"00111000",
				"01110000"
			)
		} }
	}
}

pub struct Blur { fruit: Fruit }
impl FruitDrawable for Blur { fn fruit(&self) -> &Fruit {&self.fruit} }
impl Blur {
	pub fn new() -> Self {
		Self { fruit: Fruit {
			name: "Blur",
			color:Color::DarkGrey, bg_color:Color::White,
			pattern:vec!(
				"01000000",
				"01010000",
				"00010100",
				"00000100"
			)
		} }
	}
}

pub trait FruitDrawable {
	fn draw_fruit_at(&self, x:usize,y:i32,y_offset:i32,y_clip_top:usize,y_clip_bottom:usize,frame:&mut Frame) {
		for (row,row_pattern) in self.pattern().iter().enumerate() {
			if row>=y_clip_top && row<self.pattern().len()-y_clip_bottom {
				for (col,c) in row_pattern.bytes().enumerate() {
					frame.set_pixel(x+col,(y+(row as i32)+y_offset).try_into().unwrap()," ".to_string(),Color::Black,
													if c==b'1' { *self.color() } else { *self.bg_color() });
				}
			}
		}
	}
	fn color(&self) -> &Color { &self.fruit().color }
	fn bg_color(&self) -> &Color { &self.fruit().bg_color }
	fn pattern(&self) -> &Vec<&'static str> { &self.fruit().pattern }
	fn fruit(&self) -> &Fruit;
	fn name(&self) -> &'static str { &self.fruit().name }
}
