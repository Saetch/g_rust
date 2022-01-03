use std::sync::{ Arc, Mutex};

use opengl_graphics::{GlGraphics, OpenGL};
use piston::{ RenderArgs};



pub struct PistonView{
   // model_ref : Weak<Model>,
    gl: GlGraphics,
    rot: Arc<Mutex<f64>>
}

impl PistonView {
    pub fn new(rot: &Arc<Mutex<f64>>, opengl: OpenGL) -> Self{

        PistonView{
            gl : GlGraphics::new(opengl),
            rot: Arc::clone(rot)
        }
    }

    pub fn render(&mut self,  args : &RenderArgs){
        use graphics::*;
        //RGBA color definition in array: red, green, blue, alpha (1- opacity)
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let rotation = *self.rot.lock().unwrap();
        //place it at x,y, in this case in the middle: args.window_size[0] -> width, args.window_size[1] -> height
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let square = graphics::rectangle::square(0.0, 0.0, 100.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
//transformations are calculatedfor the viewPort. This means, that the center of the screen will be moved to x,y, then 
//rotated, then offset an then the square is drawn with the top left corner at the given point. Then the screen is reset to the default
//position
            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-50.0, -50.0);               

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }


}