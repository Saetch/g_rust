use std::sync::{Arc, Mutex, RwLock};
use crate::glorper_line::GlorperLine;
use piston::UpdateArgs;
pub struct Model{
    //Arc -> atomically reference counted, used to share data between threads, mutex for MUTability and thread safety (rust enforces thread safety or it throws)
    pub ball_pos: Arc<Mutex<(f32, f32)>>,
    ball_dir: (f32, f32),
    //RwLock makes multiple reads to shared data simultaneously possible. Write access is blocked, tho.
    pub elements: Arc<RwLock<Vec<GlorperLine>>>,
}

impl Model {
    pub fn new( o: (f32, f32), dir: (f32, f32)) -> Self{
        Model{
            ball_dir: dir,
            ball_pos: Arc::new(Mutex::new(o)),
            elements: Arc::new(RwLock::new(Vec::new())),
        }
    }
    pub fn update(&mut self, args : &UpdateArgs){

    }

    pub fn debug_rad_action(&self){
        let mut state = 0;
        {
            //This was put into this extra scope that ends before state is processed, so the readval lock is undone before continuing
           let readval = self.elements.read().unwrap(); //Rwlock permits mutiple readers, but only one writer, so trying with read first, prevents other threads from waiting
           println!("length: {}", readval.len());
        if readval.len() < 4 {
            state = 1;
            if readval.len() == 0{
                state = 2;
            }
        }
        else{
            return;
        }
        }
        if state == 2{
            let mut mutval = self.elements.write().unwrap();
            mutval.push(GlorperLine{ start: (400.0f32, 400f32), end : ( 400f32, 300f32) });
            return;
        }else{
            let mut mutval = self.elements.write().unwrap();

            if mutval.len() == 1{
                mutval.push(GlorperLine{ start: (400.0f32, 400f32), end : ( 300f32, 400f32) });
                return;
            }
            if mutval.len() == 2{
                mutval.push(GlorperLine{ start: (400.0f32, 400f32), end : ( 500f32, 400f32) });
                return;
            }
            if mutval.len() == 3{
                mutval.push(GlorperLine{ start: (400.0f32, 400f32), end : ( 400f32, 500f32) });
                return;
            }
        }
    }
}