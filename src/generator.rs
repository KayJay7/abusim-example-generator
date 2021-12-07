use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Config {
    version: String,
    image: String,
    coordinator_image: String,
    namespace: String,
    includes: Vec<String>,
    agents: HashMap<String, Agent>,
    prototypes: HashMap<String, Prototype>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
struct Agent {
    prototype: String,
    memory_controller: String,
    memory: Vec<String>,
    rules: Vec<String>,
    tick: String,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
struct Prototype {
    memory_controller: String,
    memory: Vec<String>,
    rules: Vec<String>,
    tick: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            version: "1.0".to_string(),
            image: "abulang/abusim-goabu-agent:latest".to_string(),
            coordinator_image: "abulang/abusim-coordinator:latest".to_string(),
            namespace: "abusim-example".to_string(),
            includes: vec![],
            agents: HashMap::new(),
            prototypes: HashMap::new(),
        }
    }
}
