use std::collections::HashMap;

pub type TypeMap = HashMap<String, HashMap<String, String>>;

#[derive(Debug)]
pub struct TypeChart {
    type_map: TypeMap,
    type_list: Vec<String>,
}

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
            current_effectiveness_map.insert(type_name.clone(), "?".to_string());
        }
        let mut new_effectiveness_map = HashMap::new();
        for current_type in &self.type_list {
            new_effectiveness_map.insert(current_type.clone(), "?".to_string());
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
        effectiveness_map.insert(opposing_type_name.clone(), effectiveness.clone());
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
            if effectiveness == "?" {
                continue;
            }
            let type_list = match reverse_effectiveness_map.get_mut(effectiveness) {
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
            if effectiveness == "?" {
                continue;
            }
            let type_list = match reverse_effectiveness_map.get_mut(effectiveness) {
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
}
