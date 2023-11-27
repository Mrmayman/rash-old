use crate::{interpreter::Value, thread::Thread};

pub struct GraphicalProperties {
    pub x: f64,
    pub y: f64,
    pub size: f32,
    pub direction: f32,
}

impl Default for GraphicalProperties {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            size: 100.0,
            direction: 90.0,
        }
    }
}

pub struct Costume<'a> {
    pub centre_x: f64,
    pub centre_y: f64,
    pub data: Option<sdl2::render::Texture<'a>>,
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

    pub fn run(&mut self, memory: &mut [Value]) {
        let mut i = 0;
        while i < self.threads.len() {
            let thread = &mut self.threads[i];
            thread.run(memory);

            if thread.killed {
                self.threads.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
