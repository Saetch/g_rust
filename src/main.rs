extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;


const WIDTH : u32 = 1200;
const HEIGHT : u32 = 800;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut up : bool = true;
    let mut vertical = true;
    let mut x = 500;
    let mut y = 200;
    'running: loop {

        //Das ist bad practice! Wenn die Framerate fällt, wäre hier die Logik langsamer sleep(Duration::new(0, 1_000_000_000u32 / 60). Das muss man verbessern!
        //indem man die vergangene Zeit explizit berechnet und als argument der Logik übergibt, die darauf multipliziert
        if up{ 
            i = i + 1 ;
            if i == 255{
                up = false;
            }
        }
        else{
            i = i-1;
            if i == 0 {
                up = true;
            }
        }
        if vertical{
            if y == 200{
                x+=1;
                if x == 700{
                    vertical = false;
                }
            }else{
                x-=1;
                if x == 500{
                    vertical = false;
                }
            }
        }else{
            if x == 500 {
                y-=1;
                if y == 200{
                    vertical= true;
                }
            }else{
                y+=1;
                if y == 400{
                    vertical = true;
                }
            }
        }

        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        
        //45*
        draw_line_width(&mut canvas , Point::new(100, 100), Point::new(200, 100), 5);
    
        draw_line_width(&mut canvas, Point::new( 200, 300), Point::new(200, 200), 5);
        draw_line_width(&mut canvas, Point::new( 200, 300), Point::new(100, 300), 5);
        draw_line_width(&mut canvas, Point::new(600, 300), Point::new(x, y), 10);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


pub fn draw_line_width(can : &mut Canvas<Window>, start : Point, end : Point, width: u32){
    let x_diff = end.x() - start.x();
    let y_diff = end.y() - start.y();
    let len;
    
    len = f32::sqrt( (x_diff*x_diff +y_diff*y_diff) as f32);
    
    let sin: f32 =  x_diff as f32 / len;
    let cos: f32 = y_diff as f32 / len;
    let mut x_offset;
    let mut y_offset;
    let mut x_int;
    let mut y_int;
    for i in -(width as i32)..= width as i32 {

        x_offset = (cos* i as f32).round();
        y_offset= (sin* i as f32).round();
        x_int = x_offset as i32;
        y_int = y_offset as i32;
        can.draw_line(Point::new(start.x()- x_int , start.y()+y_int),  Point::new(end.x()- x_int , end.y()+y_int)).unwrap();

           /*
        //first try   
        if cos > sin{
            can.draw_line(Point::new(start.x()+i , start.y()+(((i as f32)*sin).round() as i32)),  Point::new(end.x()+i , end.y()+(((i as f32)*sin).round()as i32))).unwrap();

        }else{
            can.draw_line(Point::new(start.x()+(((i as f32) *cos ).round()as i32), start.y()+i ),  Point::new(end.x()+(((i as f32) *cos ).round()as i32), end.y()+i )).unwrap();
        }*/

       
        
    }
  
}
    


    