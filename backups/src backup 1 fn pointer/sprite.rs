use crate::code_thread::CodeThread;

pub struct Sprite {
    pub threads: Vec<CodeThread>
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite { threads: vec![] }
    }

    pub fn add_thread(&mut self, thread: CodeThread) {
        self.threads.push(thread);
    }

    pub fn run(&mut self) {
        let mut i = 0;
        while i < self.threads.len() {
            let thread = &mut self.threads[i];
            thread.run();
            println!("{}", thread.state.get_number(&thread.state.memory[0]));

            if thread.state.killed {
                self.threads.remove(i);
            } else {
                i += 1;
            }
        }
    }
}