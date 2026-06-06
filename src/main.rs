use macroquad::{prelude::*};
use macroquad::rand::ChooseRandom;
use std::fs;

const MOVEMENT_SPEED:f32 = 200.0;

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}
struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    colour:Color,
    collided:bool,
}
impl Shape {
    fn collides_with(&self,other:&Self) ->bool {
        self.rect().overlaps(&other.rect())
    }
    fn rect(&self) -> Rect {
        Rect {
            x:self.x - self.size/2.0,
            y:self.y - self.size/2.0,
            w:self.size,
            h:self.size,
            
        }
    }
}


#[macroquad::main("MyGame")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    let mut game_state = GameState::MainMenu;
    let mut squares = vec![];
    let mut bullets: Vec<Shape> = vec![];
    let mut lastshot = get_time();
    let mut score:u32 = 0;
    let mut highscore:u32 = fs::read_to_string("highscore.dat")
    .map_or(Ok(0), |i| i.parse::<u32>())
    .unwrap_or(0);
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x:screen_width() / 2.0,
        y:screen_height()/2.0,
        colour: GREEN,
        collided:false,
    };

    let colours:Vec<Color> = vec![GREEN,PURPLE,BLUE,BLACK,PINK];
    loop {
        clear_background(RED);
        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::Space) {
                    squares.clear();
                    bullets.clear();
                    circle.x = screen_width() / 2.0;
                    circle.y = screen_height() / 2.0;
                    score = 0;
                    game_state = GameState::Playing;
                }
                let text = "Press Space";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width()/2.0 - text_dimensions.width/2.0,
                    screen_height()/2.0,
                    50.0,
                    WHITE);
            },
            GameState::Playing => {
                let delta_time = get_frame_time();
                if is_key_down(KeyCode::Right) {
                    circle.x += circle.speed * delta_time;
                }
                if is_key_down(KeyCode::Left) {
                    circle.x -= circle.speed * delta_time;
                }
                if is_key_down(KeyCode::Down) {
                    circle.y += circle.speed * delta_time;
                }
                if is_key_down(KeyCode::Up) {
                    circle.y -= circle.speed * delta_time;
                }
                if is_key_pressed(KeyCode::Space) {
                    if get_time() - lastshot > 0.5 {
                        bullets.push(Shape {
                            x:circle.x,
                            y:circle.y,
                            size:5.0,
                            speed:circle.speed*2.0,
                            collided:false,
                            colour:WHITE,
                        });
                        lastshot = get_time();
                    }
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                circle.x = clamp(circle.x, 0.0, screen_width());
                circle.y = clamp(circle.y, 0.0, screen_height());

                if rand::gen_range(0, 99) >= 95 {
                    let size = rand::gen_range(16.0, 64.0);
                    let rand_colour = *colours.choose().unwrap();
                    squares.push(Shape {
                        size,
                        speed: rand::gen_range(50.0, 150.0),
                        x: rand::gen_range(size/2.0, screen_width()-size/2.0),
                        y: -size,
                        colour:rand_colour,
                        collided:false,
                    });
                }
                for square in &mut squares {
                    square.y += square.speed * delta_time;
                }
                for bullet in &mut bullets {
                    bullet.y -= bullet.speed *delta_time;
                }
        
                squares.retain(|square| square.y < screen_height() + square.size);
                bullets.retain(|bullet| bullet.y > 0.0 - bullet.size/2.0);
                squares.retain(|square| !square.collided);
                bullets.retain(|bullet| !bullet.collided);

                if squares.iter().any(|square|circle.collides_with(square)) {
                if score == highscore {
                    fs::write("highscore.dat", highscore.to_string()).ok();
                }
                game_state = GameState::GameOver;
                }

                for square in squares.iter_mut() {
                    for bullet in bullets.iter_mut() {
                        if bullet.collides_with(square) {
                            bullet.collided = true;
                            square.collided = true;
                            score +=square.size.round() as u32;
                            highscore = highscore.max(score);
                        }
                    }
                }
                for square in &squares {

                    draw_rectangle(
                        square.x - square.size/2.0, 
                        square.y - square.size/2.0, 
                        square.size, 
                        square.size,
                        square.colour,
                        );
                }
    
                for bullet in &bullets {
                    draw_circle(bullet.x, bullet.y, bullet.size/2.0, bullet.colour);
                }
                draw_circle(circle.x, circle.y, circle.size, YELLOW);
                draw_text(
                    format!("Score: {}",score).as_str(),
                    10.0,
                    35.0,
                    25.0,
                    WHITE,
                );
                let highscore_text = format!("High score: {}", highscore);
                let text_dimensions =measure_text(highscore_text.as_str(),None,25,1.0);
                draw_text(
                    highscore_text.as_str(),
                    screen_width() - text_dimensions.width - 10.0,
                    35.0,
                    25.0,
                    WHITE,
                );
            },
            GameState::Paused => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
                let text = "Paused";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width()/2.0 - text_dimensions.width/2.0,
                    screen_height()/2.0,
                    50.0,
                    WHITE
                );
            },
            GameState::GameOver => {
                 if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
                let text = "GAME OVER";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width()/2.0 - text_dimensions.width/2.0,
                    screen_height()/2.0,
                    50.0,
                    WHITE
                );
            },
        }
        next_frame().await
    }
}
