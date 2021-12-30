use std::{sync::{Arc, Mutex, RwLock},
 thread, time::Duration};
use crate::{ gerade::Gerade, constants::{FIELDWIDTH, FIELDHEIGHT, SPAWN_SIDES_WITH_DELAY}};
use graphics::math::Vec2d;
use piston::{UpdateArgs};
pub struct Model{
    //Arc -> atomically reference counted, used to share data between threads, mutex for MUTability and thread safety (rust enforces thread safety or it throws)
    pub ball_pos: Arc<Mutex<(f64, f64)>>,
    pub ball_mov_vec: Arc<RwLock<Vec2d>>,           //this is not necessary for single use, but it makes calling multiple references of model simultaneously possible, as the model does not change, only the arc references do
    //RwLock makes multiple reads to shared data simultaneously possible. Write access is blocked, tho.
    pub elements: Arc<RwLock<Vec<Gerade>>>,

}

impl Model {
    pub fn new( o: (f64, f64)) -> Self{
        Model{
            ball_mov_vec: Arc::new(RwLock::new(Vec2d::from( [40.0f64, -40.0f64]))),
            ball_pos: Arc::new(Mutex::new(o)),
            elements: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn update(& self, args : &UpdateArgs){
        let mut pos = self.ball_pos.lock().unwrap();
        let ball_mov_vec = *self.ball_mov_vec.read().unwrap();
        pos.0+= ball_mov_vec[0]*args.dt;
        pos.1+= ball_mov_vec[1]*args.dt;
    }

    pub fn spawn_sides(& self){
        //TODO, this falls back to debug_rad, if actually implemented, update this
        for _ in 0..(FIELDWIDTH/25.0f64).floor() as usize *2 + (FIELDHEIGHT/25.0).floor() as usize *2{
            thread::sleep(Duration::from_millis(SPAWN_SIDES_WITH_DELAY.into()));
            self.debug_rad_action();
        }
        let mut mutval = self.elements.write().unwrap();
        for grad in &mut *mutval{
            grad.normalize();
        }
        drop(mutval);                           //dropping the value means releasing the lock. This isn't necessary from a functional perspective, but it will make things faster, if there is more to follow, because other threads can pick up faster. Alternatively could have put this in {} parenthesis and not used drop()
    }

    pub fn debug_rad_action(& self){
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