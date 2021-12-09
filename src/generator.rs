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
            version: "1.0".to_string(),
            image: "abulang/abusim-goabu-agent:latest".to_string(),
            coordinator_image: "abulang/abusim-coordinator:latest".to_string(),
            namespace: "abusim-example".to_string(),
            includes: vec![],
            agents: generate_devices(&opt),
            prototypes: generate_prototypes(&opt),
        }
    }
}

fn generate_devices(opt: &Opt) -> HashMap<String, Agent> {
    (0..(opt.devices_number))
        .map(|id| {
            (
                id.to_string(),
                Agent {
                    prototype: "agent".to_string(),
                    memory_controller: "basic".to_string(),
                    memory: vec![
                        String::from(format!("integer:id:{}", id)),
                        "bool:start:false".to_string(),
                    ],
                    rules: vec![],
                    tick: "1s".to_string(),
                },
            )
        })
        .collect()
}

fn generate_prototypes(opt: &Opt) -> HashMap<String, Prototype> {
    let mut prototypes = HashMap::new();
    let transform = get_rules_generator(&opt);

    prototypes.insert(
        "agent".to_string(),
        Prototype {
            memory_controller: "basic".to_string(),
            memory: (0..(opt.chain_length))
                .map(|index| String::from(format!("integer:a{}:0", index)))
                .chain([
                    String::from(format!(
                        "rule start on start for start do a0 = {}; start = false",
                        opt.devices_length
                    )),
                    String::from(format!(
                        "rule activate on a{0} \
                                for all this.a{0} > 0 && (\
                                    ext.id == (this.id + 1) || (\
                                        this.id == {1} && ext.id == 0\
                                    )\
                                ) \
                                do ext.a0 = (this.a{0} - 1)",
                        opt.chain_length - 1,
                        opt.devices_number - 1
                    )),
                ])
                .collect(),
            rules: (0..(opt.chain_length)).map(transform).collect(),
            tick: "1s".to_string(),
        },
    );

    prototypes
}

/// Returns a closure that maps the rule's index to a rule
fn get_rules_generator(opt: &Opt) -> impl Fn(u32) -> String + '_ {
    |index: u32| {
        if index == opt.chain_length - 1 {
            String::from(format!(
                "rule last_step on a{0} \
                    for a{0} > 0 \
                    do a{0} = 0",
                index
            ))
        } else {
            String::from(format!(
                "rule step{0} on a{0} \
                    for a{0} > 0 \
                    do a{1} = a{0}; a{0} = 0",
                index,
                index + 1
            ))
        }
    }
}
