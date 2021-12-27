use std::sync::{Arc, Mutex};

use piston::{ButtonArgs, Key, Button, ButtonState};

use crate::model::Model;

//Implementation is quite minimalistic, multiple states might be necessary for multiple game states
#[derive()]
pub struct Controller{
    model: Arc<Mutex<Model>>,

}


impl Controller{
    pub fn new( modelr : &Arc<Mutex<Model>>) -> Self{
        return Controller{
            model : modelr.clone(),
        };
    }

    pub fn compute_input(&mut self, args: &ButtonArgs){
        if args.state == ButtonState::Release{                          //only compute key presses, not 
            return;
        }
        match args.button{
            //add the type of input to read here
            Button::Keyboard(key) => self.compute_keyboard(key),
            _ => (),
        }
    }




    fn compute_keyboard(&mut self, key: Key){

        match key{
            Key::O => self.model.lock().unwrap().debug_rad_action(),
            _ =>(),
        }

    }

}