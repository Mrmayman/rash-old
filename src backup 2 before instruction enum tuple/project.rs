use std::io::Read;

use tempfile::TempDir;

use crate::{
    interpreter::{Instruction, Value},
    sprite::Sprite,
    thread::Thread,
};

pub struct Project {
    temp_directory: TempDir,
    memory: Box<[Value]>,
    sprites: Vec<Sprite>,
    pub path: std::path::PathBuf,
    pub json: serde_json::Value,
}

impl Project {
    pub fn new(file_path: String) -> Result<Project, String> {
        let (temp_dir, temp_dir_path) = Project::load_project_file(file_path);
        let json = Project::parse(&temp_dir_path);

        let mut temp_project = Project {
            temp_directory: temp_dir,
            memory: Box::new([]),
            sprites: vec![],
            path: temp_dir_path,
            json,
        };

        /*println!(
            "{}",
            serde_json::to_string_pretty(&json).expect("Could not print project.json")
        );*/

        for sprite in temp_project.json["targets"]
            .as_array()
            .expect("Malformed JSON - No \"targets\" list of sprites")
        {}

        temp_project.load_pi_project();

        Ok(temp_project)
    }

    fn load_pi_project(&mut self) {
        self.sprites.push(Sprite::new());
        self.sprites[0].threads.push(Thread::new(
            vec![
                Value::Bytecode(Instruction::MemoryStore),
                Value::Pointer(0),
                Value::Number(0.0),
                Value::Bytecode(Instruction::MemoryStore),
                Value::Pointer(2),
                Value::Number(1.0),
                Value::Bytecode(Instruction::MemoryStore),
                Value::Pointer(4),
                Value::Number(0.0),
                Value::Bytecode(Instruction::OperatorModulo),
                Value::Pointer(5),
                Value::Pointer(4),
                Value::Number(2.0),
                Value::Bytecode(Instruction::OperatorMultiply),
                Value::Pointer(5),
                Value::Pointer(5),
                Value::Number(2.0),
                Value::Bytecode(Instruction::OperatorSubtract),
                Value::Pointer(3),
                Value::Pointer(5),
                Value::Number(1.0),
                Value::Bytecode(Instruction::OperatorDivide),
                Value::Pointer(5),
                Value::Number(4.0),
                Value::Pointer(2),
                Value::Bytecode(Instruction::OperatorMultiply),
                Value::Pointer(5),
                Value::Pointer(3),
                Value::Pointer(5),
                Value::Bytecode(Instruction::OperatorAdd),
                Value::Pointer(0),
                Value::Pointer(0),
                Value::Pointer(5),
                Value::Bytecode(Instruction::OperatorAdd),
                Value::Pointer(2),
                Value::Pointer(2),
                Value::Number(2.0),
                Value::Bytecode(Instruction::OperatorAdd),
                Value::Pointer(4),
                Value::Pointer(4),
                Value::Number(1.0),
                Value::Bytecode(Instruction::OperatorLesser),
                Value::Pointer(5),
                Value::Pointer(4),
                Value::Number(1_000_000.0),
                Value::Bytecode(Instruction::FlowIfJump),
                Value::Pointer(5),
                Value::Number(8.0),
                // Value::Bytecode(Instruction::MemoryDump),
                Value::Bytecode(Instruction::ThreadPause),
                Value::Bytecode(Instruction::ThreadKill),
            ]
            .into_boxed_slice(),
        ));
        self.malloc(16);
    }

    fn parse(path: &std::path::PathBuf) -> serde_json::Value {
        let mut file = std::fs::File::open(&path.join("project.json"))
            .expect("Could not open project.json file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Could not read project.json file");

        serde_json::from_str(&content).expect("Could not parse project.json")
    }

    fn read_file_to_bytes(path: &str) -> std::io::Result<Vec<u8>> {
        // Open the file
        let mut file = std::fs::File::open(path)?;

        // Read the contents into a Vec<u8>
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    fn load_project_file(file_path: String) -> (TempDir, std::path::PathBuf) {
        // Create a temporary directory
        let temp_dir = tempfile::TempDir::new().expect("Failed to create temporary directory");

        // Get the path of the temporary directory
        let temp_dir_path: std::path::PathBuf = temp_dir.path().to_path_buf();

        let archive: Vec<u8> =
            Project::read_file_to_bytes(&file_path.as_str()).expect("Could not read .sb3 file");
        let target_dir = std::path::PathBuf::from(&temp_dir_path);
        // Doesn't need to exist

        // The third parameter allows you to strip away toplevel directories.
        // If `archive` contained a single folder, that folder's contents would be extracted instead.
        zip_extract::extract(std::io::Cursor::new(archive), &target_dir, true)
            .expect("Could not extract .sb3 zip");
        (temp_dir, temp_dir_path)
    }

    pub fn malloc(&mut self, size: usize) {
        self.memory = vec![Value::Number(0.0); size].into_boxed_slice();
    }

    pub fn run(&mut self) {
        for sprite in &mut self.sprites {
            sprite.run(&mut self.memory);
        }
    }
}
