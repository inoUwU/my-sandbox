// HACK:  https://youtu.be/PX4dEky1pxA?si=LYMKqiDjrVtkYVgf

use std::str::pattern::Pattern;

struct Config {
    pattern: String,
    files: Vec<String>,
    case_insensitive: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!("Usage:{} pattern file (or files [-i])", args[0]));
        }

        let mut case_insensitive: bool = false;
        let mut non_flag_args: Vec<String> = Vec::new();

        for arg in &args[1..] {
            if arg == "-i" {
                case_insensitive = true;
            } else {
                non_flag_args.push(arg.clone());
            }
        }

        if non_flag_args.is_empty() {
            return Err("Error; No input files provided.".to_string());
        }

        let Pattern = non

         

        Ok(())
    }
}

fn main() {}
