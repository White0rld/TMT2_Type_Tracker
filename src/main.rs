use clap::Parser;
use std::io;

mod files;
mod type_chart;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("examples/types.csv"))]
    filepath: String,
}

fn get_info_from_user(display_string: &str) -> Result<String, i32> {
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
        if !user_input.trim().is_empty() {
            return Ok(user_input);
        }
    }
}

fn get_effectiveness_from_user(display_string: &str) -> Result<String, i32> {
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
            _ => println!("That is not a valid effectiveness")
        }
    }
}

fn main() -> Result<(), i32> {
    let args = Args::parse();
    println!("{}", args.filepath);

    let stdin = io::stdin();
    let mut quit: bool = false;
    let mut user_input = String::new();
    let mut type_chart = files::get_types_from_file(&args.filepath).map_err(|()| 1)?;
    while !quit {
        // Show user the available options
        println!("Welcome to the TMT2 Type Track!");
        println!("What would you like to do?");
        println!("1: Add a new type");
        println!("2: Remove an existing type");
        println!("3: Add a new weakness/resistance");
        println!("4: See stats about a type (not implemented)");
        println!("5: Quit");

        // Take user input
        user_input.clear();
        let _ = match stdin.read_line(&mut user_input) {
            Err(err) => {
                eprintln!("{}", err);
                return Err(1);
            }
            Ok(input_len) => input_len,
        };
        println!("You chose {}", user_input);
        let trimed_user_input = user_input.trim();
        match trimed_user_input {
            "1" => {
                let type_name = get_info_from_user("Which type would you like to add?")?;
                type_chart.add_new_type(&type_name.trim().to_string());
                println!();
            }
            "2" => {
                let type_name = get_info_from_user("Which type would you like to remove?")?;
                type_chart.remove_existing_type(&type_name.trim().to_string());
                println!();
            }
            "3" => {
                let type_name = get_info_from_user("Which type is the attacking type?")?;
                let opposing_type_name = get_info_from_user("Which type is the opposing type?")?;
                let effectiveness = get_effectiveness_from_user("What is the effectiveness of the attacking type")?;

                type_chart.add_effectiveness(&type_name.trim().to_string(), &opposing_type_name.trim().to_string(), effectiveness.trim().to_string());
                println!();
            }
            "4" => {
                dbg!(&type_chart);
            }
            "5" => {
                quit = true;
                println!("Goodbye!");
            },
            _ => {
                println!("Incorrect Option");
            }
        }
    }
    let _ = files::save_types_to_file(type_chart, &args.filepath);
    return Ok(());
}
