mod constants;
mod paddle;
mod ball;
mod collision;
mod game;
mod renderer;

use constants::*;
use macroquad::prelude::*;
use crate::game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Pong".to_owned(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    
    loop {
        // move things, check collisions, update score, etc.
        game.update();
        
        // draw
        renderer::draw(&game);

        next_frame().await;
    }
}
