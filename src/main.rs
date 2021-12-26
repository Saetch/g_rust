extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod view;
mod model;
mod constants;

use std::time::Instant;
use constants::{FIELDWIDTH, FIELDHEIGHT};
use glutin_window::GlutinWindow;
use model::Model;
use opengl_graphics::{ OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use view::PistonView;



fn main() {

        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let mut window: GlutinWindow = WindowSettings::new("glorper!", [FIELDWIDTH as u32 +40, FIELDHEIGHT as u32 +40])
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());
    let mut start = Instant::now();
    let mut model =  Model::new(800.0f32, 800.0f32, (400.0f32, 600.0f32), (10.0f32, 10.0f32));
    let mut view = PistonView::new(&model.rotation, opengl, &model.ball_pos);

    //TODO: split rendering and game logic to different threads. Check if this is possible with channels(sync)
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            //This gets called 60 times per second (60fps), if the elapsed time shown is >17, the fps dropped
            //app.render(&args);
            view.render(&args);
            let elapsed = start.elapsed();
            println!("Elapsed time: {}ms", elapsed.as_millis());
            start = Instant::now();
    
        }

        if let Some(args) = e.update_args() {
            model.update(&args);


        }
    }
}