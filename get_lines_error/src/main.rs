use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // path file
    let path_file = "src/dado.txt";

    // open file to read
    let file = match File::open(path_file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error to open the file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error to read the line: {}", e);
                continue;
            }
        };

        if line.contains(' ') && line.contains('.') && line.contains(' ') {
            println!("{}", line);
        }

        if line.starts_with("└") && line.contains('-') {
            println!("{}", line);
        }
    }
}
