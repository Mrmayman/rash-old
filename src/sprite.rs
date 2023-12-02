use sdl2::pixels::Color;

use crate::{interpreter::Value, project_state::ProjectState, thread::Thread};

pub struct GraphicalProperties {
    pub x: f64,
    pub y: f64,
    pub size: f32,
    pub shown: bool,
    pub direction: f32,
    pub costume_number: usize,
    pub pen_down: bool,
    pub pen_radius: i32,
    pub pen_color: Color,
}

impl Default for GraphicalProperties {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            size: 100.0,
            shown: true,
            direction: 90.0,
            costume_number: 0,
            pen_down: false,
            pen_radius: 1,
            pen_color: Color::RGB(0, 0, 255),
        }
    }
}

pub struct Costume<'a> {
    pub centre_x: f64,
    pub centre_y: f64,
    pub data: sdl2::render::Texture<'a>,
    pub name: String,
}

pub struct Sprite<'a> {
    pub threads: Vec<Thread>,
    pub name: String,
    pub graphical_properties: GraphicalProperties,
    pub costumes: Vec<Costume<'a>>,
}

impl<'a> Sprite<'a> {
    pub fn new(name: String, graphical_properties: GraphicalProperties) -> Sprite<'a> {
        Sprite {
            threads: vec![],
            name,
            graphical_properties,
            costumes: vec![],
        }
    }

    pub fn run(
        &mut self,
        memory: &mut [Value],
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        pen_canvas: &mut ProjectState,
    ) {
        let mut i = 0;
        while i < self.threads.len() {
            let thread = &mut self.threads[i];
            thread.run(
                memory,
                &mut self.graphical_properties,
                &self.costumes,
                canvas,
                pen_canvas,
            );

            if thread.killed {
                self.threads.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
