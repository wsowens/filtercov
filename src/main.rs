use std::process;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect(); 
    let conf = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = filter_coverage(&conf) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    levels: Vec<u32>,
    filenames: Vec<String>,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err("Not enough arguments".to_string());
        }
        let mut levels = Vec::<u32>::new();
        for lvl in args[1].split(" ") {
            if lvl != "" {
                let parsed = lvl.parse::<u32>();
                let parsed = match parsed {
                    Ok(value) => value,
                    Err(_x) => { 
                        return Err(format!("Could not parse coverage level \'{}\'", lvl));
                    },
                };
                    
                if !(levels.iter().any(|x| x == &parsed)) {
                    levels.push(parsed);
                }
            }
        }
        levels.sort();
        let mut filenames = Vec::<String>::new();
        if args.len() == 2 {
            return Ok(Config {levels, filenames});
        }
        //do this better with lifetimes later
        //test the files exist
        for name in &args[2..] {
            if Path::new(&name).exists() {
                filenames.push(name.to_string());
            } else {
                return Err(format!("Could not find file with name \'{}\'", name));
            }
        }
        Ok(Config {levels, filenames})
    }
}

fn modify_filename(filename: &String, coverage: &u32) -> String {
    let mut pieces: Vec<String> = filename.split(".").map(|string| string.to_string()).collect();
    pieces.insert(pieces.len()-1, coverage.to_string());
    pieces.join(".")
}

fn filter_coverage(config: &Config) -> std::io::Result<()> {
    let mut previous = config.filenames.clone();
    for coverage in config.levels.iter() {
        //println!("Previous: {:?}", previous);
        let current: Vec<String> = config.filenames.iter().map(|string| modify_filename(string, coverage)).collect();
        //println!("Current:  {:?}", current);
        for (new_file, old_file) in current.iter().zip(previous) {
            println!("{} -> {}", old_file, new_file); 

            let out_file = File::create(new_file).unwrap();
            let mut out_file = BufWriter::new(out_file);

            for line in BufReader::new(File::open(old_file).unwrap()).lines() {
                let line = match line {
                    Ok(line) => line,
                    Err(e) => {
                        return Err(e);
                    },
                };
                if check_cov(&line, *coverage).unwrap() {
                    out_file.write(format!("{}\n", line).as_bytes())?;
                }
            }
        }
        previous = current;
    }
    Ok(())
}

fn check_cov(line: &String, cov: u32) -> Result<bool, String> {
    let pieces:Vec<&str> = line.split_whitespace().collect();
    let file_cov: u32 = pieces[4].parse().unwrap();
    return Ok(file_cov >= cov);
}
