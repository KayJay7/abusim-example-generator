use crate::args::Opt;
use serde::Serialize;
use std::collections::HashMap;

/// A serializable struct to contain the generated example
/// this will be serialized as YAML in the output file
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

/// Like `Config` but for agents
#[derive(Debug, PartialEq, Serialize, Clone)]
struct Agent {
    prototype: String,
    memory_controller: String,
    memory: Vec<String>,
    rules: Vec<String>,
    tick: String,
}

/// Like `Config` but for prototypes
#[derive(Debug, PartialEq, Serialize, Clone)]
struct Prototype {
    memory_controller: String,
    memory: Vec<String>,
    rules: Vec<String>,
    tick: String,
}

impl Config {
    /// Empty constructor
    #[allow(dead_code)]
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

    /// Generate a configuration based on argument options
    pub fn from(opt: Opt) -> Config {
        Config {
            version: opt.configuration_version.clone(),
            image: opt.image.clone(),
            coordinator_image: opt.coordinator_image.clone(),
            namespace: opt.namespace.clone(),
            includes: opt.includes.clone(),
            agents: generate_devices(&opt),
            prototypes: generate_prototypes(&opt),
        }
    }
}

fn generate_devices(opt: &Opt) -> HashMap<String, Agent> {
    (0..(opt.devices_number))
        .map(|id| {
            let mut rules = Vec::with_capacity(opt.chains_number as usize);
            let mut activated_devices = format!("ext.id == {}", (id + 0 + 1) % opt.devices_number);

            for device_index in 1..opt.devices_width {
                activated_devices.push_str(&format!(
                    " || ext.id == {}",
                    (id + device_index + 1) % opt.devices_number
                ));
            }

            for chain_index in 0..opt.chains_number {
                rules.push(format!(
                    "rule activate{0} on a{0}_{1} \
                    for all this.a{0}_{1} > 0 && ({2}) \
                    do ext.a{0}_0 = (this.a{0}_{1} - 1)",
                    chain_index,
                    opt.chain_length - 1,
                    activated_devices
                ));
            }

            (
                format!("agent{}", id),
                Agent {
                    prototype: "agent".to_string(),
                    memory_controller: opt.memory_controller.clone(),
                    memory: vec![format!("integer:id:{}", id)],
                    rules,
                    tick: opt.tick.clone(),
                },
            )
        })
        .collect()
}

fn generate_prototypes(opt: &Opt) -> HashMap<String, Prototype> {
    let mut prototypes = HashMap::new();

    prototypes.insert(
        "agent".to_string(),
        Prototype {
            memory_controller: opt.memory_controller.clone(),
            memory: generate_memory(&opt),
            rules: generate_rules(&opt),
            tick: opt.tick.clone(),
        },
    );

    prototypes
}

/// Maps the rule's indexes to a rule
fn get_rule(opt: &Opt, chain_index: u32, step_index: u32) -> String {
    if step_index == opt.chain_length - 1 {
        format!(
            "rule last_step{0} on a{0}_{1} \
            for a{0}_{1} > 0 \
            do a{0}_{1} = 0",
            chain_index, step_index
        )
    } else {
        format!(
            "rule step{0}_{1} on a{0}_{1} \
            for a{0}_{1} > 0 \
            do a{0}_{2} = a{0}_{1}; a{0}_{1} = 0",
            chain_index,
            step_index,
            step_index + 1
        )
    }
}

fn generate_memory(opt: &Opt) -> Vec<String> {
    let mut memory =
        Vec::with_capacity((opt.chain_length as usize * opt.chains_number as usize) + 2);
    memory.push("bool:start:false".to_string());
    memory.push("bool:start_all:false".to_string());

    for chain_index in 0..opt.chains_number {
        for step_index in 0..opt.chain_length {
            memory.push(format!("integer:a{}_{}:0", chain_index, step_index));
        }
    }

    memory
}

fn generate_rules(opt: &Opt) -> Vec<String> {
    let mut rules =
        Vec::with_capacity((opt.chain_length as usize * opt.chains_number as usize) + 3);
    let mut starting_rule = String::new();

    for chain_index in 0..opt.chain_width {
        starting_rule.push_str(&format!(" a{}_0 = {};", chain_index, opt.devices_length));
    }

    rules.push(format!(
        "rule start on start for start do{} start = false",
        starting_rule
    ));
    rules
        .push("rule start_all on start_all for all this.start_all do ext.start = true".to_string());
    rules.push(
        "rule start_local on start_all for start_all do start = true; start_all = false"
            .to_string(),
    );

    for chain_index in 0..opt.chains_number {
        for step_index in 0..opt.chain_length {
            rules.push(get_rule(&opt, chain_index, step_index));
        }
    }

    rules
}
