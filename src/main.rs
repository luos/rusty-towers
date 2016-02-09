extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::collections::LinkedList;
mod map;
use rand::Rng;



pub struct App {

    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    map: map::Map
}

impl App {


    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
		let elem : map::MapElement = map::MapElement{ x:1, y:2, color: RED };

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, CELL_SIZE);

		let map = &self.map;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            for row in &map.elements {
            		for cell in row {
            			let transform = c.transform.trans( CELL_SIZE * ( cell.x as f64) , CELL_SIZE * ( cell.y as f64))
			                                   .rot_rad( 0.0 );
			                                   //.trans(-CELL_HALF, -CELL_HALF);

			        // Draw a box rotating around the middle of the screen.
			        	rectangle( cell.color , square, transform, gl );
            		}


            }

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        //self.rotation += 2.0 * args.dt;
    }
}

const MAP_WIDTH : u32 = 45;
const MAP_HEIGHT : u32 = 30;
const CELL_SIZE : f64 = 22.0;
//const CELL_HALF : f64 = CELL_SIZE / 2.0;
const WINDOW_WIDTH : u32 = ( (MAP_WIDTH  as f64 ) * CELL_SIZE) as u32;
const WINDOW_HEIGHT : u32 = ((MAP_HEIGHT  as f64 ) * CELL_SIZE) as u32;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let mut rng = rand::thread_rng();

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [ WINDOW_WIDTH, WINDOW_HEIGHT]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut elems = Vec::new();

    for x in 0..MAP_WIDTH {
    	let mut cols = Vec::new();

        for y in 00..MAP_HEIGHT {
    		cols.push( map::MapElement{ x: x, y: y, color: [ rng.gen::<f32>(), 0.2, rng.gen::<f32>(), 1.0  ] } );
        };
        elems.push( cols );
    };

    let map = map::Map{ width: MAP_WIDTH , height: MAP_HEIGHT , elements: elems };

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        map: map
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
