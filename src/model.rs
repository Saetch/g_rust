use std::sync::{Arc, Mutex};

use piston::UpdateArgs;
pub struct Model{
    //Arc for sharing data between threads (not needed in pistonSquare, as it is singleThreaded, but I wanted to add it, so we don't 
    //need to change it later), arc-> atomically reference counted. Mutex gives MUTability and is thread safe (sync), see use below
    pub rotation: Arc<Mutex<f64>>
}

impl Model {
    pub fn new() -> Self{
        Model{
            rotation: Arc::new(Mutex::new(0.0f64)),
        }
    }
    pub fn update(&mut self, args : &UpdateArgs){
        *self.rotation.lock().unwrap() += 2.0 * args.dt;
    }
}