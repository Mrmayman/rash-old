use crate::{code_thread::CodeThread, instructions::*, machine_state::Value, sprite::Sprite};

pub const DEBUG_OUTPUT: bool = false;

pub struct Interpreter {
    sprites: Vec<Sprite>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { sprites: vec![] }
    }

    pub fn load(&mut self) {
        self.sprites.push(Sprite::new());
        self.sprites[0].add_thread(CodeThread::new(
            vec![
                base::var_set,
                base::var_set,
                base::var_set,
                base::var_set,
                operator::modulo,
                operator::mult,
                operator::sub,
                operator::div,
                operator::mult,
                operator::add,
                operator::add,
                operator::add,
                operator::lesser,
                // base::memory_dump,
                base::flow_if,
                base::kill_thread,
            ],
            vec![
                Value::Pointer(0),
                Value::Number(0.0),
                Value::Pointer(1),
                Value::Number(4.0),
                Value::Pointer(2),
                Value::Number(1.0),
                Value::Pointer(4),
                Value::Number(0.0),
                Value::Pointer(5),
                Value::Pointer(4),
                Value::Number(2.0),
                Value::Pointer(5),
                Value::Pointer(5),
                Value::Number(2.0),
                Value::Pointer(3),
                Value::Pointer(5),
                Value::Number(1.0),
                Value::Pointer(5),
                Value::Pointer(1),
                Value::Pointer(2),
                Value::Pointer(5),
                Value::Pointer(3),
                Value::Pointer(5),
                Value::Pointer(0),
                Value::Pointer(0),
                Value::Pointer(5),
                Value::Pointer(2),
                Value::Pointer(2),
                Value::Number(2.0),
                Value::Pointer(4),
                Value::Pointer(4),
                Value::Number(1.0),
                Value::Pointer(5),
                Value::Pointer(4),
                Value::Number(10_000_000.0),
                Value::Pointer(5),
                Value::Number(5.0),
                Value::Number(8.0),
            ],
        ));
        self.sprites[0].threads[0].state.malloc(6);
    }

    pub fn run(&mut self) {
        for sprite in &mut self.sprites {
            sprite.run();
        }
    }
}
