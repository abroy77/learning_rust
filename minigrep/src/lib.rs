use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result_lines = search(&config.query, &contents);
    for line in result_lines {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            Err("not enough arguments")
        } else {
            Ok(Config {
                query: args[1].clone(),
                file_path: args[2].clone(),
            })
        }
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut success_lines: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            success_lines.push(line)
        }
    }
    success_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}
