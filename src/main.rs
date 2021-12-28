extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod view;
mod model;
mod constants;
mod controller;
mod glorper_line;

use std::sync::{Arc, Mutex};
use std::time::Instant;
use constants::{FIELDWIDTH, FIELDHEIGHT};
use controller::Controller;
use glutin_window::GlutinWindow;
use model::Model;
use opengl_graphics::{ OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent, ButtonEvent};
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
    let mut view;
    //WIDTH and HEIGHT are defined in constants.rs
    let model =  Arc::new(Mutex::new(Model::new( (450.0f64, 400.0f64))));
    {
        let unwrap_model = model.lock().unwrap();                           //mutex gets automatically unlocked, when out of scope, so these parenthesis: {} are used to make a new scope, but view is declared above and thus is only changed here        
        view = PistonView::new( opengl, &unwrap_model.ball_pos , &unwrap_model.elements); //if you were to use two model.lock().unwrap()s in this declaration, the application would deadlock
    }
    let mut controller = Controller::new(&model);
    //TODO: split rendering and game logic to different threads. Check if this is possible with channels(sync), otherwise might use shared message buffer that is continuously read from. Compare performance if needed
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            view.render(&args);
            let elapsed = start.elapsed();
            //println!("Elapsed time: {}ms", elapsed.as_millis());              //This gets called 60 times per second (60fps), if the elapsed time shown is >17, the fps dropped
            start = Instant::now();
            
            continue;                                      //SKIP the other possible updateArgs, because only one can be valid
        }

        if let Some(args) = e.update_args() {
            model.lock().unwrap().update(&args);
            continue;                                     //SKIP the other possible updateArgs, because only one can be validoo
        }

        if let Some(args) = e.button_args(){
            controller.compute_input(&args);
            continue;                                     //SKIP the other possible updateArgs, because only one can be validoo
        }
    }
}