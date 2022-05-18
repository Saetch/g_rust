use std::{sync::{Arc, RwLock, RwLockWriteGuard, Mutex},
 thread, time::Duration, vec};
use crate::{ gerade::Gerade, constants::{FIELDWIDTH, FIELDHEIGHT, SPAWN_SIDES_WITH_DELAY, self, CIRCLERADIUS}, vect_2d::Vector2D};
use piston::{UpdateArgs};
use rand::Rng;


pub struct Model{
    //Arc -> atomically reference counted, used to share data between threads, mutex for MUTability and thread safety (rust enforces thread safety or it throws)
    //RwLock makes multiple reads to shared data simultaneously possible. Write access is blocked, tho.
    pub elements: Arc<RwLock<Vec<Gerade>>>,

    dummy_element: Arc<RwLock<(f64, f64)>>,   //holds the final point of the last added line

    //These were added later to replace single ball with multiple balls
    pub ball_mov_vectors: Arc<RwLock<Vec<RwLock<Vector2D>>>>,
    pub ball_positions: Arc<RwLock<Vec<RwLock<Vector2D>>>>,
    pub ball_speeds: Arc<RwLock<Vec<RwLock<f64>>>>,
    running: Mutex<bool>,
}

impl Model {
    pub fn new() -> Self{
        Model{
            elements: Arc::new(RwLock::new(Vec::new())),
            dummy_element: Arc::new(RwLock::new((0.0,0.0))),
            ball_mov_vectors: Arc::new(RwLock::new(Vec::new())),
            ball_positions: Arc::new(RwLock::new(Vec::new())),
            ball_speeds: Arc::new(RwLock::new(Vec::new())),
            running: Mutex::new(false),
        }
    }

  /*   this is not working, if needed, update this and unquote
      pub fn order_elements_x_then_y(&self ){
        let mut vec_guard = self.elements.write().unwrap();
        vec_guard.sort_by(|a, b| a.start_punkt.0.cmp(b.start_punkt.0))
    }*/

    pub fn update(& self, args : &UpdateArgs){

        let len = self.ball_positions.read().unwrap().len();
        if ! *self.running.lock().unwrap(){
            return;
        }
        for i in 0 ..len {
            let speed_g = &self.ball_speeds.read().unwrap()[i];
            if *speed_g.read().unwrap() > 0.0001{
                let pos = &self.ball_positions.read().unwrap()[i];
                let ball_mov_vec_guard = &self.ball_mov_vectors.write().unwrap()[i];       
                let mut ball_mov_vec      = ball_mov_vec_guard.write().unwrap();                                                                  //keeping this variable for longer is no problem, since the only thread that accesses it locking, is this thread
                let new_x = ball_mov_vec.x*args.dt + pos.read().unwrap().x;
                let new_y = ball_mov_vec.y*args.dt + pos.read().unwrap().y;
                let mut offset = (0.0, 0.0);
                let mut defaultref = false;
                let mut to_reflect_vec = Vector2D {x : 0.0 , y : 0.0};
                let mut distance= 100.0;
                let actualvector = self.elements.read().unwrap();
                drop(pos);      //this is dropped here, so the rendering thread does not delay if it tries to read the ballpos.
                'elementsloop:for element in &*actualvector{
                    if (element.start_punkt.0 <= new_x + 50.0 && element.start_punkt.0 >= new_x - 50.0) &&  (element.start_punkt.1 <= new_y + 50.0 && element.start_punkt.1 >= new_y -50.0) {            //only precisely check for a hit, if the elements are close enough. For larger amounts of elements, try a dedicated data structure, which yields only the relevant elements
                            let mut new_point = (0.0, 0.0);
                            let ret = self.check_for_hit(element, &mut new_point, i);
                            if ret.0.is_some(){
    
                                if ret.1{
                                        if !defaultref {
                                            to_reflect_vec.x = 0.0;
                                            to_reflect_vec.y = 0.0;
                                        }
                                        to_reflect_vec.x += element.linien_vektor.0;
                                        to_reflect_vec.y += element.linien_vektor.1;
    
                                        distance = f64::min(distance, ret.0.unwrap());
                                        defaultref = true;
    
                                }
                                else if !defaultref{
                                    to_reflect_vec.x += element.normalvektor.unwrap().0;
                                    to_reflect_vec.y += element.normalvektor.unwrap().1;
    
                                    distance = f64::min(distance, ret.0.unwrap());
    
                                }
    
                            }   
                        
                    }
                }
    
                drop(actualvector);
    
                if to_reflect_vec.x != 0.0 || to_reflect_vec.y != 0.0 {
                    ball_mov_vec.mirror_on_vec(to_reflect_vec, *speed_g.read().unwrap()); 
                    let actualdist = CIRCLERADIUS+ 1.0- distance;               //make sure the ball is not touching anything after it is moved!
                    let oned_vec = (ball_mov_vec.x / ball_mov_vec.length() , ball_mov_vec.y / ball_mov_vec.length());
                    offset.0 = actualdist * oned_vec.0;
                    offset.1 = actualdist * oned_vec.1;
                }
    
    
    
    
                let mutposs = &self.ball_positions.read().unwrap()[i];
                let mut mutpos = mutposs.write().unwrap();
                if offset.0 != 0.0 || offset.1 != 0.0{
                    mutpos.x += offset.0;
                    mutpos.y += offset.1;
                }else{
                    mutpos.x = new_x ;
                    mutpos.y = new_y ;
                }
    
        }


    }

        
    }

    pub fn init_speed(& self){
        let len = self.ball_mov_vectors.read().unwrap().len();

        for i in 0 .. len{
            let mut speedm_g = self.ball_speeds.write().unwrap(); 
            let vector_g = &(self.ball_mov_vectors.read().unwrap()[i]);
            let vector = vector_g.read().unwrap();
            if i>= speedm_g.len(){
                speedm_g.push( RwLock::new((vector.x.powi(2) + vector.y.powi(2)).sqrt()));

            }else{
                println!("Something happened at initspeed, which was not planned!");
            }
        }
    }

    pub fn spawn_sides(& self){
        let mut run = self.running.lock().unwrap();

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
    
        let mut ball_pos_g = self.ball_positions.write().unwrap();
        let mut ball_mov_g = self.ball_mov_vectors.write().unwrap();

        for _i in 0..120 {

            let mut rand = rand::thread_rng();
            ball_pos_g.push(RwLock::new(Vector2D{x: 400.0, y: 400.0}));
            ball_mov_g.push(RwLock::new(Vector2D{x: rand.gen_range(20..420) as f64, y: rand.gen_range(20..370) as f64 }));
        }

    
        //these need to be dropped, because init_speed will try to read from them!
        drop(ball_mov_g);
    
        self.init_speed();
        *run = true;
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
    pub fn check_for_hit(&self, gerade: &Gerade, actual_crossing_point: &mut (f64, f64), index: usize) -> (Option<f64>, bool){

        let ball_v = &self.ball_positions.read().unwrap()[index];
        let ball_ = ball_v.read().unwrap();

        let ball_pos = (ball_.x, ball_.y);
        drop(ball_v);
        drop(ball_);
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