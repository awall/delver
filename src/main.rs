extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;//{EventSettings, Events};
use piston::input::*;//{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, MouseCursorEvent};
use piston::window::{Window, WindowSettings};


type Angle = f64;
type WorldPosition = [f64; 2];
type ScreenPosition = [f64; 2];

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
    rotation: Angle,
    position: WorldPosition,
}

impl App {
    fn render(&mut self, args: &RenderArgs, center: ScreenPosition) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 0.6, 0.0, 1.0];
        const RED: [f32; 4] = [0.8, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 0.8, 1.0];

        let rotation = self.rotation;
        let [px,py] = self.position;
        let [cx,cy] = center;
        let [x, y] = [cx + px, cy - py];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            let front = [40.0, 0.0, 10.0, 50.0];
            let body = [0.0, 0.0, 40.0, 50.0];

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, front, transform, gl);
            rectangle(BLUE, body, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs, walk: bool) {
        if !walk {
            return;
        }

        let x = self.rotation.cos();
        let y = -self.rotation.sin();
        self.position[0] += x * 100.0 * args.dt;
        self.position[1] += y * 100.0 * args.dt;
    }

    fn rotate_from_pos(&mut self, center: ScreenPosition, pos: ScreenPosition) {
        let pos = to_world_position(center, pos);
        let x = pos[0] - self.position[0];
        let y = pos[1] - self.position[1];
        self.rotation = -y.atan2(x);
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
        rotation: 0.0,
        position: [0.0, 0.0],
    };

    let mut events = Events::new(EventSettings::new());
    let mut walk = false;
    while let Some(e) = events.next(&mut window) {
        let center = || {
            let size = window.size();
            [size.width / 2.0, size.height / 2.0]
        };

        if let Some(args) = e.render_args() {
            app.render(&args, center());
        }

        if let Some(args) = e.update_args() {
            app.update(&args, walk);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::W {
                walk = true;
            }
        };

        if let Some(Button::Keyboard(key)) = e.release_args() {
            if key == Key::W {
                walk = false;
            }
        };

        e.mouse_cursor(|pos| {
            app.rotate_from_pos(center(), pos);
        });
    }
}