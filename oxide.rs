use std::{env, fs};
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || !args[1].ends_with(".oxd") {
        return println!("Must provide a file ending in .oxd");
    }
    
    let raw_code = fs::read_to_string(&args[1]).expect("Failed to read file");
    let mut vars = HashMap::new();

    let code: String = raw_code
        .lines()
        .filter(|line| !line.trim().starts_with("//") && !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .join("\n");

    for line in code.lines() {
        let line = line.trim();

        if line.starts_with("let ") {
            if let Some(clean) = line.strip_prefix("let ") {
                let parts: Vec<&str> = clean.split('=').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    let var_name = parts[0].to_string();
                    let raw_val = parts[1];

                    if raw_val == "input?()" {
                        let _ = io::stdout().flush();
                        let mut user_input = String::new();
                        io::stdin().read_line(&mut user_input).expect("Failed to read input");

                        let clean_input = format!("\"{}\"", user_input.trim());
                        vars.insert(var_name, clean_input);
                    } else {
                        vars.insert(var_name, raw_val.to_string());
                    }
                }
            }
        } 
        else if line.starts_with("sleep?(") && line.ends_with(')') {
            let ms_str = line[7..line.len() - 1].trim();
            let resolved_ms = vars.get(ms_str).map(|s| s.as_str()).unwrap_or(ms_str);
            let ms = resolved_ms.parse::<u64>().unwrap_or(0);

            std::thread::sleep(std::time::Duration::from_millis(ms));
            }
        else if line.starts_with("println?(") && line.ends_with(')') {
            let expr = line[9..line.len() - 1].trim();

            let val = vars.get(expr).map(|s| s.as_str()).unwrap_or(expr);

            if val.starts_with('"') && val.ends_with('"') {

                print!("{}", &val[1..val.len() - 1].replace("\\n", "\n"));
                let _ = io::stdout().flush();
            } else {

                let total: i32 = val
                    .split('+')
                    .map(|num| {
                        let num = num.trim();
                        let resolved = vars.get(num).map(|s| s.as_str()).unwrap_or(num);
                        resolved.parse::<i32>().unwrap_or(0)
                    })
                    .sum();
                    print!("{}", total);
                    let _ = io::stdout().flush();
            }
        }
    }
}
