use sdl2::{image::LoadTexture, rect::Rect};
use std::collections::HashMap;
use tempfile::TempDir;

use crate::{
    interpreter::{Instruction, Value},
    pen_canvas::PenCanvases,
    project::state::ParseState,
    sprite::{Costume, GraphicalProperties, Sprite},
    third_party,
    thread::Thread,
};

pub struct Project<'a> {
    _temp_directory: TempDir,
    memory: Box<[Value]>,
    sprites: Vec<Sprite<'a>>,
    pub path: std::path::PathBuf,
    pub json: serde_json::Value,
}

pub enum BlockResult {
    Nothing,
    AllocatedMemory(usize),
}

impl<'a> Project<'a> {
    pub fn new(
        file_path: String,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> Result<Project<'a>, String> {
        let (temp_dir, temp_dir_path) = Project::load_project_file(file_path);
        let json = Project::parse(&temp_dir_path);

        let mut temp_project = Project {
            _temp_directory: temp_dir,
            memory: Box::new([]),
            sprites: vec![],
            path: temp_dir_path,
            json,
        };

        /*println!(
            "{}",
            serde_json::to_string_pretty(&temp_project.json).expect("Could not print project.json")
        );*/

        let mut variables: HashMap<String, usize> = HashMap::new();
        let mut variable_memory: Vec<Value> = vec![];

        let mut db = usvg_text_layout::fontdb::Database::new();
        db.load_system_fonts();

        for sprite in temp_project.json["targets"]
            .as_array()
            .expect("Malformed JSON - No \"targets\" list of sprites")
        {
            println!("[info] started compiling sprite {}", sprite["name"]);

            // Create the sprite
            let mut temp_sprite = Project::sprite_create_from_json(sprite);

            // Load sprite costumes
            Project::sprite_costume_load(
                sprite,
                &temp_project,
                &db,
                texture_creator,
                &mut temp_sprite,
            );

            // Load variables
            Project::sprite_load_variables(sprite, &mut variable_memory, &mut variables);

            let orphans = Project::sprite_find_hat_blocks(sprite);

            Project::compile_hat_blocks(
                orphans,
                &mut variables,
                &mut variable_memory,
                &mut temp_sprite,
                sprite,
            );

            temp_project.sprites.push(temp_sprite);
        }

        // Allocate enough memory for the variables.
        temp_project.memory = variable_memory.into_boxed_slice();

        Ok(temp_project)
    }

    /*fn _load_pi_project(&mut self) {
        self.sprites
            .push(Sprite::new("Sprite1".to_string(), Default::default()));
        self.sprites[0].threads.push(Thread::new(
            vec![
                Instruction::MemoryStore(Value::Pointer(0), Value::Number(0.0)),
                Instruction::MemoryStore(Value::Pointer(2), Value::Number(1.0)),
                Instruction::MemoryStore(Value::Pointer(4), Value::Number(0.0)),
                // Instruction::MemoryDump,
                // Instruction::FlowDefinePlace("Starting".to_string()),
                Instruction::OperatorModulo(
                    Value::Pointer(5),
                    Value::Pointer(4),
                    Value::Number(2.0),
                ),
                Instruction::OperatorMultiply(
                    Value::Pointer(5),
                    Value::Pointer(5),
                    Value::Number(2.0),
                ),
                Instruction::OperatorSubtract(
                    Value::Pointer(3),
                    Value::Pointer(5),
                    Value::Number(1.0),
                ),
                Instruction::OperatorDivide(
                    Value::Pointer(5),
                    Value::Number(4.0),
                    Value::Pointer(2),
                ),
                Instruction::OperatorMultiply(
                    Value::Pointer(5),
                    Value::Pointer(3),
                    Value::Pointer(5),
                ),
                Instruction::OperatorAdd(Value::Pointer(0), Value::Pointer(0), Value::Pointer(5)),
                Instruction::OperatorAdd(Value::Pointer(2), Value::Pointer(2), Value::Number(2.0)),
                Instruction::OperatorAdd(Value::Pointer(4), Value::Pointer(4), Value::Number(1.0)),
                Instruction::OperatorLesser(
                    Value::Pointer(5),
                    Value::Pointer(4),
                    Value::Number(1_000_000.0),
                ),
                // Instruction::FlowIfJumpToPlace(Value::Pointer(5), "Starting".to_string()),
                Instruction::FlowIfJump(Value::Pointer(5), Value::Number(5.0)),
                // Instruction::MemoryDump,
                Instruction::ThreadPause,
                Instruction::ThreadKill,
            ]
            .into_boxed_slice(),
        ));
        self._malloc(16);
    }

    pub fn _malloc(&mut self, size: usize) {
        self.memory = vec![Value::Number(0.0); size].into_boxed_slice();
    }*/

    pub fn run(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        pen_canvas: &mut PenCanvases,
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
        pen_canvas: &mut PenCanvases,
    ) {
        for sprite in &self.sprites {
            let properties = &sprite.graphical_properties;
            let current_costume = &sprite.costumes[properties.costume_number];
            let rect = get_sprite_rect(properties, current_costume, canvas.output_size().unwrap());
            canvas.copy(&current_costume.data, None, rect).unwrap();

            if sprite.name == "Stage" {
                canvas
                    .copy(&pen_canvas.pen_canvas, None, Rect::new(0, 0, 800, 600))
                    .unwrap();
            }
        }
    }

    pub fn sprite_create_from_json(sprite: &serde_json::Value) -> Sprite<'a> {
        let temp_sprite = Sprite::new(
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
                }
            },
        );
        temp_sprite
    }

    fn sprite_costume_load(
        sprite: &serde_json::Value,
        temp_project: &Project<'a>,
        db: &usvg_text_layout::fontdb::Database,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        temp_sprite: &mut Sprite<'a>,
    ) {
        for costume in sprite["costumes"].as_array().unwrap() {
            costume_convert_svg_to_png(costume, temp_project, db);
            let texture = costume_load_png(texture_creator, temp_project, costume);

            temp_sprite.costumes.push(Costume {
                centre_x: costume["rotationCenterX"].as_f64().unwrap(),
                centre_y: costume["rotationCenterY"].as_f64().unwrap(),
                data: texture,
                name: costume["name"].as_str().unwrap().to_string(),
            });
        }
    }

    fn sprite_load_variables(
        sprite: &serde_json::Value,
        variable_memory: &mut Vec<Value>,
        variables: &mut HashMap<String, usize>,
    ) {
        for (variable_hash, variable_data) in sprite["variables"].as_object().unwrap() {
            variable_memory.push({
                match &variable_data.as_array().unwrap()[1] {
                    serde_json::Value::Bool(n) => Value::Boolean(*n),
                    serde_json::Value::Number(n) => Value::Number(n.as_f64().unwrap()),
                    serde_json::Value::String(n) => Value::String(n.clone()),
                    _ => panic!(),
                }
            });
            variables.insert(variable_hash.clone(), variable_memory.len() - 1);
        }
    }

    fn sprite_find_hat_blocks(sprite: &serde_json::Value) -> Vec<(&String, &serde_json::Value)> {
        let mut hat_blocks: Vec<(&String, &serde_json::Value)> = vec![];
        for (block_id, block_data) in sprite["blocks"].as_object().unwrap() {
            if block_data.is_array() {
                eprintln!("[unimplemented] block is an array");
                break;
            }
            // If the block has no parent (At the top)
            if block_data["parent"] == serde_json::Value::Null {
                hat_blocks.push((block_id, block_data));
            }
        }
        hat_blocks
    }

    fn compile_hat_blocks(
        orphans: Vec<(&String, &serde_json::Value)>,
        variables: &mut HashMap<String, usize>,
        variable_memory: &mut Vec<Value>,
        temp_sprite: &mut Sprite<'_>,
        sprite: &serde_json::Value,
    ) {
        for (thread_number, (_, block_data)) in orphans.iter().enumerate() {
            let opcode = block_data["opcode"].as_str().unwrap();
            match opcode {
                "event_whenflagclicked" => c_events_whenflagclicked(
                    variables,
                    variable_memory,
                    temp_sprite,
                    block_data,
                    thread_number,
                    sprite,
                ),
                _ => {
                    eprintln!("[unimplemented hat block] {opcode}")
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

    let rect = sdl2::rect::Rect::new(
        sprite_x as i32,
        sprite_y as i32,
        width as u32,
        height as u32,
    );
    rect
}

fn costume_load_png<'a>(
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    temp_project: &Project<'_>,
    costume: &serde_json::Value,
) -> sdl2::render::Texture<'a> {
    let texture: sdl2::render::Texture<'a> = load_texture_from_file(
        texture_creator,
        temp_project
            .path
            .join(costume["assetId"].as_str().unwrap().to_string() + ".png"),
    )
    .unwrap();
    texture
}

fn costume_convert_svg_to_png(
    costume: &serde_json::Value,
    temp_project: &Project<'_>,
    db: &usvg_text_layout::fontdb::Database,
) {
    if costume["dataFormat"].as_str().unwrap() == "svg" {
        third_party::svg_to_png::render(
            temp_project
                .path
                .join(costume["md5ext"].as_str().unwrap())
                .as_ref(),
            temp_project
                .path
                .join(costume["assetId"].as_str().unwrap().to_string() + ".png")
                .as_ref(),
            db,
        )
        .unwrap();
    }
}

fn load_texture_from_file(
    texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    file_path: std::path::PathBuf,
) -> Result<sdl2::render::Texture<'_>, String> {
    Ok(texture_creator.load_texture(file_path).unwrap())
}

fn c_events_whenflagclicked(
    variables: &mut HashMap<String, usize>,
    variable_memory: &mut Vec<Value>,
    temp_sprite: &mut Sprite,
    block_data: &serde_json::Value,
    thread_number: usize,
    sprite: &serde_json::Value,
) {
    let mut instructions: Vec<Instruction> = vec![];
    let mut current_block = (*block_data).clone();

    let mut state = ParseState::new(
        variables,
        variable_memory,
        &mut instructions,
        thread_number,
        sprite,
    );

    while current_block["next"] != serde_json::Value::Null {
        let next = current_block["next"].as_str().unwrap();
        current_block = state.get_block(next).unwrap();
        state.compile_block(&current_block);
    }
    instructions.push(Instruction::ThreadKill);

    dump_instructions_and_variables(variables, variable_memory, &instructions);

    temp_sprite
        .threads
        .push(Thread::new(instructions.into_boxed_slice()));
}

fn dump_instructions_and_variables(
    variables: &mut HashMap<String, usize>,
    variable_memory: &mut [Value],
    instructions: &Vec<Instruction>,
) {
    println!("[variable dump] {{");
    for (variable, i) in variables.iter() {
        println!(
            "    {i}: {variable} ({})",
            variable_memory[*i].print(Some(variables))
        );
    }
    println!("}}");
    println!("[instruction dump] {{");
    for instruction in instructions {
        println!("    {}", instruction.print(Some(variables)));
    }
    println!("}}");
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
