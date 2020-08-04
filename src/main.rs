extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
mod grid;
mod search;
mod point;
mod a_star;

use crate::grid::Grid;
use crate::search::{Search,State};
use crate::a_star::search::AStarSearch;
use crate::point::{Point};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const OPEN_GL: OpenGL = OpenGL::V3_2;
const HEIGHT: u32 = 100;
const WIDTH: u32 = 100;
const RENDER_FACTOR: u32 = 10;

pub struct AppState {
    gl: GlGraphics, // OpenGL drawing backend.
    search: Box<dyn Search>,
    start: Point,
    end: Point,
    grid_shape: Point
}

impl AppState {
    pub fn new() -> AppState {
        let x_max = WIDTH as usize;
        let y_max = HEIGHT as usize;
        // let start = Point::new(32, 23);
        // let end = Point::new(63, 4);
        let start = Point::rand(x_max, y_max);
        let end = Point::rand(x_max, y_max);
        let grid_shape = Point::new(x_max, y_max);
        let grid = Grid::new(grid_shape).unwrap();
        let search = AStarSearch::create(start, end, grid).unwrap();
        return AppState {
            gl: GlGraphics::new(OPEN_GL),
            search: Box::from(search),
            start: start,
            end: end,
            grid_shape: grid_shape
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const WHITE: [f32; 4] = [1.0,1.0,1.0,1.0];
        const GREY: [f32; 4] = [0.3,0.3,0.3,1.0];
        
        let width = self.grid_shape.x;
        let height = self.grid_shape.y;
        let search = &self.search;
        let end = &self.end;
        let start = &self.start;
        self.gl.draw(args.viewport(), |c, g| {
            clear(WHITE, g);

            let mut draw_square = |x, y, color| {
                let rectangle = Rectangle::new(color);
                let dims = rectangle::square(x as f64 * RENDER_FACTOR as f64, y as f64 * RENDER_FACTOR as f64, 1.0*RENDER_FACTOR as f64);
                rectangle.draw(dims, &c.draw_state, c.transform, g);
            };
            
            for x in 0..width {
                for y in 0..height {
                    let color = match search.get_cell_state(Point::new(x, y)) {
                        Some(State::Current) => RED,
                        Some(State::ChosenPath) => GREEN,
                        Some(State::Visited) => GREY,
                        Some(State::NotVisited) => WHITE,
                        None => color::BLACK
                    };
                    draw_square(x, y, color);
                }
            }
            draw_square(end.x, end.y, GREEN);
            draw_square(start.x, start.y, BLUE);

        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.search.step();
    }
}

fn main() {
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [HEIGHT*RENDER_FACTOR, WIDTH*RENDER_FACTOR])
        .graphics_api(OPEN_GL)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = AppState::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}