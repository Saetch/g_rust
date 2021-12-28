use std::{sync::{Arc, Mutex, RwLock}, f64::consts::PI};
use crate::glorper_line::GlorperLine;
use graphics::math::Vec2d;
use piston::{UpdateArgs, Position};
pub struct Model{
    //Arc -> atomically reference counted, used to share data between threads, mutex for MUTability and thread safety (rust enforces thread safety or it throws)
    pub ball_pos: Arc<Mutex<(f64, f64)>>,
    ball_mov_vec: Vec2d,
    //RwLock makes multiple reads to shared data simultaneously possible. Write access is blocked, tho.
    pub elements: Arc<RwLock<Vec<GlorperLine>>>,

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

    pub fn debug_rad_action(&self){
        let mut _state = 0;
        {
            //This was put into this extra scope that ends before state is processed, so the readval lock is undone before continuing
           let readval = self.elements.read().unwrap(); //Rwlock permits mutiple readers, but only one writer, so trying with read first, prevents other threads from waiting
           if readval.len() > 0{
             let line = &readval[readval.len()-1];
            let xdiff = line.end.0 - line.start.0;
            let ydiff = line.end.1 - line.start.1;
            let len = (xdiff*xdiff + ydiff*ydiff).sqrt();
            println!("degrees arcsin: {}  :   {}", (xdiff/len).asin()* 180.0f64/PI, xdiff/len);
            println!("degrees arcos: {}  :   {}", (ydiff/len).acos()*180f64/PI, ydiff/len );
     /*       for i in -100 ..=100{                         //DEBUG: show asin in degrees and asin for -1 to 1 sin value;
                println!("asin: {} otig: {}", (0.01f64* i as f64).asin() *360.0f64/(2.0f64*PI) , (0.01f64* i as f64).asin());
            }*/
           }

           if readval.len() < 4 {
            _state = 1;
            if readval.len() == 0{
                _state = 2;
            }
        }
        else{
            return;
        }
        }
        if _state == 2{
            let mut mutval = self.elements.write().unwrap();
            mutval.push(GlorperLine{ start: (400.0f64, 400f64), end : ( 300f64, 300f64) });
            return;
        }else if _state == 1{
            let mut mutval = self.elements.write().unwrap();

            if mutval.len() == 1{
                mutval.push(GlorperLine{ start: (400.0f64, 400f64), end : ( 500f64, 300f64) });
                return;
            }
            if mutval.len() == 2{
                mutval.push(GlorperLine{ start: (400.0f64, 400f64), end : ( 500f64, 500f64) });
                return;
            }
            if mutval.len() == 3{
                mutval.push(GlorperLine{ start: (400.0f64, 400f64), end : ( 300f64, 500f64) });
                return;
            }
        }
        else{
            let mut mutval = self.elements.write().unwrap();
            let el = &mut mutval[0];

            if el.end.0 <= 300.0f64{
                if el.end.1 <= 300.0f64{
                    el.end.0 += 5.0f64;
                }
            }
        }

    }
}