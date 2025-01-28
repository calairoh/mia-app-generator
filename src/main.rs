use std::env;
use std::fs;
use translate::{translate_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }
    let input_file = &args[1];
    let output_file = &args[2];

    // Read the input file
    let config_data = fs::read_to_string(input_file)?;
    
    // Transform the configuration
    let result = translate_config(config_data);
    let app_definition_json = result.unwrap();
    
    // Write the output to a file
    fs::write(output_file, app_definition_json)?;
    
    Ok(())
}