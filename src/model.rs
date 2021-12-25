use std::sync::{Arc, Mutex};

use piston::UpdateArgs;
pub struct Model{
    width : f32,
    height: f32,
    //Arc -> atomically reference counted, used to share data between threads, mutex for MUTability and thread safety (rust enforces thread safety or it throws)
    ball_pos: Arc<Mutex<(f32, f32)>>,
    ball_dir: (f32, f32),
    pub rotation: Arc<Mutex<f64>>
}

impl Model {
    pub fn new(w: f32, h:f32, o: (f32, f32), dir: (f32, f32)) -> Self{
        Model{
            width : w,
            height: h,
            ball_dir: dir,
            ball_pos: Arc::new(Mutex::new(o)),
            rotation: Arc::new(Mutex::new(0.0f64)),
        }
    }
    pub fn update(&mut self, args : &UpdateArgs){
        *self.rotation.lock().unwrap() += 2.0 * args.dt;
    }
}