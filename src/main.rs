use macroquad::prelude::*;
const MOVEMENT_SPEED:f32 = 200.0;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    //let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x:screen_width() / 2.0,
        y:screen_height()/2.0,
    };
    loop {
        let delta_time = get_frame_time();
        clear_background(RED);
        if is_key_down(KeyCode::Right) {
            x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            y -= MOVEMENT_SPEED * delta_time;
        }

        x = clamp(x, 0.0, screen_width());
        y = clamp(y, 0.0, screen_height());
        draw_circle(x, y, 16.0, YELLOW);
        next_frame().await
    }
}

