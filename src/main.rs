use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = String::from("example_files/test_file.cpp");
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let ip: &str = ip.trim();
                if ip.is_empty() {
                    continue;
                }
                println!("{}", ip);
            }
        }
    } else {
        println!("Else part executed")
    }
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}
