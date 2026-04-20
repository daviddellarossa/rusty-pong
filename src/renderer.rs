use macroquad::prelude::*;
use crate::constants::*;
use crate::game::{Game, GamePhase};

pub fn draw(game: &Game){
    clear_background(BLACK);
    draw_net();
    draw_scores(game.score_l, game.score_r);
    game.pl.draw();
    game.pr.draw();
    if game.phase != GamePhase::StartScreen {
        game.ball.draw();
    }
}

fn draw_net(){
    let x = SCREEN_WIDTH as f32 / 2.0 - NET_WIDTH / 2.0;
    let mut y = 0f32;
    while y < SCREEN_HEIGHT as f32 {
        draw_rectangle(x, y, NET_WIDTH, NET_DASH_HEIGHT, GRAY);
        y += NET_DASH_HEIGHT + NET_DASH_GAP;
    }
}

fn draw_scores(score_l: u32, score_r: u32){
    let font_size = 64u16;
    let y = SCREEN_HEIGHT as f32 / 4.0;
    draw_text_centered(&score_l.to_string(), SCREEN_WIDTH as f32 / 4.0, y, font_size, WHITE);
    draw_text_centered(&score_r.to_string(), SCREEN_WIDTH as f32 * 3.0 / 4.0, y, font_size, WHITE);
}

fn draw_text_centered(text: &str, x: f32, y: f32, font_size: u16, color: Color){
    let dims = measure_text(text, None, font_size, 1.0);
    draw_text(text, x - dims.width / 2.0, y, font_size as f32, color);
}