use piston_window::types::Color;
use piston_window::*;

#[allow(unused_imports)]
use rand::{thread_rng,Rng};
#[allow(unused_imports)]
use crate::draw::{draw_block,draw_rectangle};

const BORDER_COLOR:Color=[0.0,0.0,0.0,1.0];

pub struct Game {
	width:i32,
	height:i32,

}

impl Game {
	pub fn new(width:i32,height:i32)-> Game{
		Game{
			width,
			height,
		}
	}

	pub fn draw(&self,con:&Context,g:&mut G2d){
		draw_rectangle(BORDER_COLOR,0,0,self.width,self.height,con,g);
		draw_rectangle(BORDER_COLOR,1,1,self.height-1,self.width,con,g);
		draw_rectangle(BORDER_COLOR,0,0,1,self.height,con,g);
		draw_rectangle(BORDER_COLOR,self.width-1,0,1,self.height,con,g);
	}

	#[allow(unused_variables)]
	pub fn update(&mut self,delta_time:f64){

	}
}