use std::collections::HashMap;

use crate::type_chart::{TypeChart, TypeMap};

pub fn get_types_from_file(filepath: &String) -> Result<TypeChart, ()> {
    // Maybe should handle empty file? => return empty typechart
    match std::fs::exists(filepath) {
        Err(err) => {
            eprintln!("Could not check if file exists :");
            eprintln!("{}", err.to_string());
            return Err(());
        },
        Ok(false) => return Ok(TypeChart::empty()),
        _ => (),
    }
    let mut reader_builder = csv::ReaderBuilder::new();
    // Disable headers to handle them manually
    let mut file_reader = match reader_builder.has_headers(false).from_path(filepath) {
        Err(err) => {
            eprintln!("Could not open and read the file:");
            eprintln!("{}", err.to_string());
            return Err(());
        },
        Ok(file_content) => file_content,
    };
    let mut type_list: Vec<String> = Vec::new();
    let mut hashmap: TypeMap = HashMap::new();
    
    let mut index: usize = 0;
    for line in file_reader.records() {
        let line = match line {
            Err(err) => {
                eprintln!("There was an error on line {}:", index);
                eprintln!("{}", err.to_string());
                continue;
            }
            Ok(line) => line,
        };
        if index == 0 {
            for new_type in line.iter() {
                hashmap.insert(new_type.trim().to_string(), HashMap::new());
                type_list.push(new_type.trim().to_string());
            }
        } else {
            let current_type = match type_list.get(index - 1) {
                None => {
                    // Can only happen if there are more lines than types
                    eprintln!("Error while trying to load file : There are more lines than types");
                    return Err(());
                },
                Some(current_type) => current_type,
            };

            let type_matchups = match hashmap.get_mut(current_type) {
                None => {
                    // Should never happen?
                    eprintln!("The type existed in the list but not in the hashmap");
                    return Err(());
                },
                Some(type_matchups) => type_matchups,
            };

            let mut line_index: usize = 0;
            for effectiveness in line.iter() {
                let opposing_type = match type_list.get(line_index) {
                    None => {
                        // Can only happen if there are more lines than types
                        eprintln!("Error while trying to load file : There are more lines than types");
                        return Err(());
                    },
                    Some(opposing_type) => opposing_type,
                };
                let effectiveness_value: f32 = match effectiveness.trim() {
                    "Immune" => 0.,
                    "Not Very Effective" => 0.5,
                    "Neutral" => 1.,
                    "Super Effective" => 2.,
                    _ => {
                        // Can only happen if the file contains incorrect effectivenesses
                        eprintln!("Error while trying to load file : Effectiveness {} doesn't exist", effectiveness);
                        return Err(());
                    }
                };
                type_matchups.insert(opposing_type.clone(), effectiveness_value);
                line_index += 1;
            }
        }
        index += 1;
    }

    return Ok(TypeChart::new(hashmap, type_list));
}

pub fn save_types_to_file(type_chart: &TypeChart, filepath: &String) -> Result<(), ()> {
    if type_chart.is_empty() {
        if let Err(err) = std::fs::File::create(filepath) {
            eprintln!("Could not create file to store empty type chart :");
            eprintln!("{}", err.to_string());
            return Err(());
        }
        return Ok(());
    }
    let mut writer_builder = csv::WriterBuilder::new();
    let mut file_writer = match writer_builder.has_headers(false).from_path(filepath) {
        Err(err) => {
            eprintln!("Could not open and write the file:");
            eprintln!("{}", err.to_string());
            return Err(());
        },
        Ok(file_writer) => file_writer,
    };
    // First write the type list
    let type_list = type_chart.get_type_list();
    if let Err(err) = file_writer.write_record(&type_list) {
        eprintln!("Error while trying to write the types in the file:");
        eprintln!("{}", err.to_string());
        return Err(());
    }

    let type_map =  type_chart.get_type_map();
    for current_type in &type_list {
        // Converting the map into a list ordered the same way that the type list is
        let effectiveness_map = match type_map.get(current_type) {
            None => {
                eprintln!("Error while trying to get effectiveness of type {}", current_type);
                return Err(());
            },
            Some(effectiveness_map) => effectiveness_map,
        };
        let mut effectiveness_list: Vec<String> = Vec::new();
        for opposing_type in &type_list {
            let effectiveness = match effectiveness_map.get(opposing_type) {
                None => {
                    eprintln!("Error while trying to get effectiveness of type {} versus {}", current_type, opposing_type);
                    return Err(());
                },
                Some(effectiveness) => effectiveness,
            };
            effectiveness_list.push(
                match effectiveness {
                    0. => "Immune",
                    0.5 => "Not Very Effective",
                    1. => "Neutral",
                    2. => "Super Effective",
                    _ => {
                        // Should never happen
                        eprintln!("Error while trying to save type chart to file: Effectiveness {} doesn't exist", effectiveness);
                        return Err(());
                    }
                }.to_string()
            );
        }
        if let Err(err) = file_writer.write_record(&effectiveness_list) {
            eprintln!("Error while trying to write the effectiveness of type {}", current_type);
            eprintln!("{}", err.to_string());
            return Err(());
        }
    }

    return Ok(())
}
