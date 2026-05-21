use macroquad::{color, prelude::*};
use macroquad::rand::ChooseRandom;
const MOVEMENT_SPEED:f32 = 200.0;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    colour:Color,
}


#[macroquad::main("MyGame")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
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
        let delta_time = get_frame_time();
        clear_background(RED);
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
        next_frame().await
    }
}

