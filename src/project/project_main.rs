use sdl2::{pixels::Color, rect::Rect};

type SDLTextureCreator = sdl2::render::TextureCreator<sdl2::video::WindowContext>;

use crate::{
    ansi_codes,
    interpreter::{Instruction, Value},
    project_state::Renderer,
    sprite::{Costume, GraphicalProperties, Sprite},
    thread::Thread,
    thread_compiler::{
        thread_compiler_main::ThreadCompiler, thread_compiler_variable_manager::VariableCompiler,
    },
};

pub struct Project<'a> {
    memory: Box<[Value]>,
    sprites: Vec<Sprite<'a>>,
    pub path: std::path::PathBuf,
    pub json: serde_json::Value,
}

impl<'a> Project<'a> {
    pub fn new(
        project_path: String,
        texture_creator: &'a SDLTextureCreator,
    ) -> Result<Project<'a>, String> {
        // Extract sb3 zip to a temporary directory.
        let (_project_directory_object, project_path) = Project::extract_zip_file(project_path)?;
        let json = Project::load_json(&project_path);

        // Create a temporary project. We will load the code into this and return it.
        let mut project = Project {
            memory: Box::new([]),
            sprites: vec![],
            path: project_path,
            json,
        };

        let mut variables = VariableCompiler::new();

        let mut font_database = usvg_text_layout::fontdb::Database::new();
        font_database.load_system_fonts();

        let sprites = project.json["targets"]
            .as_array()
            .expect("Malformed JSON - No \"targets\" list of sprites");

        for sprite_json in sprites.iter() {
            println!(
                "{}[info]{} started compiling sprite {}",
                ansi_codes::GREEN,
                ansi_codes::RESET,
                sprite_json["name"]
            );

            let mut sprite = Project::sprite_create_from_json(sprite_json);

            sprite.load_costumes(sprite_json, &project, &font_database, texture_creator)?;

            variables.load_from_json(&sprite_json);

            Project::compile_hat_blocks(&mut variables, &mut sprite, sprite_json);

            project.sprites.push(sprite);
        }

        // Allocate enough memory for the variables.
        project.memory = variables.finish_processing();

        Ok(project)
    }

    pub fn run(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        pen_canvas: &mut Renderer,
    ) {
        for sprite in &mut self.sprites {
            sprite.run(&mut self.memory, canvas, pen_canvas);
        }
    }

    pub fn _print_pretty(value: &serde_json::Value) -> String {
        serde_json::to_string_pretty(&value).expect("Could not print project.json")
    }

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        renderer: &mut Renderer,
    ) {
        // TODO: Draw sprites based on their layer order.
        // Currently it just draws sprites in the order of how they were loaded.
        for sprite in self.sprites.iter() {
            if sprite.graphics.shown {
                // let properties = &sprite.graphics;
                let current_costume = &sprite.costumes[sprite.graphics.costume_number];
                let rect = get_sprite_rect(
                    &sprite.graphics,
                    current_costume,
                    canvas.output_size().unwrap(),
                );
                canvas.copy(&current_costume.data, None, rect).unwrap();
            }

            if sprite.name == "Stage" {
                canvas
                    .copy(&renderer.main_canvas, None, Rect::new(0, 0, 800, 600))
                    .unwrap();
            }
        }
    }

    pub fn sprite_create_from_json(sprite: &serde_json::Value) -> Sprite<'a> {
        Sprite::new(
            sprite["name"].as_str().unwrap().to_string(),
            if sprite["isStage"].as_bool().unwrap() {
                Default::default()
            } else {
                GraphicalProperties {
                    x: sprite["x"].as_f64().unwrap(),
                    y: sprite["y"].as_f64().unwrap(),
                    shown: sprite["visible"].as_bool().unwrap(),
                    direction: sprite["direction"].as_f64().unwrap() as f32,
                    size: sprite["size"].as_f64().unwrap() as f32,
                    costume_number: 0,
                    pen_down: false,
                    pen_radius: 1,
                    pen_color: Color::RGB(0, 0, 255),
                }
            },
        )
    }

    fn sprite_find_hat_blocks(sprite: &serde_json::Value) -> Vec<(&String, &serde_json::Value)> {
        let mut hat_blocks: Vec<(&String, &serde_json::Value)> = vec![];
        for (block_id, block_data) in sprite["blocks"].as_object().unwrap() {
            if block_data.is_array() {
                continue;
            }
            // If the block has no parent (At the top)
            if block_data["parent"] == serde_json::Value::Null {
                hat_blocks.push((block_id, block_data));
            }
        }
        hat_blocks
    }

    fn compile_hat_blocks(
        variables: &mut VariableCompiler,
        temp_sprite: &mut Sprite<'_>,
        sprite_json: &serde_json::Value,
    ) {
        let hat_blocks = Project::sprite_find_hat_blocks(sprite_json);

        for (thread_number, (_, block_json)) in hat_blocks.iter().enumerate() {
            let opcode = block_json["opcode"].as_str().unwrap();
            match opcode {
                "event_whenflagclicked" => c_events_whenflagclicked(
                    variables,
                    temp_sprite,
                    block_json,
                    thread_number,
                    sprite_json,
                ),
                _ => {
                    eprintln!(
                        "{}[unimplemented hat block]{} {opcode}",
                        ansi_codes::RED,
                        ansi_codes::RESET
                    )
                }
            }
        }
    }
}

pub fn get_sprite_rect(
    properties: &GraphicalProperties,
    current_costume: &Costume<'_>,
    (canvas_width, canvas_height): (u32, u32),
) -> Rect {
    let size = properties.size / 100.0;
    let query = current_costume.data.query();

    let size_difference_f64 = canvas_width as f64 / 480.0;
    let size_difference_f32 = canvas_width as f32 / 480.0;

    let width = size * query.width as f32 * size_difference_f32;
    let height = size * query.height as f32 * size_difference_f32;

    let mut sprite_x = properties.x - current_costume.centre_x;
    let mut sprite_y = properties.y + current_costume.centre_y;

    sprite_x *= size_difference_f64;
    sprite_y *= size_difference_f64;

    sprite_x += canvas_width as f64 / 2.0;
    sprite_y = (canvas_height as f64 / 2.0) - sprite_y;

    sdl2::rect::Rect::new(
        sprite_x as i32,
        sprite_y as i32,
        width as u32,
        height as u32,
    )
}

pub fn get_scaled_point(
    (x, y): (f64, f64),
    (canvas_width, canvas_height): (u32, u32),
) -> (i32, i32) {
    let size_difference_f64 = canvas_width as f64 / 480.0;

    let mut sprite_x = x;
    let mut sprite_y = y;

    sprite_x *= size_difference_f64;
    sprite_y *= size_difference_f64;

    sprite_x += canvas_width as f64 / 2.0;
    sprite_y = (canvas_height as f64 / 2.0) - sprite_y;

    (sprite_x as i32, sprite_y as i32)
}

fn c_events_whenflagclicked(
    variables: &mut VariableCompiler,
    temp_sprite: &mut Sprite,
    event_block_json: &serde_json::Value,
    thread_number: usize,
    sprite: &serde_json::Value,
) {
    let mut instructions: Vec<Instruction> = vec![];

    let mut compiler = ThreadCompiler::new(variables, &mut instructions, thread_number, sprite);

    let mut block = (*event_block_json).clone();
    while block["next"] != serde_json::Value::Null {
        let block_id = block["next"].as_str().unwrap();
        block = compiler.get_block(block_id).unwrap();
        compiler.compile_block(&block);
    }

    compiler.finish();
    compiler.optimize();
    compiler.dump();

    temp_sprite
        .threads
        .push(Thread::new(instructions.into_boxed_slice()));
}

fn _ls(path: &std::path::Path) {
    match std::fs::read_dir(path) {
        Ok(entries) => {
            // Iterate over the entries and print their names
            for entry in entries {
                match entry {
                    Ok(dir_entry) => {
                        let entry_path = dir_entry.path();
                        let entry_name = entry_path.file_name().unwrap_or_default();
                        println!("{}", entry_name.to_string_lossy());
                    }
                    Err(err) => eprintln!("Error reading directory entry: {}", err),
                }
            }
        }
        Err(err) => eprintln!("Error reading directory: {}", err),
    }
}
