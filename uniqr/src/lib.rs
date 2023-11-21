use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String, // Não é option, pois há um valor default
    out_file: Option<String>,
    count: bool,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust uniq")
        .arg(
            Arg::with_name("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-"), // default é stdin
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("OUT_FILE")
                .help("Output file"),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .help("Show counts")
                .takes_value(false), // flag
        )
        .get_matches();

    Ok(Config {
        //in_file: matches.value_of_lossy("in_file").unwrap().to_string(),
        //in_file: matches.value_of_lossy("in_file").map(String::from).unwrap(),
        //in_file: matches.value_of_lossy("in_file").map(|v| v.into()).unwrap(),
        in_file: matches.value_of_lossy("in_file").map(Into::into).unwrap(),
        out_file: matches.value_of("out_file").map(|v| v.to_string()),
        count: matches.is_present("count"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;

    // ou abre o arquivo passado ou abre stdout. Os 2 implementam
    // a trait Write
    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    // função inline pois só é usada dentro de run
    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            // como é uma closure, já tem acesso ao config.count
            // que está no escopo de run
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        };
        Ok(())
    };

    // só imprime a linha quando tivermos passado por todas as
    // ocorrências iguais consecutivas
    // ex.:
    /*
     * a
     * a
     * b
     * c
     * 
     * a primeira iteração não vai imprimir, pois ainda não sabemos
     * se o "a" continuará ocorrendo.
     * na segunda iteração, ainda não imprimimos, pois a linha anterior
     * não é diferente a linha atual ("a" = "a").
     * na terceira iteração, previous é "a" e estamos em "b", que é !=
     * então imprimimos a contagem atual = 2 e o a. 
     * assim mostramos apenas um "a" e seu número de ocorrências (se
     * o usuário adicionou a flag de contagem)
     */

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            // a ultima linha do arquivo está em previous
            // e tentamos ler um arquivo que já acabou
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }
    // lida com a última linha do arquivo
    print(count, &previous)?;

    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
