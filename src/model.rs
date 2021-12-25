use std::sync::{Arc, Mutex};

use piston::UpdateArgs;
pub struct Model{
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