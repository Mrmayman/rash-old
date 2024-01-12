use sdl2::pixels::Color;

use crate::{
    interpreter::Value, project::project_main::Project, project_state::Renderer, thread::Thread,
};

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
    pub graphics: GraphicalProperties,
    pub costumes: Vec<Costume<'a>>,
}

impl<'a> Sprite<'a> {
    pub fn new(name: String, graphical_properties: GraphicalProperties) -> Sprite<'a> {
        Sprite {
            threads: vec![],
            name,
            graphics: graphical_properties,
            costumes: vec![],
        }
    }

    pub fn load_costumes(
        &mut self,
        sprite: &serde_json::Value,
        project: &Project<'a>,
        db: &usvg_text_layout::fontdb::Database,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> Result<(), String> {
        let costumes = match sprite["costumes"].as_array() {
            Some(costumes) => costumes,
            None => return Err("JSON error: Cannot find costumes field in sprite.".to_owned()),
        };

        for costume_json in costumes {
            if costume_json["dataFormat"].as_str().unwrap() == "svg" {
                crate::costume_loader::convert_svg_to_png(costume_json, project, db)?;
            }

            let texture =
                match crate::costume_loader::load_png(texture_creator, project, costume_json) {
                    Ok(texture) => texture,
                    Err(err) => {
                        return Err(format!("JSON error: Failed to load costume: {:?}", err))
                    }
                };

            self.costumes.push(Costume {
                centre_x: costume_json["rotationCenterX"].as_f64().unwrap(),
                centre_y: costume_json["rotationCenterY"].as_f64().unwrap(),
                data: texture,
                name: costume_json["name"].as_str().unwrap().to_string(),
            });
        }

        let costume_number = sprite["currentCostume"].as_i64().unwrap();
        self.graphics.costume_number = costume_number as usize;

        Ok(())
    }

    pub fn run(
        &mut self,
        memory: &mut [Value],
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        pen_canvas: &mut Renderer,
    ) {
        let mut i = 0;
        while i < self.threads.len() {
            let thread = &mut self.threads[i];
            thread.run(
                memory,
                &mut self.graphics,
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
