use std::{sync::{Arc, Mutex, RwLock}, f64::consts::PI, thread, time::Duration};
use crate::{ gerade::Gerade, constants::{FIELDWIDTH, FIELDHEIGHT, SPAWN_SIDES_WITH_DELAY}};
use graphics::math::Vec2d;
use piston::{UpdateArgs, Position};
pub struct Model{
    //Arc -> atomically reference counted, used to share data between threads, mutex for MUTability and thread safety (rust enforces thread safety or it throws)
    pub ball_pos: Arc<Mutex<(f64, f64)>>,
    ball_mov_vec: Vec2d,
    //RwLock makes multiple reads to shared data simultaneously possible. Write access is blocked, tho.
    pub elements: Arc<RwLock<Vec<Gerade>>>,

}

impl Model {
    pub fn new( o: (f64, f64)) -> Self{
        Model{
            ball_mov_vec: Vec2d::from( [10.0f64, -10.0f64]),
            ball_pos: Arc::new(Mutex::new(o)),
            elements: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn update(&mut self, args : &UpdateArgs){
        let mut pos = self.ball_pos.lock().unwrap();
    }

    pub fn spawn_sides(&mut self){
        //TODO, this falls back to debug_rad, if actually implemented, update this
        for _ in 0..(FIELDWIDTH/25.0f64).floor() as usize *2 + (FIELDHEIGHT/25.0).floor() as usize *2{
            thread::sleep(Duration::from_millis(SPAWN_SIDES_WITH_DELAY.into()));
            self.debug_rad_action();
        }
    }

    pub fn debug_rad_action(&mut self){
        let mut _state = 0;
        {
            //This was put into this extra scope that ends before state is processed, so the readval lock is undone before continuing
           let readval = self.elements.read().unwrap(); //Rwlock permits mutiple readers, but only one writer, so trying with read first, prevents other threads from waiting
            if readval.len() >=(FIELDWIDTH/25.0f64).floor() as usize *2 + (FIELDHEIGHT/25.0).floor() as usize *2{
                return ;
            }
        }
        let mut mutval = self.elements.write().unwrap();
        if mutval.len() < (FIELDWIDTH / 25.0f64).floor() as usize {
            if mutval.len() == 0{
                let start = (0.0, 0.0);
                let end = (25.0, 0.0);
                mutval.push(Gerade::from_two_points(start, end));
                return;
            }
            let end_punkt = mutval.last().unwrap().end_punkt;
            mutval.push(Gerade::from_two_points(end_punkt, (end_punkt.0 +25.0, 0.0)));
            return;
        }
        if mutval.len() < (FIELDWIDTH/25.0f64).floor() as usize + (FIELDHEIGHT/25.0).floor() as usize {
            let end_punkt = mutval.last().unwrap().end_punkt;

            mutval.push(Gerade::from_two_points(end_punkt, (end_punkt.0, end_punkt.1+25.0)));
            return;
        }
        if mutval.len() < (FIELDWIDTH/25.0f64).floor() as usize *2 + (FIELDHEIGHT/25.0).floor() as usize {
            let end_punkt = mutval.last().unwrap().end_punkt;

            mutval.push(Gerade::from_two_points(end_punkt, (end_punkt.0 -25.0, end_punkt.1)));
            return;
        }
        
        let end_punkt = mutval.last().unwrap().end_punkt;

        mutval.push(Gerade::from_two_points(end_punkt, (end_punkt.0 , end_punkt.1 -25.0)));

    }
}