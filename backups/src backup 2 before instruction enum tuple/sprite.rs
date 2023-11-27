use crate::{thread::Thread, interpreter::Value};

pub struct Sprite {
    pub threads: Vec<Thread>
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite { threads: vec![] }
    }

    pub fn run(&mut self, memory: &mut [Value]) {
        let mut i = 0;
        while i < self.threads.len() {
            let thread = &mut self.threads[i];
            thread.run(memory);
            // println!("{}", thread.state.get_number(&thread.state.memory[0]));

            if thread.killed {
                self.threads.remove(i);
            } else {
                i += 1;
            }
        }
    }
}