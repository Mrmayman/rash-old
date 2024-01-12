use std::collections::HashMap;

use crate::{ansi_codes, interpreter::Value};

pub struct VariableCompiler {
    lookup: HashMap<String, usize>,
    data: Vec<Value>,
}

impl VariableCompiler {
    pub fn new() -> Self {
        Self {
            lookup: HashMap::new(),
            data: Vec::new(),
        }
    }

    pub fn load_from_json(&mut self, sprite_json: &serde_json::Value) {
        for (variable_hash, variable_data) in sprite_json["variables"].as_object().unwrap() {
            self.data.push({
                match &variable_data.as_array().unwrap()[1] {
                    serde_json::Value::Bool(n) => Value::Boolean(*n),
                    serde_json::Value::Number(n) => Value::Number(n.as_f64().unwrap()),
                    serde_json::Value::String(n) => Value::String(n.clone()),
                    _ => panic!(),
                }
            });
            self.lookup
                .insert(variable_hash.clone(), self.data.len() - 1);
        }
    }

    pub fn finish_processing(self) -> Box<[Value]> {
        self.data.into_boxed_slice()
    }

    pub fn push(&mut self, name: String, value: Value) {
        let var_number = self.lookup.len();
        self.lookup.entry(name).or_insert(var_number);
        self.data.push(value);
    }

    pub fn get_id(&self, name: &str) -> Option<usize> {
        self.lookup.get(name).copied()
    }

    pub fn get_name(&self, id: usize) -> Option<&String> {
        self.lookup.iter().find(|(_, &v)| v == id).map(|(k, _)| k)
    }

    pub fn dump(&self) {
        for (variable, i) in self.lookup.iter() {
            println!(
                "    {}{i}: {}{variable}{} ({})",
                ansi_codes::YELLOW,
                ansi_codes::WHITE,
                ansi_codes::RESET,
                self.data[*i].print(Some(&self))
            );
        }
    }
}
