use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};

use crate::{
    ansi_codes,
    interpreter::{Instruction, Value},
    pen_line,
    project::base::{get_scaled_point, get_sprite_rect},
    project_state::ProjectState,
    sprite::{Costume, GraphicalProperties},
};

pub struct Thread {
    instructions: Box<[Instruction]>,
    pub killed: bool,
    counter: usize,
}

impl<'a> Thread {
    pub fn new(instructions: Box<[Instruction]>) -> Thread {
        Thread {
            instructions,
            killed: false,
            counter: 0,
        }
    }

    pub fn run(
        &mut self,
        memory: &mut [Value],
        properties: &mut GraphicalProperties,
        costumes: &Vec<Costume<'a>>,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        pen_canvas: &mut ProjectState,
    ) {
        loop {
            let should_break: bool =
                self.run_bytecode(memory, properties, costumes, canvas, pen_canvas);
            self.counter += 1;
            if should_break {
                break;
            }
        }
    }

    fn get_place(&self, string: &String) -> Option<usize> {
        for (index, item) in self.instructions.iter().enumerate() {
            if let Instruction::FlowDefinePlace(n) = item {
                if n == string {
                    return Some(index);
                }
            }
        }
        None
    }

    fn run_bytecode(
        &mut self,
        memory: &mut [Value],
        properties: &mut GraphicalProperties,
        costumes: &Vec<Costume<'a>>,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        project_state: &mut ProjectState,
    ) -> bool {
        match &self.instructions[self.counter] {
            Instruction::MemoryDump => {
                println!("{}[memory dump]{} {{", ansi_codes::GREEN, ansi_codes::RESET);
                dump_memory(memory);
                println!("}}")
            }
            Instruction::MemoryStore(location, value) => {
                let location = location.get_pointer();
                let value_read: Value;
                if let Value::Pointer(n) = value {
                    value_read = memory[*n].clone();
                } else {
                    value_read = value.clone();
                }
                memory[location] = value_read;
            }
            Instruction::ThreadPause => {
                return true;
            }
            Instruction::ThreadKill => {
                self.killed = true;
                return true;
            }
            Instruction::OperatorModulo(location, a, b) => {
                let location = location.get_pointer();
                let a = a.get_number(memory);
                let b = b.get_number(memory);
                memory[location] = Value::Number(a.rem_euclid(b));
            }
            Instruction::OperatorAdd(location, a, b) => {
                let location = location.get_pointer();
                let a = a.get_number(memory);
                let b = b.get_number(memory);
                memory[location] = Value::Number(a + b);
            }
            Instruction::OperatorSubtract(location, a, b) => {
                let location = location.get_pointer();
                let a = a.get_number(memory);
                let b = b.get_number(memory);
                memory[location] = Value::Number(a - b);
            }
            Instruction::OperatorMultiply(location, a, b) => {
                let location = location.get_pointer();
                let a = a.get_number(memory);
                let b = b.get_number(memory);
                memory[location] = Value::Number(a * b);
            }
            Instruction::OperatorDivide(location, a, b) => {
                let location = location.get_pointer();
                let a = a.get_number(memory);
                let b = b.get_number(memory);
                memory[location] = Value::Number(a / b);
            }
            Instruction::OperatorPower(location, a, b) => {
                let location = location.get_pointer();
                let a = a.get_number(memory);
                let b = b.get_number(memory);
                memory[location] = Value::Number(a.powf(b));
            }
            Instruction::OperatorERaised(l, n) => {
                memory[l.get_pointer()] = Value::Number(n.get_number(memory).exp())
            }
            Instruction::OperatorSin(l, n) => {
                memory[l.get_pointer()] =
                    Value::Number((n.get_number(memory) * std::f64::consts::PI / 180.0).sin())
            }
            Instruction::OperatorCos(l, n) => {
                memory[l.get_pointer()] =
                    Value::Number((n.get_number(memory) * std::f64::consts::PI / 180.0).cos())
            }
            Instruction::OperatorTan(l, n) => {
                memory[l.get_pointer()] =
                    Value::Number((n.get_number(memory) * std::f64::consts::PI / 180.0).tan())
            }
            Instruction::OperatorAbs(l, n) => {
                memory[l.get_pointer()] = Value::Number(n.get_number(memory).abs())
            }
            Instruction::OperatorASin(l, n) => {
                memory[l.get_pointer()] =
                    Value::Number(n.get_number(memory).asin() * (180.0 / std::f64::consts::PI))
            }
            Instruction::OperatorACos(l, n) => {
                memory[l.get_pointer()] =
                    Value::Number(n.get_number(memory).acos() * (180.0 / std::f64::consts::PI))
            }
            Instruction::OperatorATan(l, n) => {
                memory[l.get_pointer()] =
                    Value::Number(n.get_number(memory).atan() * (180.0 / std::f64::consts::PI))
            }
            Instruction::OperatorSqrt(l, n) => {
                let num = n.get_number(memory);
                memory[l.get_pointer()] = if num < 0.0 {
                    Value::Number(0.0)
                } else {
                    Value::Number(num.sqrt())
                }
            }
            Instruction::OperatorLn(l, n) => {
                memory[l.get_pointer()] = Value::Number(n.get_number(memory).ln())
            }
            Instruction::OperatorLog(l, n) => {
                memory[l.get_pointer()] = Value::Number(n.get_number(memory).log10())
            }
            Instruction::OperatorFloor(l, n) => {
                memory[l.get_pointer()] = Value::Number(n.get_number(memory).floor())
            }
            Instruction::OperatorCeiling(l, n) => {
                memory[l.get_pointer()] = Value::Number(n.get_number(memory).ceil())
            }
            Instruction::OperatorLesser(location, a, b) => {
                let location = location.get_pointer();
                let a = a.get_number(memory);
                let b = b.get_number(memory);
                memory[location] = Value::Boolean(a < b);
            }
            Instruction::OperatorGreater(location, a, b) => {
                let location = location.get_pointer();
                let a = a.get_number(memory);
                let b = b.get_number(memory);
                memory[location] = Value::Boolean(a > b);
            }
            Instruction::OperatorEquals(location, a, b) => {
                let a = if let Value::Pointer(n) = a {
                    &memory[*n]
                } else {
                    a
                };
                let b = if let Value::Pointer(n) = b {
                    &memory[*n]
                } else {
                    b
                };
                let location = location.get_pointer();
                memory[location] = Value::Boolean(match &(a, b) {
                    (Value::Number(n1), Value::Number(n2)) => *n1 == *n2,
                    (Value::Boolean(n1), Value::Boolean(n2)) => *n1 == *n2,
                    (Value::String(n1), Value::String(n2)) => *n1 == *n2,
                    (Value::Number(_), Value::Boolean(n2)) => a.get_bool(memory) == *n2,
                    (Value::Boolean(n1), Value::Number(_)) => *n1 == b.get_bool(memory),
                    (Value::String(_), Value::Number(n2)) => a.get_number(memory) == *n2,
                    (Value::Number(n1), Value::String(_)) => *n1 == b.get_number(memory),
                    (Value::Boolean(n1), Value::String(_)) => *n1 == b.get_bool(memory),
                    (Value::String(_), Value::Boolean(n2)) => a.get_bool(memory) == *n2,
                    _ => panic!(
                        "Unsupported types for equality comparison at {}",
                        self.counter
                    ),
                })
            }
            Instruction::SensingTimer(location) => {
                memory[location.get_pointer()] =
                    Value::Number(project_state.scratch_timer.elapsed().as_secs_f64())
            }
            Instruction::FlowIfJump(condition, location) => {
                if condition.get_bool(memory) {
                    let location = location.get_number(memory) as usize;
                    self.counter = location;
                }
            }
            Instruction::FlowIfJumpToPlace(condition, place) => {
                if condition.get_bool(memory) {
                    let location = self
                        .get_place(place)
                        .expect("Could not find jump point in program");
                    self.counter = location;
                }
            }
            Instruction::FlowIfNotJump(condition, location) => {
                if !condition.get_bool(memory) {
                    let location = location.get_number(memory) as usize;
                    self.counter = location;
                }
            }
            Instruction::FlowIfNotJumpToPlace(condition, place) => {
                if !condition.get_bool(memory) {
                    let location = self.get_place(place).unwrap_or_else(|| {
                        panic!("Could not find jump point in program {}", place)
                    });
                    self.counter = location;
                }
            }
            Instruction::FlowDefinePlace(_) => {}
            Instruction::MotionChangeX(n) => {
                let mut new_x = properties.x + n.get_number(memory);
                let mut new_y = properties.y;
                fencing_clamp(
                    &mut new_x,
                    &mut new_y,
                    costumes[properties.costume_number].data.query(),
                );
                pen_line::draw(canvas, project_state, properties, new_x, new_y);
                properties.x = new_x;
            }
            Instruction::MotionChangeY(n) => {
                let mut new_x = properties.x;
                let mut new_y = properties.y + n.get_number(memory);
                fencing_clamp(
                    &mut new_x,
                    &mut new_y,
                    costumes[properties.costume_number].data.query(),
                );
                pen_line::draw(canvas, project_state, properties, new_x, new_y);
                properties.y = new_y;
            }
            Instruction::MotionSetX(x) => {
                let mut new_x = x.get_number(memory);
                let mut new_y = properties.y;
                fencing_clamp(
                    &mut new_x,
                    &mut new_y,
                    costumes[properties.costume_number].data.query(),
                );
                pen_line::draw(canvas, project_state, properties, new_x, new_y);
                properties.x = new_x;
            }
            Instruction::MotionSetY(y) => {
                let mut new_x = properties.x;
                let mut new_y = y.get_number(memory);
                fencing_clamp(
                    &mut new_x,
                    &mut new_y,
                    costumes[properties.costume_number].data.query(),
                );
                pen_line::draw(canvas, project_state, properties, new_x, new_y);
                properties.y = new_y;
            }
            Instruction::MotionSetXY(x, y) => {
                let mut new_x = x.get_number(memory);
                let mut new_y = y.get_number(memory);
                fencing_clamp(
                    &mut new_x,
                    &mut new_y,
                    costumes[properties.costume_number].data.query(),
                );
                pen_line::draw(canvas, project_state, properties, new_x, new_y);
                properties.x = new_x;
                properties.y = new_y;
            }
            Instruction::LooksSetSize(size) => properties.size = size.get_number(memory) as f32,
            Instruction::LooksSetCostume(costume_val) => {
                let costume_name = costume_val.get_string(memory);
                match costumes
                    .iter()
                    .position(|costume| costume.name == costume_name)
                {
                    // Setting costume via name.
                    // Example: set costume to "costume1".
                    Some(n) => {
                        properties.costume_number = n;
                        // println!("costume name: {}", properties.costume_number);
                    }
                    // Setting costume via number.
                    // Example: set costume to 1.
                    None => {
                        let number_of_costumes = costumes.len() as i32;
                        let costume_number = costume_val.get_number(memory) as i32;
                        properties.costume_number =
                            ((costume_number - 1).rem_euclid(number_of_costumes)) as usize;
                        // println!("costume number: {}", properties.costume_number)
                    }
                }
            }
            Instruction::LooksGetCostumeNumber(location) => {
                memory[location.get_pointer()] =
                    Value::Number(properties.costume_number as f64 + 1.0);
            }
            Instruction::MotionGetX(location) => {
                memory[location.get_pointer()] = Value::Number(properties.x)
            }
            Instruction::MotionGetY(location) => {
                memory[location.get_pointer()] = Value::Number(properties.y)
            }
            Instruction::PenClear => {
                canvas
                    .with_texture_canvas(&mut project_state.main_canvas, |texture_canvas| {
                        texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
                        texture_canvas.clear();
                    })
                    .unwrap();
            }
            Instruction::PenStamp => {
                let size = canvas.output_size().unwrap();
                canvas
                    .with_texture_canvas(&mut project_state.main_canvas, |texture_canvas| {
                        texture_canvas
                            .copy(
                                &costumes[properties.costume_number].data,
                                None,
                                get_sprite_rect(
                                    properties,
                                    &costumes[properties.costume_number],
                                    size,
                                ),
                            )
                            .unwrap();
                    })
                    .unwrap();
            }
            Instruction::PenUp => properties.pen_down = false,
            Instruction::PenDown => properties.pen_down = true,
            Instruction::PenSetRadius(value) => {
                properties.pen_radius = value.get_number(memory) as i32 * 2
            }
            Instruction::LooksHide => properties.shown = false,
            Instruction::LooksShow => properties.shown = true,
        }
        false
    }
}

fn fencing_clamp(new_x: &mut f64, new_y: &mut f64, query: sdl2::render::TextureQuery) {
    if *new_x > 240.0 {
        if query.width > 32 {
            *new_x = 240.0 + (query.width / 2) as f64 - 15.0;
        } else {
            *new_x = 240.0
        }
    }
    if *new_y > 180.0 {
        if query.height > 32 {
            *new_y = 180.0 + (query.height / 2) as f64 - 15.0;
        } else {
            *new_y = 180.0
        }
    }
}

fn dump_memory(memory: &[Value]) {
    for val in memory {
        match val {
            Value::Pointer(n) => {
                println!("    pointer: {}", n)
            }
            Value::Number(n) => {
                println!("    number: {}", n)
            }
            Value::Boolean(n) => {
                println!("    bool: {}", n)
            }
            Value::String(n) => {
                println!("    string: {}", n)
            }
        }
    }
}
