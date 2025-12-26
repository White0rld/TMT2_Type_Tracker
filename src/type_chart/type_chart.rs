use std::collections::HashMap;

pub type TypeMap = HashMap<String, HashMap<String, f32>>;

#[derive(Debug)]
pub struct TypeChart {
    type_map: TypeMap,
    type_list: Vec<String>,
}

pub static ALL_POSSIBLE_EFFECTIVENESSES: [&str; 16] = [
    "Immune",
    "Triple Not Very Effective",
    "Double Not Very Effective", "Double Not Very Effective?",
    "Not Very Effective", "Not Very Effective?", "Not Very Effective??",
    "Neutral", "Neutral?", "Neutral??",
    "Super Effective", "Super Effective?", "Super Effective??",
    "Double Super Effective", "Double Super Effective?",
    "Triple Super Effective"
];

impl TypeChart {
    pub fn empty() -> TypeChart {
        return TypeChart { type_map: HashMap::new(), type_list: Vec::new() }
    }
    
    pub fn new(type_map: TypeMap, type_list: Vec<String>) -> TypeChart {
        return TypeChart { type_map, type_list };
    }

    pub fn is_empty(&self) -> bool {
        return self.type_list.is_empty() && self.type_map.is_empty();
    }

    pub fn get_type_list(&self) -> Vec<String> {
        return self.type_list.clone();
    }

    pub fn get_type_map(&self) -> TypeMap {
        return self.type_map.clone();
    }

    pub fn add_new_type(&mut self, type_name: &String) {
        // Check id the type already is in the list
        if self.type_list.contains(type_name) {
            eprintln!("Type is already in the type chart");
            return;
        }
        self.type_list.push(type_name.clone());
        for current_effectiveness_map in self.type_map.values_mut() {
            current_effectiveness_map.insert(type_name.clone(), -1.);
        }
        let mut new_effectiveness_map = HashMap::new();
        for current_type in &self.type_list {
            new_effectiveness_map.insert(current_type.clone(), -1.);
        }
        self.type_map.insert(type_name.clone(), new_effectiveness_map);
        println!("New type {} added", type_name);
    }

    pub fn remove_existing_type(&mut self, type_name: &String) {
        let idx = match self.type_list.iter().position(|current_type| current_type == type_name) {
            None => {
                eprintln!("There is no type named {}", type_name);
                return;
            },
            Some(idx) => idx,
        };
        self.type_list.remove(idx);
        
        for current_effectiveness_map in self.type_map.values_mut() {
            current_effectiveness_map.remove(type_name);
        }
        self.type_map.remove(type_name);
        println!("Removed type {}", type_name);
    }

    pub fn add_effectiveness(&mut self, type_name: &String, opposing_type_name: &String, effectiveness: String) {
        let effectiveness_value = effectiveness_string_to_f32(&effectiveness).expect("Effectiveness doesn't exist");
        let effectiveness_map = match self.type_map.get_mut(type_name) {
            None => {
                eprintln!("Type {} doesn't exist!", type_name);
                return;
            }
            Some(effectiveness_map) => effectiveness_map,
        };
        // Check if the opposing type exists
        if !self.type_list.contains(opposing_type_name) {
            eprintln!("Type {} doesn't exist!", opposing_type_name);
            return;
        }
        effectiveness_map.insert(opposing_type_name.clone(), effectiveness_value);
        println!("{} type attacks are now {} against {}", type_name, &effectiveness, opposing_type_name);
    }

    pub fn get_attacking_effectiveness(&mut self, type_name: &String) -> Result<HashMap<String, Vec<String>>, ()> {
        let effectiveness_map = match self.type_map.get(type_name) {
            None => {
                eprintln!("Type {} doesn't exist!", type_name);
                return Err(());
            }
            Some(effectiveness_map) => effectiveness_map,
        };
        let mut reverse_effectiveness_map: HashMap<String, Vec<String>> = HashMap::new();
        for effectiveness in vec!["Immune", "Not Very Effective", "Neutral", "Super Effective"] {
            reverse_effectiveness_map.insert(effectiveness.to_string(), Vec::new());
        }
        for (opposing_type, effectiveness) in effectiveness_map {
            if effectiveness == &-1. {
                continue;
            }
            let effectiveness_string = effectiveness_f32_to_string(*effectiveness, 0).expect("Effectiveness doesn't exist").to_string();
            let type_list = match reverse_effectiveness_map.get_mut(&effectiveness_string) {
                None => {
                    // Should not happen, unless we read a file with wrong effectivenesses
                    eprintln!("Effectiveness {} doesn't exist", effectiveness);
                    return Err(());
                },
                Some(type_list) => type_list,
            };
            type_list.push(opposing_type.clone());
        }
        return Ok(reverse_effectiveness_map);
    }

    pub fn get_defensive_effectiveness(&mut self, type_name: &String) -> Result<HashMap<String, Vec<String>>, ()> {
        if !self.type_list.contains(type_name) {
            eprintln!("Type {} isn't in the type chart", type_name);
            return Err(());
        }
        let mut reverse_effectiveness_map: HashMap<String, Vec<String>> = HashMap::new();
        for effectiveness in vec!["Immune", "Not Very Effective", "Neutral", "Super Effective"] {
            reverse_effectiveness_map.insert(effectiveness.to_string(), Vec::new());
        }
        for (opposing_type, effectiveness_map) in &self.type_map {
            let effectiveness = match effectiveness_map.get(type_name) {
                None => {
                    // Should not happen, since we checked before that the type exists
                    eprintln!("Type {} doesn't have an effectiveness with {}", type_name, opposing_type);
                    continue;
                },
                Some(effectiveness) => effectiveness,
            };
            if effectiveness == &-1. {
                continue;
            }
            let effectiveness_string = effectiveness_f32_to_string(*effectiveness, 0).expect("Effectiveness doesn't exist").to_string();
            let type_list = match reverse_effectiveness_map.get_mut(&effectiveness_string) {
                None => {
                    // Should not happen, unless we read a file with wrong effectivenesses
                    eprintln!("Effectiveness {} doesn't exist", effectiveness);
                    return Err(());
                },
                Some(type_list) => type_list,
            };
            type_list.push(opposing_type.clone());
        }
        return Ok(reverse_effectiveness_map);
    }
    pub fn get_multiple_defensive_effectiveness(&mut self, first_type_name: &String, second_type_name: Option<&String>, third_type_name: Option<&String>) -> Result<HashMap<String, Vec<String>>, ()> {
        // First check that all types are in the type list
        if !self.type_list.contains(first_type_name) {
            eprintln!("Type {} isn't in the type chart", first_type_name);
            return Err(());
        }
        let mut nb_types = 1;
        if let Some(second_type_name) = second_type_name {
            nb_types += 1;
            if !self.type_list.contains(second_type_name) {
                eprintln!("Type {} isn't in the type chart", second_type_name);
                return Err(());
            }
        }
        if let Some(third_type_name) = third_type_name {
            nb_types += 1;
            if !self.type_list.contains(third_type_name) {
                eprintln!("Type {} isn't in the type chart", third_type_name);
                return Err(());
            }
        }
        let mut reverse_effectiveness_map: HashMap<String, Vec<String>> = HashMap::new();
        for effectiveness in ALL_POSSIBLE_EFFECTIVENESSES {
            reverse_effectiveness_map.insert(effectiveness.to_string(), Vec::new());
        }
        for (opposing_type, effectiveness_map) in &self.type_map {
            let mut combined_effectiveness = match effectiveness_map.get(first_type_name) {
                None => {
                    // Should not happen, since we checked before that the type exists
                    eprintln!("Type {} doesn't have an effectiveness with {}", first_type_name, opposing_type);
                    continue;
                },
                Some(effectiveness) => *effectiveness,
            };
            let mut unknown_effectiveness_counter = 0;
            if combined_effectiveness == -1. {
                unknown_effectiveness_counter += 1;
                combined_effectiveness = 1.;
            }
            if let Some(second_type_name) = second_type_name {
                let second_effectiveness = match effectiveness_map.get(second_type_name) {
                    None => {
                        // Should not happen, since we checked before that the type exists
                        eprintln!("Type {} doesn't have an effectiveness with {}", second_type_name, opposing_type);
                        continue;
                    },
                    Some(effectiveness) => *effectiveness,
                };
                if second_effectiveness == -1. {
                    unknown_effectiveness_counter += 1;
                } else {
                    combined_effectiveness *= second_effectiveness;
                }
            }
            if let Some(third_type_name) = third_type_name {
                let third_effectiveness = match effectiveness_map.get(third_type_name) {
                    None => {
                        // Should not happen, since we checked before that the type exists
                        eprintln!("Type {} doesn't have an effectiveness with {}", third_type_name, opposing_type);
                        continue;
                    },
                    Some(effectiveness) => *effectiveness,
                };
                if third_effectiveness == -1. {
                    unknown_effectiveness_counter += 1;
                } else {
                    combined_effectiveness *= third_effectiveness;
                }
            }
            if nb_types == unknown_effectiveness_counter {
                // We don't know anything about this type
                continue;
            }
            let effectiveness_string = effectiveness_f32_to_string(combined_effectiveness, unknown_effectiveness_counter).expect("Effectiveness doesn't exist");
            match reverse_effectiveness_map.get_mut(&effectiveness_string) {
                None => {
                    // Should not happen, unless we read a file with wrong effectivenesses
                    eprintln!("Effectiveness {} doesn't exist", combined_effectiveness);
                    return Err(());
                },
                Some(type_list) => type_list.push(opposing_type.clone()),
            }
        }
        
        return Ok(reverse_effectiveness_map);
    }
}

pub fn effectiveness_string_to_f32(effectiveness: &String) -> Result<f32, ()> {
    match effectiveness.as_str() {
        "Immune" => Ok(0.),
        "Triple Not Very Effective" => Ok(0.125),
        "Double Not Very Effective" => Ok(0.25),
        "Not Very Effective" => Ok(0.5),
        "Neutral" => Ok(1.),
        "Super Effective" => Ok(2.),
        "Double Super Effective" => Ok(4.),
        "Tripel Super Effective" => Ok(8.),
        "?" => Ok(-1.), // Not sure how to represent this yet
        _ => Err(())
    }
}

pub fn effectiveness_f32_to_string(effectiveness: f32, unknown_effectiveness_counter: usize) -> Result<String, ()> {
    let mut effectiveness_string = match effectiveness {
        0. => return Ok("Immune".to_string()),
        0.125 => Ok("Triple Not Very Effective"),
        0.25 => Ok("Double Not Very Effective"),
        0.5 => Ok("Not Very Effective"),
        1. => Ok("Neutral"),
        2. => Ok("Super Effective"),
        4. => Ok("Double Super Effective"),
        8. => Ok("Triple Super Effective"),
        -1. => Ok("?"),
        _ => Err(()),
    }?.to_string();
    effectiveness_string.push_str(&"?".repeat(unknown_effectiveness_counter));
    return Ok(effectiveness_string);
}
