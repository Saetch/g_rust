extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::time::Instant;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        //RGBA color definition in array: red, green, blue, alpha (1- opacity)
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let rotation = self.rotation;
        //place it at x,y, in this case in the middle: args.window_size[0] -> width, args.window_size[1] -> height
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let square = graphics::rectangle::square(0.0, 0.0, 50.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
//transformations are calculatedfor the viewPort. This means, that the center of the screen will be moved to x,y, then 
//rotated, then offset an then the square is drawn with the top left corner at the given point. Then the screen is reset to the default
//position
            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);               

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });

    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let mut window: GlutinWindow = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    let mut start = Instant::now();
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            //This gets called 60 times per second (60fps), if the elapsed time shown is >17, the fps dropped
            app.render(&args);
    
        }

        if let Some(args) = e.update_args() {
            app.update(&args);  
            let elapsed = start.elapsed();
            println!("Elapsed time: {}ms", elapsed.as_millis());
            start = Instant::now();

        }
    }
}