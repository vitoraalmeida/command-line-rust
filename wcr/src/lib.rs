use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

// Struct que conterá os argumentos passados para o programa
#[derive(Debug)] // derive implementa a trait automaticamente
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Vitor Almeida")
        .about("Implementação do programa wc em Rust")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-") // STDIN
                .multiple(true),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("Show word count")
                .takes_value(false), // flags que não recebem valores
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Show byte count")
                .takes_value(false), // flags que não recebem valores
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("Show character count")
                .takes_value(false) // flags que não recebem valores
                // não permite tentar executar a flag bytes junto com chars
                .conflicts_with("bytes"), 
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("Show line count")
                .takes_value(false), // flags que não recebem valores
        )
        .get_matches();


    // busca todas as flags
    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");
    
    // se todas as flags forem falsas, define por padrão que serão
    // mostradas linhas, palavras e bytes
    if [words, bytes, chars, lines].iter().all(|v| v == &false) {
        // transforma um slice em iterador e o iter retorna uma
        // referencia para os valores, então por isso checamos
        // contra uma referencia para false
        // Se todos os valores foram avaliados como verdadeiros
        // &false == &false -> true), Iterator::all retorna true
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
             Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, config.lines),
                        format_field(info.num_words, config.words),
                        format_field(info.num_bytes, config.bytes),
                        format_field(info.num_chars, config.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", &filename)
                        },
                    );

                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            }
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars)
        );
    }

    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// --------------------------------------------------
fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

// --------------------------------------------------
pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new(); // buffer

    loop {
         // read_line retorna a quantidade de bytes lidos
         let line_bytes = file.read_line(&mut line)?;
         if line_bytes == 0 {
             // arquivo vazio
             break;
         }
         num_bytes += line_bytes;
         num_lines += 1;
         // split_whitespace e chars retornam iterators e
         // Iterator::count retorna o total de itens
         num_words += line.split_whitespace().count();
         num_chars += line.chars().count();
         // limpa o buffer para leitura posterior
         line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    // Cursor é usado para buffers em memória para implementar as traits
    // Read e Write em qualquer valor que implemente AsRef<[u8]>, para
    // para os buffers possam ser usados em qualquer lugar que se use um
    // reader ou writer que lide com I/O. Nesse caso, simulamos um
    // arquivo
    use std::io::Cursor;
    use crate::format_field;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want yout half.\r\n";
        // criamos um arquivo fake a partir do texto acima
        let info = count(Cursor::new(text));
        assert!(info.is_ok()); // checa se o Result é Ok
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
