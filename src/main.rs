extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;//{EventSettings, Events};
use piston::input::*;//{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, MouseCursorEvent};
use piston::window::{Window, WindowSettings};
use graphics::math::{identity};

mod types;
mod player;

use types::*;
use player::*;

// According to the "World" coordinates:
//   - (0,0) is the center of screen
//   - increasing y moves UP
// According to screen coordinates:
//   - (width/2,height/2) is the center of screen
//   - increasing y moves DOWN
fn to_world_position(center: ScreenPosition, pos: ScreenPosition) -> WorldPosition {
    [pos[0] - center[0], center[1] - pos[1]]
}

pub struct App {
    gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs, center: ScreenPosition, player: &Player) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 0.6, 0.0, 1.0];
        const RED: [f32; 4] = [0.8, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 0.8, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BROWN: [f32; 4] = [0.7, 0.3, 0.0, 1.0];

        let rotation = player.rotation;
        let [px,py] = player.position;
        let [cx,cy] = center;
        let [x, y] = [cx + px, cy - py];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            let screen_transform = c
                .transform
                .trans(cx, cy)
                .flip_v();

            let player_transform = identity()
                .trans(px, py)
                .rot_rad(rotation)
                .prepend_transform(screen_transform);

            let front = [15.0, -25.0, 10.0, 50.0];
            let sword = [5.0, -5.0, 60.0, 10.0];
            let body = [-25.0, -25.0, 40.0, 50.0];            

            let pillar = [100.0, 100.0, 50.0, 50.0];

            rectangle(BROWN, pillar, screen_transform, gl);

            rectangle(RED, front, player_transform, gl);
            rectangle(BLUE, body, player_transform, gl);
            if player.attacking {
                rectangle(WHITE, sword, player_transform, gl);
            }
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Delver", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut player = Player::new();
    let mut events = Events::new(EventSettings::new());
    let mut dir = Direction {
        up: false,
        down: false,
        left: false,
        right: false,
    };
    while let Some(e) = events.next(&mut window) {
        let center = || {
            let size = window.size();
            [size.width / 2.0, size.height / 2.0]
        };

        if let Some(args) = e.render_args() {
            app.render(&args, center(), &player);
        }

        if let Some(args) = e.update_args() {
            player.update(args.dt, &dir);
        }


        if let Some(Button::Mouse(button)) = e.press_args() {
            if MouseButton::Left == button {
                player.attack();
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::W {
                dir.up = true;
            }
            if key == Key::S {
                dir.down = true;
            }
            if key == Key::A {
                dir.left = true;
            }
            if key == Key::D {
                dir.right = true;
            }
        };

        if let Some(Button::Keyboard(key)) = e.release_args() {
            if key == Key::W {
                dir.up = false;
            }
            if key == Key::S {
                dir.down = false;
            }
            if key == Key::A {
                dir.left = false;
            }
            if key == Key::D {
                dir.right = false;
            }
        };

        e.mouse_cursor(|pos| {
            let pos = to_world_position(center(), pos);
            player.rotate_from_pos(pos);
        });
    }
}