use std::sync::{ Arc, Mutex};

use graphics::{Context, rectangle::{ self, rectangle_by_corners}, Rectangle, Transformed, draw_state};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{ RenderArgs};

//constants are defined in constants.rs, for use in the whole project
use crate::constants::{self as constant, FIELDWIDTH, FIELDHEIGHT};


        //const values are compile time values and thus don't slow down the program
        //RGBA color definition in array: red, green, blue, alpha (1- opacity)
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const DARKGREY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

pub struct PistonView{
   // model_ref : Weak<Model>,
    gl: GlGraphics,
    rot: Arc<Mutex<f64>>,
    pos: Arc<Mutex<(f32, f32)>>,
}

impl PistonView {
    pub fn new(rot: &Arc<Mutex<f64>>, opengl: OpenGL, pos: &Arc<Mutex<(f32, f32)>>) -> Self{

        PistonView{
            gl : GlGraphics::new(opengl),
            rot: Arc::clone(rot),
            pos: Arc::clone(pos),
        }
    }

    pub fn render(&mut self,  args : &RenderArgs){
        use graphics::*;




        let position = *self.pos.lock().unwrap();
        //place it at x,y, in this case in the middle: args.window_size[0] -> width, args.window_size[1] -> height
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            //the functions used here, like clear/rectangle are in namespace graphics::*, the use statement makes these omittable
            clear(DARKGREY, gl);

            PistonView::draw_background(&c, gl, args);
            

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


   // #[inline(always)]
    pub fn draw_background(c: &Context, gl: &mut GlGraphics, args: &RenderArgs){
        //this could have been done with a static field that is always at top left or is at top left, as long as the total width is smaller than the field 
        //OR this could have been downscaled as long as the field doesn't fit into the window


        let mid_x = args.window_size[0]/2.0;
        let mid_y = args.window_size[1]/2.0;
        let rec = Rectangle::new(BLUE);
        let bkgrnd = rectangle_by_corners(mid_x - (FIELDWIDTH as f64 / 2.0) , mid_y - (FIELDHEIGHT as f64 /2.0), mid_x + (FIELDWIDTH as f64 / 2.0), mid_y + (FIELDHEIGHT as f64 /2.0));
        //this function was called with &c, but it does not need to be dereferenced here (*c), as this is automatically done, so Object functions can be called on reference
        rec.draw(bkgrnd, &draw_state::DrawState::default(), c.transform, gl);
    }


}