use std::sync::{ Arc, Mutex, RwLock, RwLockReadGuard};

use graphics::{Context, rectangle::{  rectangle_by_corners}, Rectangle, draw_state, ellipse};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{ RenderArgs};

//constants are defined in constants.rs, for use in the whole project
use crate::{constants::{FIELDWIDTH, FIELDHEIGHT, CIRCLERADIUS}, gerade::Gerade, vect_2d::Vector2D};


        //const values are compile time values and thus don't slow down the program
        //RGBA color definition in array: red, green, blue, alpha (1- opacity)
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const DARKGREY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

pub struct PistonView{
   // model_ref : Weak<Model>,
    gl: GlGraphics,
    pos: Arc<RwLock<Vec<RwLock<Vector2D>>>>,
    elements: Arc<RwLock<Vec<Gerade>>>,
}

impl PistonView {
    pub fn new(opengl: OpenGL, pos: &Arc<RwLock<Vec<RwLock<Vector2D>>>>, elems: &Arc<RwLock<Vec<Gerade>>>) -> Self{

        PistonView{
            gl : GlGraphics::new(opengl),
            elements : Arc::clone(elems),
            pos: Arc::clone(pos),
        }
    }

    pub fn render(&mut self,  args : &RenderArgs){
        use graphics::*;




        let position_read = self.pos.read();
        let position ;
        if position_read.is_err(){
            println!("{}", position_read.err().unwrap());
            return;
        }else{
            position = position_read.unwrap();
        }
        self.gl.draw(args.viewport(), |c, gl| {
            //the functions used here, like clear/rectangle are in namespace graphics::*, the use statement makes these omittable
            clear(DARKGREY, gl);

            PistonView::draw_background(&c, gl, args);
            PistonView::draw_objects(&c, gl, args, &position, &self.elements);

//transformations are calculatedfor the viewPort. This means, that the center of the screen will be moved to x,y, then 
//rotated, then offset an then the square is drawn with the top left corner at the given point. Then the screen is reset to the default
//position

/*
            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);               

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);*/
        });
    }

//remove #[inline] if you want to debug inside this function!
//inline results in the code being compiled into commands and inserted wherever this function is called, instead of actually calling a function (reduces overhead, increases speed and binary size)
    #[inline(always)]
    pub fn draw_background(c: &Context, gl: &mut GlGraphics, args: &RenderArgs){
        //this could have been done with a static field that is always at top left or is at top left, as long as the total width is smaller than the field 
        //OR this could  be downscaled as long as the field doesn't fit into the window, possible extension in the future


        let mid_x = args.window_size[0]/2.0;
        let mid_y = args.window_size[1]/2.0;
        let rec = Rectangle::new(BLUE);
        let bkgrnd = rectangle_by_corners(mid_x - (FIELDWIDTH as f64 / 2.0) , mid_y - (FIELDHEIGHT as f64 /2.0), mid_x + (FIELDWIDTH as f64 / 2.0), mid_y + (FIELDHEIGHT as f64 /2.0));
        //this function was called with &c, but it does not need to be dereferenced here (*c), as this is automatically done, so Object functions can be called on reference (autoderef)
        rec.draw(bkgrnd, &draw_state::DrawState::default(), c.transform, gl);
    }

    pub fn draw_objects( c: &Context, gl: &mut GlGraphics, args: &RenderArgs, locations: &RwLockReadGuard<Vec<RwLock<Vector2D>>>, elements: &Arc<RwLock<Vec<Gerade>>>){
        {
            let readvec = elements.read().unwrap();
            for elem in readvec.iter(){            //into_iter consumes original data, while .iter() does not. But into_iter is faster (no clone)
                let (x0, y0) = to_screen_coordinates(elem.start_punkt.0, elem.start_punkt.1, args);
                let (xend, yend) = to_screen_coordinates(elem.end_punkt.0,  elem.end_punkt.1, args);
                
                graphics::line(GREEN, 2.0, [x0 as f64, y0 as f64, xend as f64, yend as f64], c.transform, gl);
    
            }
        }
        
        //draw all circles:
        for i in 0..(&locations).len(){

            let locat = locations[i].read().unwrap();
            //let transform = c.transform.trans(location.0.into(), location.1.into()), would transform the drawing center into the actual center of the circle, so you could draw a circle at the correct spit with location of 0,0
            let (act_x, act_y) = to_screen_coordinates(locat.x, locat.y, args);

            let circle = graphics::ellipse::circle(act_x.into(), act_y.into(), CIRCLERADIUS.into());
            ellipse(RED, circle, c.transform, gl);

        }



    }

    

}

//this function takes x/y coordinates from an object and transfers them into 
#[inline(always)]
pub fn to_screen_coordinates(x: f64, y: f64, args: &RenderArgs) -> (f64, f64){
    let mid_x = args.window_size[0]/2.0;
    let mid_y = args.window_size[1]/2.0;
    let x0: f64 = mid_x as f64 - FIELDWIDTH/2.0f64;
    let y0: f64 = mid_y as f64 - FIELDHEIGHT/2.0f64;

    return (x0+ x, y0+y);

}