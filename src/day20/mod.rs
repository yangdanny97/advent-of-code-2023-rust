use std::collections::HashMap;
use std::collections::VecDeque;

fn input() -> String {
    String::from(
        r#"
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
    "#,
    )
}

// the second example input
fn _input() -> String {
    String::from(
        r#"
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    "#,
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    Broadcaster(Broadcaster),
    Inv(Inv),
    Conj(Conj),
    FlipFlop(FlipFlop),
}

#[derive(Debug)]
enum ModuleType {
    Broadcaster,
    Conj,
    FlipFlop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Broadcaster {
    outputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Inv {
    input: String,
    outputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Conj {
    inputs: Vec<String>,
    outputs: Vec<String>,
    memory: Vec<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FlipFlop {
    input: String,
    outputs: Vec<String>,
    state: bool,
}

pub fn part1() {
    let input = input();
    let input_str = input.as_str();
    let mut outputs: HashMap<&str, Vec<String>> = HashMap::new();
    let mut inputs: HashMap<&str, Vec<String>> = HashMap::new();
    let mut types: HashMap<&str, ModuleType> = HashMap::new();
    inputs.insert("broadcaster", vec![]);
    outputs.insert("output", vec![]);
    for line in input_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let parts = trimmed.split(" -> ").collect::<Vec<_>>();
        let input = parts[0];
        let output_list = parts[1].split(", ").collect::<Vec<_>>();
        let module_type = if input.starts_with('&') {
            ModuleType::Conj
        } else if input.starts_with('%') {
            ModuleType::FlipFlop
        } else {
            ModuleType::Broadcaster
        };
        let module_name = if input.starts_with('&') || input.starts_with('%') {
            &input[1..]
        } else {
            input
        };
        types.insert(module_name, module_type);
        outputs.insert(
            module_name,
            output_list
                .iter()
                .map(|&s| s.to_string())
                .collect::<Vec<_>>(),
        );
        for &o in output_list.iter() {
            inputs
                .entry(o)
                .and_modify(|vec| vec.push(module_name.to_string()))
                .or_insert(vec![module_name.to_string()]);
        }
    }
    let mut modules: HashMap<String, Module> = HashMap::new();
    for (name, t) in types {
        let i = inputs.get(name).unwrap();
        let o = outputs.get(name).unwrap();
        modules.insert(
            String::from(name),
            match t {
                ModuleType::Broadcaster => Module::Broadcaster(Broadcaster { outputs: o.clone() }),
                ModuleType::Conj => {
                    if i.len() == 1 {
                        Module::Inv(Inv {
                            input: i.first().unwrap().to_string(),
                            outputs: o.clone(),
                        })
                    } else {
                        Module::Conj(Conj {
                            inputs: i.clone(),
                            outputs: o.clone(),
                            memory: vec![false; i.len()],
                        })
                    }
                }
                ModuleType::FlipFlop => Module::FlipFlop(FlipFlop {
                    input: i.first().unwrap().to_string(),
                    outputs: o.clone(),
                    state: false,
                }),
            },
        );
    }
    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back(("button".to_string(), "broadcaster".to_string(), false));
        while let Some((from, to, ishigh)) = pulses.pop_front() {
            // println!(
            //     "{} {} {}",
            //     from,
            //     if ishigh { "-high->" } else { "-low->" },
            //     to
            // );
            if ishigh {
                high += 1;
            } else {
                low += 1;
            }
            modules.entry(to.to_string()).and_modify(|v| match v {
                Module::Broadcaster(module) => {
                    for o in module.outputs.iter() {
                        pulses.push_back(("broadcaster".to_string(), o.clone(), ishigh));
                    }
                }
                Module::Inv(module) => {
                    for o in module.outputs.iter() {
                        pulses.push_back((to.to_string(), o.clone(), !ishigh));
                    }
                }
                Module::FlipFlop(module) => {
                    if !ishigh {
                        module.state = !module.state;
                        for o in module.outputs.iter() {
                            pulses.push_back((to.to_string(), o.clone(), module.state));
                        }
                    }
                }
                Module::Conj(module) => {
                    let idx = module
                        .inputs
                        .iter()
                        .position(|i| i == from.as_str())
                        .unwrap();
                    module.memory[idx] = ishigh;
                    if module.memory.iter().all(|&b| b) {
                        for o in module.outputs.iter() {
                            pulses.push_back((to.to_string(), o.clone(), false));
                        }
                    } else {
                        for o in module.outputs.iter() {
                            pulses.push_back((to.to_string(), o.clone(), true));
                        }
                    }
                }
            });
        }
    }
    println!("{} {} {}", low, high, low * high)
}

/*

For part 2, since there's no solution for the example inputs here is what I did.

1. run 10000 iterations, print the count & signal source whenever a high signal is emitted to the predecessor of rx
2. find LCM of cycle lengths

*/
