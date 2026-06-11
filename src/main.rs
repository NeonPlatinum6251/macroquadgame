
use macroquad::{prelude::*};
use macroquad::rand::ChooseRandom;
use std::fs;
use macroquad_particles::{self as particles, AtlasConfig, Emitter, EmitterConfig};
use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::audio::{load_sound, play_sound, play_sound_once, PlaySoundParams};
use macroquad::ui::{hash, root_ui, Skin};

/*TODO:
music for different gamestates
volume button
exit the game from anywhere 
add a pause menu 
 */

const MOVEMENT_SPEED:f32 = 200.0;
const FRAGMENT_SHADER: &str = include_str!("starfield-shader.glsl");

const VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying float iTime;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 _Time;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;
}
";

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

fn particle_explosion() ->particles::EmitterConfig {
    particles::EmitterConfig {
        local_coords: false,
        one_shot: true,
        emitting: true,
        lifetime: 0.6,
        lifetime_randomness:0.3,
        explosiveness: 0.65,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 400.0,
        initial_velocity_randomness: 0.8,
        size: 16.0,
        size_randomness: 0.3,
        atlas: Some(AtlasConfig::new(5,1,0..)),
        ..Default::default()
    }
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
    set_pc_assets_folder("assets");

    let mut game_state = GameState::MainMenu;
    let mut squares = vec![];
    let mut bullets: Vec<Shape> = vec![];
    let mut lastshot = get_time();
    let mut score:u32 = 0;
    let mut highscore:u32 = fs::read_to_string("highscore.dat")
    .map_or(Ok(0), |i| i.parse::<u32>())
    .unwrap_or(0);

    let theme_music = load_sound("8bit-spaceshooter.ogg").await.unwrap();
    let sound_explosion = load_sound("explosion.wav").await.unwrap();
    let sound_laser = load_sound("laser.wav").await.unwrap();
    
    let ship_texture: Texture2D = load_texture("ship.png").await.expect("Couldnt'load file");
    ship_texture.set_filter(FilterMode::Nearest);
    let bullet_texture:Texture2D = load_texture("laser-bolts.png").await.expect("Couldnt load file");
    bullet_texture.set_filter(FilterMode::Nearest);
    let explosion_texture: Texture2D = load_texture("explosion.png").await.expect("Couldn't load file");
    explosion_texture.set_filter(FilterMode::Nearest);
    let enemy_small_texture: Texture2D = load_texture("enemy-small.png").await.expect("Couldnt load file");
    enemy_small_texture.set_filter(FilterMode::Nearest);
    build_textures_atlas();

    let mut bullet_sprite = AnimatedSprite::new(
        16,
        16,
        &[
            Animation {
                name: "bullet".to_string(),
                row:0,
                frames:2,
                fps:12,
            },
            Animation {
                name: "bolt".to_string(),
                row:1,
                frames:2,
                fps:12,
            },
        ],
        true,
    );
    bullet_sprite.set_animation(1);

    let mut ship_sprite = AnimatedSprite::new(
        16,
        24,
        &[
            Animation {
            name: "idle".to_string(),
            row:0,
            frames:2,
            fps:12,
            },
            Animation {
                name: "left".to_string(),
                row:2,
                frames:2,
                fps:12,
            },
            Animation {
                name: "right".to_string(),
                row:4,
                frames:2,
                fps:12,
            },
        ],
        true
    );

    let mut small_enemy_sprite = AnimatedSprite::new(
        17,
        16,
        &[Animation {
            name:"enemy_small".to_string(),
            row:0,
            frames:2,
            fps:12,
        }],
        true,
    );

    let window_background = load_image("window_background.png").await.unwrap();
    let button_background = load_image("button_background.png").await.unwrap();
    let button_clicked_background = load_image("button_clicked_background.png").await.unwrap();
    let font = load_file("atari_games.ttf").await.unwrap();

    let window_style = root_ui()
        .style_builder()
        .background(window_background)
        .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
        .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
        .build();

    let button_style = root_ui()
        .style_builder()
        .background(button_background)
        .background_clicked(button_clicked_background)
        .background_margin(RectOffset::new(16.0,16.0,16.0,16.0))
        .margin(RectOffset::new(16.0, 0.0, -8.0,-8.0))
        .font(&font)
        .unwrap()
        .text_color(WHITE)
        .font_size(64)
        .build();

    let label_style = root_ui()
        .style_builder()
        .font(&font)
        .unwrap()
        .text_color(WHITE)
        .font_size(28)
        .build();

    let ui_skin = Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);
    let window_size = vec2(370.0,320.0);


    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x:screen_width() / 2.0,
        y:screen_height()/2.0,
        colour: GREEN,
        collided:false,
    };

    let colours:Vec<Color> = vec![GREEN,PURPLE,BLUE,RED,PINK];
    
    let mut direction_modifier:f32 = 0.0;
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);
    let material = load_material(
        ShaderSource::Glsl { 
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER 
        },
        MaterialParams {  
            uniforms: vec![
                UniformDesc::new("iResolution",UniformType::Float2),
                UniformDesc::new("direction_modifier",UniformType::Float1),
            ], 
            ..MaterialParams::default() 
        },
    ).unwrap();

    let mut explosions:Vec<(Emitter, Vec2)> = vec![];

    play_sound(
        &theme_music,
        PlaySoundParams {
            looped:true,
            volume:1.,
        }
        );

    loop {
        clear_background(BLACK);

        material.set_uniform("iResolution", (screen_width(),screen_height()));
        material.set_uniform("direction_modifier", direction_modifier);
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size:Some(vec2(screen_width(),screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();

        match game_state {
            GameState::MainMenu => {
                root_ui().window(
                    hash!(),
                    vec2(
                        screen_width() / 2.0 - window_size.x / 2.0,
                        screen_height() / 2.0 - window_size.y / 2.0,
                    ),
                    window_size,
                    |ui| {
                        ui.label(vec2(80.0, -34.0), "Main Menu");
                        if ui.button(vec2(65.0,25.0),"Play") {
                            squares.clear();
                            bullets.clear();
                            explosions.clear();
                            circle.x = screen_width() / 2.0;
                            circle.y = screen_height() / 2.0;
                            score = 0;
                            game_state = GameState::Playing;
                        }
                        if ui.button(vec2(65.0,125.0), "Quit") {
                            std::process::exit(0);
                        }
                    },
                );
            },
            GameState::Playing => {
                let delta_time = get_frame_time();
                ship_sprite.set_animation(0);
                if is_key_down(KeyCode::Right) {
                    circle.x += circle.speed * delta_time;
                    direction_modifier += 0.05*delta_time;
                    ship_sprite.set_animation(2);
                }
                if is_key_down(KeyCode::Left) {
                    circle.x -= circle.speed * delta_time;
                    direction_modifier -= 0.05*delta_time;
                    ship_sprite.set_animation(1);
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
                            y:circle.y - 24.0,
                            size:32.0,
                            speed:circle.speed*2.0,
                            collided:false,
                            colour:WHITE,
                        });
                        lastshot = get_time();
                        play_sound_once(&sound_laser);
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
                ship_sprite.update();
                bullet_sprite.update();
                small_enemy_sprite.update();

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
                            explosions.push((
                                Emitter::new(EmitterConfig {
                                    amount: square.size.round() as u32 * 4,
                                    texture: Some(explosion_texture.clone()),
                                    ..particle_explosion()
                                }),
                                vec2(square.x,square.y),
                            ));
                            play_sound_once(&sound_explosion);
                        }
                    }
                }

                explosions.retain(|(explosions,_)| explosions.config.emitting);
                let enemy_frame = small_enemy_sprite.frame();
                for square in &squares {
                    draw_texture_ex(
                        &enemy_small_texture,
                        square.x -square.size / 2.0,
                        square.y -square.size / 2.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(square.size, square.size)),
                            source: Some(enemy_frame.source_rect),
                            ..Default::default()
                        },
                    );
                }
                let bullet_frame = bullet_sprite.frame();
                for bullet in &bullets {
                    draw_texture_ex(
                        &bullet_texture,
                        bullet.x - bullet.size / 2.0,
                        bullet.y - bullet.size/ 2.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(bullet.size, bullet.size)),
                            source: Some(bullet_frame.source_rect),
                            ..Default::default()
                        },
                    );
                };

                let ship_frame = ship_sprite.frame();
                draw_texture_ex(
                    &ship_texture,
                    circle.x - ship_frame.dest_size.x,
                    circle.y - ship_frame.dest_size.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(ship_frame.dest_size * 2.0),
                        source: Some(ship_frame.source_rect),
                        ..Default::default()
                    },
                );
                               
                draw_text(
                    format!("Score: {}",score).as_str(),
                    10.0,
                    35.0,
                    25.0,
                    WHITE,
                );

                for (explosion, coords) in explosions.iter_mut() {
                    explosion.draw(*coords);
                }

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
