use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Representa um result que pode conter o struc de configuração ou um erro
type MyResult<T> = Result<T, Box<dyn Error>>;

// Config representa os argumentos que serão passados para o comando
// tanto posicionais (files) quando opcionais (flags)
// macro derive(Debug) implementa a trait debug automaticamente quando os campos
// são de tipos primitivos
#[derive(Debug)] 
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Vitor Almeida")
        .about("Versão simplificada do comando cat escrito em Rust")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"), // representa que o valor virá do stdin
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false) // é uma flag, não tem valor relacionado
                .conflicts_with("number_nonblank"), // ou podemos usar n ou b
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    // Cria e retorna o config com os valores que foram parseados pelo Clap
    Ok(Config {
        // como files possui um valor padrão, não corre o risco de panic
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let mut line_num = 0;
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                // operações com filehandles retornam Result, pois se tratando
                // de IO, podem falhar
                for line in file.lines() {
                    let line = line?;
                    if config.number_lines {
                        line_num += 1;
                        println!("{:>6}\t{}", line_num, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            line_num += 1;
                            println!("{:>6}\t{}", line_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

// --------------------------------------------------
// File::open retorna um filehandle, que é um mecanismo para ler conteúdo
// de arquivos. O filehandle implementa a trai BufRead, que possui a 
// função BufRead::lines que permite ler os arquivos por linhas de 
// forma que não corremos o risco de lotar a memório com o conteúdo do
// arquivo. io::stdin também implementa BufRead
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        // ? propaga o erro caso não consiga abrir
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
