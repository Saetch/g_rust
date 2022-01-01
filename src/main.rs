extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod vect_2d;
mod view;
mod model;
mod constants;
mod controller;
mod gerade;
use std::sync::{Arc, Mutex, RwLock};
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


        let running = Arc::new(Mutex::new(true));


    //This is the event buffer, it gets constantly filled if an event occurs to the window and is read in the game loop
    let mut events = Events::new(EventSettings::new());
    //WIDTH and HEIGHT are defined in constants.rs, these are the original ball coordinates
    let model =  Arc::new(RwLock::new(Model::new( (300.0f64, 300.0f64))));   //model is mutexed, as it gets called from input aswell!
    model.write().unwrap().init_speed();
    let mut controller = Controller::new(&model);

    let mut view;
    {
        let unwrap_model = model.read().unwrap();                           //mutex gets automatically unlocked, when out of scope, so these parenthesis: {} are used to make a new scope, but view is declared above and thus is only changed here    (could also drop the parenthesis and use drop(unwrap_model) adter creating the view)    
        view = PistonView::new( opengl, &unwrap_model.ball_pos , &unwrap_model.elements); //if you were to use two model.lock().unwrap()s in this declaration, the application would deadlock
    }
    //CREATE a thread for rendering. Send render args via channel
    let (sendermodelx, receivermodelx) = flume::unbounded();
    let running_th = running.clone();  
    let model_thread = std::thread::spawn(move || {     //consider moving this whole chunk to its own dedicated function for visual clarity

        while *running_th.lock().unwrap() {
            let arg_p = receivermodelx.recv();
            if !arg_p.is_err(){
                let args = arg_p.unwrap();
                model.read().unwrap().update(&args);

            }

        }
        println!("Stopping model thread!");
    });

    let (controllersenderx, controllerreceiverx) = flume::unbounded();
    let running_th = running.clone();
    let controller_thread = std::thread::spawn(move ||{
        while *running_th.lock().unwrap() {
            let arg_p = controllerreceiverx.recv();
            if !arg_p.is_err(){
                let args = arg_p.unwrap();
                controller.compute_input(&args);
            }
        }
        println!("Stopping the Controller thread!");
    });



    let mut start = Instant::now();                         
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {

            view.render(&args);
            let elapsed = start.elapsed();
            //println!("Elapsed time: {}ms", elapsed.as_millis());              //This gets called 60 times per second (60fps), if the elapsed time shown is >17, the fps dropped
            start = Instant::now();  
            continue;                                      //SKIP the other possible updateArgs, because only one can be valid
        }

        if let Some(args) = e.update_args() {
            let ret = sendermodelx.try_send(args);
            if ret.is_err(){
                println!("Could not send args to model thread!");           //if the app still works when this occurs, this might mean that the buffer on model side is full and it might be behind schedule
            }
            continue;                                     //SKIP the other possible updateArgs, because only one can be validoo
        }

        if let Some(args) = e.button_args(){
            let ret = controllersenderx.try_send(args);
            if ret.is_err(){
                println!("Could not send args to controller thread!");      //if the app still works when this occurs, this might mean that the buffer on controller side is full and it might be behind schedule
            }
            continue;                                     //SKIP the other possible updateArgs, because only one can be validoo
        }
    }
    //The sender is dropped, so the model thread will receive an err on .reicv and thus terminate, when the value of k is dropped.
    drop(sendermodelx);
    drop(controllersenderx);
    let mut k = running.lock().unwrap();
    *k = false;
    drop(k);
    model_thread.join().unwrap();
    controller_thread.join().unwrap();
    println!("OK!");
}



