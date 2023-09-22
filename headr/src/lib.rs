use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Vitor Almeida")
        .about("Versão do comando head escrito em rust")
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"), // por padrão mostra as 10 primeiras linhas
                // não precisa usar takes_value, pois já define um valor padrão
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)       // flag que aceita valores
                .conflicts_with("lines") // não aceita mostrar linhas e bytes
                .help("Number of bytes"), // ao mesmo tempo
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        // passa o valor que vem de value_of para a função de parsing
        .map(parse_positive_int) 
        // o map retorna Option<Result>, transpose transforma em Result<Option>
        .transpose()             
        // passa o retorno para um map que captura erros
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        // busca o valor passado substituindo code points utf8 invalidos por um
        // code point valido que não tem significado especial, um caractere
        // substituto
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes, // o valor de bytes é mantido como Option
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                // se existir mais de um arquivo, adiciona um cabeçalho
                // indicando onde começa a saída do comando para aquele
                // arquivo
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        // se for o primeiro arquivo, não pula linha
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }

                if let Some(num_bytes) = config.bytes {
                    /*
                    // le o arquivo numa string, converte em bytes e converte
                    // os bytes desejados em string de volta
                    // não é indicado pois se o arquivo for muito grande será
                    // lido por inteiro e pode faltar memória, e se estiver 
                    // vazio a leitura do buffer incorre em erro, pois não há
                    // elementos
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?; 
                    let bytes = contents.as_bytes();
                    print!("{}", String::from_utf8_lossy(&bytes[..num_bytes]));
                    */
                    // BufRead::take() retorna um handle que lerá no máximo
                    // o número de bytes informado
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    // a quantidade que ele vai tentar ler é o tamanho
                    // do buffer, ainda que o limite seja maior (passado no take)
                    handle.read(&mut buffer)?;
                    //alternativa usando turbofish
                    //let bytes = file.bytes().take(num_bytes).collect::<Result<Vec<_>, _>>();
                    print!(
                        "{}",
                        // converte os bytes em string de forma que se não for
                        // utf8 valido substitui por um símbolo que denota
                        // um caractere desconhecido
                        String::from_utf8_lossy(&buffer)
                    );
                } else {
                    // line funciona como um buffer que será preenchido
                    // a cada iteração
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        // BufRead::read_line lê os bytes até encontrar um 
                        // delimitador de linha ou EOF, mantendo o delimitador
                        // retornando o número de bytes que fora lidos no buffer
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }

    /*
    //usa BufRead.lines() que retorna um iterador sobre as linhas do arquivo
    // o iterador possui o método take, que retorna a quantidade de itens do
    // iterador informada
    // lines cria o iterador, mas remove os caracteres de new line ou CRLF
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                for line in file.lines().take(config.lines) {
                    println!("{}", line?);
                }
            }
        }
    }
    */

    Ok(())
}


// --------------------------------------------------
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    // faz o parsing do &str em um usize (inferido pelo retorno da função)
    match val.parse() {
        // if n > 0 é um guard, uma checagem adicional ao pattern matching
        Ok(n) if n > 0 => Ok(n),
        /*
        Ok(n) => {
            if n > 0 {
                Ok(n)
            } else {
                Err(From::from(val))
            }
        }
        */
        // std::convert::From é uma trait que define conversão entre tipos
        // aqui converte val (&str) em um Erro
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 é um inteiro válido
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);
    // Uma string não é possível de ser parseada em uint 
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());
    // Zero não é um inteiro positivo
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
