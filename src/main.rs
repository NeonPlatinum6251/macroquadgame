use macroquad::{color, prelude::*};
use macroquad::rand::ChooseRandom;
const MOVEMENT_SPEED:f32 = 200.0;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    colour:Color,
    collided:bool
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
    let mut gameover:bool = false;
    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x:screen_width() / 2.0,
        y:screen_height()/2.0,
        colour: GREEN,
    };
    let colours:Vec<Color> = vec![GREEN,PURPLE,BLUE,BLACK,PINK];
    loop {
        clear_background(RED);
        if !gameover {
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
                });
            }
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }
        
            squares.retain(|square| square.y < screen_height() + square.size);    
        }
        if squares.iter().any(|square|circle.collides_with(square)) {
            gameover = true;
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
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
    
        draw_circle(circle.x, circle.y, circle.size, YELLOW);

        if gameover {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }
        next_frame().await
    }
}

