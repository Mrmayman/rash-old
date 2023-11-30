use crate::{
    interpreter::{Instruction, Value},
    project::base::get_sprite_rect,
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
        pen_canvas: &mut sdl2::render::Texture,
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
        pen_canvas: &mut sdl2::render::Texture,
    ) -> bool {
        match &self.instructions[self.counter] {
            Instruction::MemoryDump => {
                println!("[memory dump] {{");
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
                properties.x += n.get_number(memory);
            }
            Instruction::MotionChangeY(n) => properties.y += n.get_number(memory),
            Instruction::MotionSetX(x) => properties.x = x.get_number(memory),
            Instruction::MotionSetY(y) => properties.y = y.get_number(memory),
            Instruction::LooksSetSize(size) => properties.size = size.get_number(memory) as f32,
            Instruction::LooksSetCostume(costume_val) => {
                let costume_name = costume_val.get_string(memory);
                match costumes
                    .iter()
                    .position(|costume| costume.name == costume_name)
                {
                    Some(n) => {
                        properties.costume_number = n;
                        // println!("costume name: {}", properties.costume_number);
                    }
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
                    .with_texture_canvas(pen_canvas, |texture_canvas| {
                        texture_canvas.clear();
                    })
                    .unwrap();
            }
            Instruction::PenStamp => {
                let size = canvas.output_size().unwrap();
                canvas
                    .with_texture_canvas(pen_canvas, |texture_canvas| {
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
        }
        false
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
