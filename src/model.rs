use std::{sync::{Arc, RwLock, RwLockWriteGuard},
 thread, time::Duration};
use crate::{ gerade::Gerade, constants::{FIELDWIDTH, FIELDHEIGHT, SPAWN_SIDES_WITH_DELAY, self, CIRCLERADIUS}, vect_2d::Vector2D};
use piston::{UpdateArgs};
pub struct Model{
    //Arc -> atomically reference counted, used to share data between threads, mutex for MUTability and thread safety (rust enforces thread safety or it throws)
    pub ball_pos: Arc<RwLock<(f64, f64)>>,
    pub ball_mov_vec: Arc<RwLock<Vector2D>>,           //this is not necessary for single use, but it makes calling multiple references of model simultaneously possible, as the model does not change, only the arc references do
    //RwLock makes multiple reads to shared data simultaneously possible. Write access is blocked, tho.
    pub elements: Arc<RwLock<Vec<Gerade>>>,

    speed: f64,                               //this speed value is used to keep the vector the same length after it has been mirrored. This is relevant because it could occur due to floating point arithmetics that it changes speed in a large amount of hits
    dummy_element: Arc<RwLock<(f64, f64)>>,   //holds the final point of the last added line

    //These were added later to replace single ball with multiple balls
    pub ball_mov_vectors: Arc<RwLock<Vec<RwLock<Vector2D>>>>,
    pub ball_positions: Arc<RwLock<Vec<RwLock<Vector2D>>>>,
}

impl Model {
    pub fn new( o: (f64, f64)) -> Self{
        Model{
            ball_mov_vec: Arc::new(RwLock::new(Vector2D { x: 200.0, y: -80.0 })),
            ball_pos: Arc::new(RwLock::new(o)),
            elements: Arc::new(RwLock::new(Vec::new())),
            dummy_element: Arc::new(RwLock::new((0.0,0.0))),
            ball_mov_vectors: Arc::new(RwLock::new(Vec::new())),
            ball_positions: Arc::new(RwLock::new(Vec::new())),
            speed: 0.0,
        }
    }

  /*   this is not working, if needed, update this and unquote
      pub fn order_elements_x_then_y(&self ){
        let mut vec_guard = self.elements.write().unwrap();
        vec_guard.sort_by(|a, b| a.start_punkt.0.cmp(b.start_punkt.0))
    }*/

    pub fn update(& self, args : &UpdateArgs){
        if self.speed > 0.0001{
            let pos = self.ball_pos.read().unwrap();
            let mut ball_mov_vec = self.ball_mov_vec.write().unwrap();                              //keeping this variable for longer is no problem, since the only thread that accesses it locking, is this thread
            let new_x = ball_mov_vec.x*args.dt + pos.0;
            let new_y = ball_mov_vec.y*args.dt + pos.1;
            let mut offset = (0.0, 0.0);
            let mut hitcount = 0;
            let mut hitpoints_vector :Vec<((f64,f64), f64)> = Vec::new();
            let default_dir_vec = ball_mov_vec.clone();
            let actualvector = self.elements.read().unwrap();
            drop(pos);      //this is dropped here, so the rendering thread does not delay if it tries to read the ballpos.
            'elementsloop:for element in &*actualvector{
                if (element.start_punkt.0 <= new_x + 50.0 && element.start_punkt.0 >= new_x - 50.0) &&  (element.start_punkt.1 <= new_y + 50.0 && element.start_punkt.1 >= new_y -50.0) {            //only precisely check for a hit, if the elements are close enough. For larger amounts of elements, try a dedicated data structure, which yields only the relevant elements
                        let mut new_point = (0.0, 0.0);
                        let ret = self.check_for_hit(element, &mut new_point);
                        if ret.0.is_some(){

                            if ret.1{
                                for g in  &hitpoints_vector{          //      check wether the point has been hit already. Needs some lee-way due to floating point conversion and polygons ending where others start
                                    if  f64::abs( g.0.0 - new_point.0 ) <2.5 && f64::abs( g.0.1 - new_point.1) < 2.5{ 
                                               if g.1 < ret.0.unwrap() {
                                                   continue 'elementsloop;
                                               }else{                       //a wrong hit was detected that should be overwritten
                                                    *ball_mov_vec = default_dir_vec.clone();
                                               }
                                       }
                                   }
                                    ball_mov_vec.mirror_on(element, self.speed);
                                    offset.0 = 2.0* ball_mov_vec.x*args.dt;
                                    offset.1 = 2.0* ball_mov_vec.y*args.dt;
                                    hitcount+=1;
                                    hitpoints_vector.push((new_point , ret.0.unwrap()));
                                

                            }
                            else{
                                    for g in  &hitpoints_vector{          //      check wether the point has been hit already. Needs some lee-way due to floating point conversion and polygons ending where others start
                                         if  f64::abs( g.0.0 - new_point.0 ) <2.5 && f64::abs( g.0.1 - new_point.1) < 2.5{ 
                                                    if g.1 < ret.0.unwrap() {
                                                        continue 'elementsloop;
                                                    }else{                       //a wrong hit was detected that should be overwritten
                                                        *ball_mov_vec = default_dir_vec.clone();
                                                   }
                                            }
                                        }

                                   ball_mov_vec.mirror_on_normal_vec(element, self.speed);
                                   offset.0 = 2.0* ball_mov_vec.x*args.dt;
                                   offset.1 = 2.0* ball_mov_vec.y*args.dt;
                                   hitcount +=1;
                                   hitpoints_vector.push((new_point , ret.0.unwrap()));
                              
                            }

                        }   
                    
                }
            }
            for e in hitpoints_vector{
                println!("WE HIT: {}  /  {} ", e.0.1, e.0.1);
            }
            drop(actualvector);
            if hitcount > 0 {
                println!("Hitc: {}",hitcount);
            }

            let mut mutpos = self.ball_pos.write().unwrap();
            mutpos.0 = new_x + offset.0;
            mutpos.1 = new_y + offset.1;

    }

        
    }

    pub fn init_speed(&mut self){
        let xdiff = self.ball_mov_vec.read().unwrap().x;
        let ydiff = self.ball_mov_vec.read().unwrap().y;
        self.speed = (xdiff*xdiff + ydiff*ydiff).sqrt();
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

        for el in &*mutval{
            println!("x: {}, y: {}",el.start_punkt.0, el.start_punkt.1);
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
        //create a rectangle of green lines around the field with subsequent calls

        let mut mutval = self.elements.write().unwrap();
        if mutval.len() < (FIELDWIDTH / 25.0f64).floor() as usize {
            if mutval.len() == 0{
                let start = (0.0, 0.0);
                let end = (25.0, 0.0);
                
                *self.dummy_element.write().unwrap() = insert_lines_sorted(Gerade::from_two_points(start, end), &mut mutval);
                return;
            }
            let end_punkt = *self.dummy_element.read().unwrap();
            *self.dummy_element.write().unwrap() = insert_lines_sorted(Gerade::from_two_points(end_punkt, (end_punkt.0 +25.0, 0.0)), &mut mutval);
            return;
        }
        if mutval.len() < (FIELDWIDTH/25.0f64).floor() as usize + (FIELDHEIGHT/25.0).floor() as usize {
            let end_punkt = *self.dummy_element.read().unwrap();

            *self.dummy_element.write().unwrap() = insert_lines_sorted(Gerade::from_two_points(end_punkt, (end_punkt.0, end_punkt.1+25.0)), &mut mutval);
            return;
        }
        if mutval.len() < (FIELDWIDTH/25.0f64).floor() as usize *2 + (FIELDHEIGHT/25.0).floor() as usize {
            let end_punkt =*self.dummy_element.read().unwrap();

            *self.dummy_element.write().unwrap() = insert_lines_sorted(Gerade::from_two_points(end_punkt, (end_punkt.0 -25.0, end_punkt.1)), &mut mutval);
            return;
        }
        
        let end_punkt = *self.dummy_element.read().unwrap();

        *self.dummy_element.write().unwrap() = insert_lines_sorted(Gerade::from_two_points(end_punkt, (end_punkt.0 , end_punkt.1 -25.0)), &mut mutval);

    }

    /**
     * returns wether or not the ball hit the line AND wether or not the movement vector needs to be mirrored on the original line or its normal variant
     */
    pub fn check_for_hit(&self, gerade: &Gerade, actual_crossing_point: &mut (f64, f64)) -> (Option<f64>, bool){

        let ball_pos = *self.ball_pos.read().unwrap();

        
        if ball_pos.0 > 880.0 && ball_pos.0 > 798.0 {
            println!("oh!");
        }

        let ret = gerade.distance_to(ball_pos, actual_crossing_point);
        //println!("Distance: {}", ret.0);
        if ret.0 < CIRCLERADIUS{
            if ret.1 {
                return (Some(ret.0), true);
            }

            return ( Some(ret.0), false);
        }

        return ( None , false);                         //the second bool is irrelevant here
    }
}


/**crate::
 * This function inserts the given Line at the correct index into the vector, update this for certain collision detection algorithms
 * 
 */
pub fn insert_lines_sorted(gerad : Gerade, vector: &mut RwLockWriteGuard<Vec<Gerade>>) -> (f64, f64){
        let start_y = gerad.start_punkt.1;
        let start_x = gerad.start_punkt.0;
        let ret = gerad.end_punkt;
        let mut check_x = true;
        for i in 0..vector.len(){
            if check_x{
                if vector[i].start_punkt.0 > start_x{
                    vector.insert(i, gerad );
                    return ret;
                }else if vector[i].start_punkt.0 == start_x{
                    check_x = false;
                }
            }else if vector[i].start_punkt.0 > start_x || vector[i].start_punkt.1 > start_y{
                vector.insert(i, gerad );
                return ret; 
            }

        }
        vector.push(gerad);
        return ret;

    

}