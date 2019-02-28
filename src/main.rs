use std::process;
use std::io::{BufRead, BufWriter, Write, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect(); 
    if args.len() < 2 {
        eprintln!("Not enough arguments.");
        process::exit(1);
    }
    let lvl: u32 = args[1].parse().unwrap();

    if let Err(e) = filter_coverage(lvl) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn filter_coverage(lvl: u32) -> std::io::Result<()> {
    let mut out_file = BufWriter::new(std::io::stdout());
    for line in BufReader::new(std::io::stdin()).lines() {
        match line {
            Ok(line) => {
                if check_cov(&line, lvl).unwrap() {
                    out_file.write(format!("{}\n", line).as_bytes())?;
                }
            } 
            Err(e) => {
                return Err(e);
            }
        }  
    }
    Ok(())
}

fn check_cov(line: &String, cov: u32) -> Result<bool, String> {
    let pieces:Vec<&str> = line.split_whitespace().collect();
    let file_cov: u32 = pieces[4].parse().unwrap();
    return Ok(file_cov >= cov);
}
