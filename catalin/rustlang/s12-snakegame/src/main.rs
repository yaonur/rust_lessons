extern crate piston_window;
extern crate rand;

mod draw;
mod game;

use draw::to_coord_u32;
use piston_window::types::Color;
use piston_window::*;
const BACK_COLOR:Color =[0.5,0.5,0.5,1.0];
const FOOD_COLOR:Color =[0.8,0.0,0.0,1.0];
fn main() {
    let (width,height)=(30,30);
    let mut window:PistonWindow  = WindowSettings::new("Snake",[to_coord_u32(width),to_coord_u32(height)])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game=game::Game::new(width,height);

    while let Some(event) = window.next(){
        window.draw_2d(&event,|c,g,_|{
            clear(BACK_COLOR,g);
            game.draw(&c,g);
        });
        event.update(|arg|{
            game.update(arg.dt);
        });
    }
}
