use clap::Parser;
use std::{io,collections::HashMap};

mod files;
mod type_chart;

use type_chart::TypeChart;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("examples/types.csv"))]
    filepath: String,
}

fn get_info_from_user(display_string: &str, type_chart: &TypeChart, filepath: &String) -> Result<String, i32> {
    let stdin = io::stdin();
    let mut user_input = String::new();
    loop  {
        println!("{}", display_string);
        let _ = match stdin.read_line(&mut user_input) {
            Err(err) => {
                eprintln!("{}", err);
                return Err(1);
            }
            Ok(input_len) => input_len,
        };
        if &user_input.trim().to_lowercase() == "quit" {
            return Err(end_program(&type_chart, filepath));
        }
        if !user_input.trim().is_empty() {
            return Ok(user_input);
        }
    }
}

fn get_effectiveness_from_user(display_string: &str, type_chart: &TypeChart, filepath: &String) -> Result<String, i32> {
    let stdin = io::stdin();
    let mut user_input = String::new();
    loop  {
        println!("{}", display_string);
        println!("{}", "1: Super Effective");
        println!("{}", "2: Neutral");
        println!("{}", "3: Not Very Effective");
        println!("{}", "4: Immune");
        let _ = match stdin.read_line(&mut user_input) {
            Err(err) => {
                eprintln!("{}", err);
                return Err(1);
            }
            Ok(input_len) => input_len,
        };
        match user_input.trim() {
            "1" | "SE" => return Ok("Super Effective".to_string()),
            "2" => return Ok("Neutral".to_string()),
            "3" | "NVE" => return Ok("Not Very Effective".to_string()),
            "4" => return Ok("Immune".to_string()),
            "Neutral" | "Super Effective" | "Not Very Effective" | "Immune" => return Ok(user_input),
            "Quit" | "quit" => return Err(end_program(type_chart, filepath)),
            _ => println!("That is not a valid effectiveness")
        }
        user_input.clear();
    }
}

fn display_type_list(type_list: &Vec<String>) {
    if !type_list.is_empty() {
        let mut current_length = 0;
        for index in 0..(type_list.len() - 1) {
            let type_name = type_list.get(index).expect("Should not be out of bounds");
            if current_length + type_name.len() > 101 {
                println!();
                current_length = 0;
            }
            print!("{}, ", type_name);
            // Need to take into account the comma and space
            current_length += type_name.len() + 2;
        }
        let last_type = type_list.get(type_list.len() - 1).expect("Should not be out of bounds, but like again");
        if current_length + last_type.len() > 101 {
            println!();
        }
        print!("{}", last_type);
    }
    println!();
}

fn end_program(type_chart: &TypeChart, filepath: &String) -> i32 {
    let stdin = io::stdin();
    let mut user_input = String::new();

    loop {
        println!("Do you want to save your changes?(y/n)");
        let _ = match stdin.read_line(&mut user_input) {
            Err(err) => {
                eprintln!("{}", err);
                return 1;
            }
            Ok(input_len) => input_len,
        };
        match user_input.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                let _ = files::save_types_to_file(&type_chart, filepath);
                return 0;
            },
            "n" | "no" => return 0,
            _ => {
                println!("Invalid answer");
                user_input.clear();
            },
        };
    }
}

fn print_type_effectiveness_map(type_effectiveness_map: &HashMap<String, Vec<String>>, type_name: &String, attacking: bool) {
    println!("{}", "=".repeat(101));
    let attacking = match attacking {
        true => "attacking",
        false => "defending"
    };
    println!("{} when {} :", type_name.trim(), attacking);
    for effectiveness in vec!["Immune", "Triple Not Very Effective", "Double Not Very Effective", "Not Very Effective", "Super Effective", "Double Super Effective", "Triple Super Effective"] {
        let type_list = match type_effectiveness_map.get(effectiveness) {
            None => {
                // Ignore it
                continue;
            },
            Some(type_list) => type_list,
        };
        if type_list.is_empty() {
            continue;
        }
        let dash_length = 50 - (effectiveness.len() / 2);
        let dashes = "-".repeat(dash_length);
        print!("{}{}{}", &dashes, effectiveness, &dashes);
        if effectiveness.len() % 2 == 0 {
            print!("-");
        }
        println!();
        display_type_list(type_list);
    }
    println!("{}", "=".repeat(101));
}

fn process_user_input(type_chart: &mut TypeChart, trimed_user_input: &str, filepath: &String) -> Result<bool, i32> {
    match trimed_user_input {
        "1" => {
            let type_name = get_info_from_user("Which type would you like to add?", type_chart, filepath)?;
            type_chart.add_new_type(&type_name.trim().to_string());
            println!();
        }
        "2" => {
            let type_name = get_info_from_user("Which type would you like to remove?", type_chart, filepath)?;
            type_chart.remove_existing_type(&type_name.trim().to_string());
            println!();
        }
        "3" => {
            let type_name = get_info_from_user("Which type is the attacking type?", type_chart, filepath)?;
            let opposing_type_name = get_info_from_user("Which type is the opposing type?", type_chart, filepath)?;
            let effectiveness = get_effectiveness_from_user("What is the effectiveness of the attacking type", type_chart, filepath)?;
            type_chart.add_effectiveness(&type_name.trim().to_string(), &opposing_type_name.trim().to_string(), effectiveness.trim().to_string());
            println!();
        }
        "4" => {
            let type_name = get_info_from_user("For what type would you like to see it's type chart?", type_chart, filepath)?;
            let attacking_type_effectiveness_map = match type_chart.get_attacking_effectiveness(&type_name.trim().to_string()) {
                Err(_) => return Ok(false),
                Ok(attacking_type_effectiveness_map) => attacking_type_effectiveness_map,
            };
            let defensing_type_effectiveness_map = match type_chart.get_defensive_effectiveness(&type_name.trim().to_string()) {
                Err(_) => return Ok(false),
                Ok(defensing_type_effectiveness_map) => defensing_type_effectiveness_map,
            };
            print_type_effectiveness_map(&attacking_type_effectiveness_map, &type_name, true);
            print_type_effectiveness_map(&defensing_type_effectiveness_map, &type_name, false);
        }
        "5" => {
            let mut first_type_name = get_info_from_user("What is the first type?", type_chart, filepath)?.trim().to_string();
            let second_type_name = match get_info_from_user("What is the second type? (write none for only 1 type)", type_chart, filepath)?.trim() {
                "none" => None,
                second_type_name => Some(&second_type_name.trim().to_string()),
            };
            let third_type_name = if second_type_name.is_some() {
                let third_type_name = get_info_from_user("What is the third type? (write none for only 2 type)", type_chart, filepath)?;
                match third_type_name.trim() {
                    "none" => None,
                    third_type_name => Some(&third_type_name.trim().to_string()),
                }
            } else {
                None
            };
            let type_effectiveness_map = match type_chart.get_multiple_defensive_effectiveness(&first_type_name, second_type_name, third_type_name) {
                Err(_) => return Ok(false),
                Ok(type_effectiveness_map) => type_effectiveness_map,
            };

            // Combining all type names together
            if let Some(second_type_name) = second_type_name {
                first_type_name.push_str(", ");
                first_type_name.push_str(second_type_name.as_str());
                if let Some(third_type_name) = third_type_name {
                    first_type_name.push_str(", ");
                    first_type_name.push_str(third_type_name.as_str());
                }
            }

            print_type_effectiveness_map(&type_effectiveness_map, &first_type_name, false);
        },
        "6" | "quit" | "Quit" => {
            return Ok(true);
        },
        _ => {
            println!("Incorrect Option");
        }
    }
    return Ok(false);
}

fn main() -> Result<(), i32> {
    let args = Args::parse();

    let stdin = io::stdin();
    let mut quit: bool = false;
    let mut user_input = String::new();
    let mut type_chart = files::get_types_from_file(&args.filepath).map_err(|()| 1)?;

    println!("Welcome to the TMT2 Type Track!");
    while !quit {
        // Show user the available options
        // TODO: add an option to stop the command at any point and return to this
        println!("What would you like to do?");
        println!("1: Add a new type");
        println!("2: Remove an existing type");
        println!("3: Add a new weakness/resistance");
        println!("4: See stats about a type");
        println!("5: See stats about multiple types");
        println!("6: Quit");
        println!("(At any point you can write \"quit\" to quit out of the program)");

        // Take user input
        user_input.clear();
        let _ = match stdin.read_line(&mut user_input) {
            Err(err) => {
                eprintln!("{}", err);
                return Err(1);
            }
            Ok(input_len) => input_len,
        };

        let trimed_user_input = user_input.trim();
        quit = match process_user_input(&mut type_chart, trimed_user_input, &args.filepath) {
            Err(0) => return Ok(()),
            Err(err) => return Err(err),
            Ok(quit) => quit,
        };
    }
    end_program(&type_chart, &args.filepath);
    return Ok(());
}
