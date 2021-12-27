use std::sync::{Arc, Mutex, RwLock};
use crate::glorper_line::GlorperLine;
use piston::UpdateArgs;
pub struct Model{
    //Arc -> atomically reference counted, used to share data between threads, mutex for MUTability and thread safety (rust enforces thread safety or it throws)
    pub ball_pos: Arc<Mutex<(f32, f32)>>,
    ball_dir: (f32, f32),
    pub rotation: Arc<Mutex<f64>>,
    //RwLock makes multiple reads to shared data simultaneously possible. Write access is blocked, tho.
    pub elements: Arc<RwLock<Vec<GlorperLine>>>,
}

impl Model {
    pub fn new( o: (f32, f32), dir: (f32, f32)) -> Self{
        Model{
            ball_dir: dir,
            ball_pos: Arc::new(Mutex::new(o)),
            rotation: Arc::new(Mutex::new(0.0f64)),
            elements: Arc::new(RwLock::new(Vec::new())),
        }
    }
    pub fn update(&mut self, args : &UpdateArgs){
        *self.rotation.lock().unwrap() += 2.0 * args.dt;
    }

    pub fn debug_rad_action(&self){
        println!("hit!");
    }
}