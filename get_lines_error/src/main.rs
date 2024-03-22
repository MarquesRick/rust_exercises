use regex::Regex;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"└\s\d+\-")?;
    let re_2 = Regex::new(r"└\s\d+\s-")?;

    let dir = env::current_dir().expect("Error getting current dir..");
    println!("Diretório atual: {:?}", dir);
    let filename = "data.txt";
    let full_path = Path::new(&dir).join("src").join(filename);
    println!("Diretório full path: {:?}", full_path);
    // Abrir o arquivo de entrada
    let input_file = File::open(full_path)?;
    let input_reader = BufReader::new(input_file);

    // Abrir ou criar o arquivo de saída em inglês
    let dir_output = env::current_dir().expect("Error getting current dir..");
    let full_path_output = Path::new(&dir_output).join("src").join("output.txt");
    let mut output_file = File::create(full_path_output)?;

    // Processar cada linha do arquivo de entrada
    for line in input_reader.lines() {
        let line = line?;
        // Verificar se a linha contém um espaço, número, ponto final e espaço
        // if re.is_match(&line) || re.is_match(&line) {
        //     writeln!(output_file, "{}", line)?;
        // }

        match line {
            x if re.is_match(&x) => {
                writeln!(output_file, "{}", x)?;
                continue;
            }
            x if re_2.is_match(&x) => {
                writeln!(output_file, "{}", x)?;
                continue;
            }
            _ => {
                print!("");
                continue;
            }
        }
    }

    println!("Linhas filtradas copiadas para output.txt com sucesso!");

    Ok(())
}
