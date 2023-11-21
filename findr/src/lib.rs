use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use walkdir::{WalkDir, DirEntry};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("Vitor Almeida")
        .about("Versão do programar find em rust")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .help("Seach paths")
                .default_value(".")
                .multiple(true),
        )
        .arg(
            Arg::with_name("names")
                .value_name("NAME")
                .short("n")
                .long("name")
                .help("Name")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("types")
                .value_name("TYPE")
                .short("t")
                .long("type")
                .help("Entry type") 
                // define os valorse possíveis para essa opção
                .possible_values(&["f", "d", "l"])
                .takes_value(true)
                .multiple(true)
            
        )
        .get_matches();

    // a partir dos valores passados, criamos um vetor de valores Regex
    // caso o termo passado pelo usuário seja um regex válido
    let names = matches
        .values_of_lossy("names") // retorna Option<Vec<String>>
        .map(|vals| { // Option::map transforma um Option<T> em Option<U>
            // U nesse caso é um Result<Vec<regex::Regex>>, pois é o
            // retorno do bloco abaixo
            vals.into_iter() // transforma o vec em iterador
                .map(|name| {
                    Regex::new(&name) // cria um Regex a partir da string
                        .map_err(|_| format!("Invalid --name \"{}\"", name))
                        // se houver erro na compilação do regex, erro
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()? // Transforma Option<Result> em Result<Option>
        .unwrap_or_default(); // Ou consegue desempacotar em um Vec<Regex
                              // com valores ou um valor Vec<regex vazio

    // clap should disallow anything but "d," "f," or "l"
    let entry_types = matches
        .values_of_lossy("types")
        .map(|vals| {
            vals.iter()
                .map(|val| match val.as_str() {
                    "d" => Dir,
                    "f" => File,
                    "l" => Link,
                    _ => unreachable!("Invalid type"), // panic se não
                                                       // for valido
                })
                .collect() // o tipo de retorno é inferido pelo tip
                           // dos valores retornados no match
        })
        .unwrap_or_default();

    Ok(Config {
        // seguro chamar unwrap pois possui um default
        paths: matches.values_of_lossy("paths").unwrap(),
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    // usando closures para ter acesso ao config
    // checa se o tipo de entrada buscada condiz com os disponíveis
    // na aplicação. Se sim, checa se a entrada (arquivo, dir ou link)
    // é de fato aquele tipo que foi filtrado pelo usuário
    let type_filter = |entry: &DirEntry| {
        config.entry_types.is_empty() // ja retorna true se não tem tipo
            || config                 // especificado
                .entry_types
                .iter()
                .any(|entry_type| {
                    match entry_type {
                        Link => entry.file_type().is_symlink(),
                        Dir => entry.file_type().is_dir(),
                        File => entry.file_type().is_file(),
                    }
                })
    };
    
    // apos passar pelo filtro de tipos, passa pelo filtro de nome
    // (regex)
    let name_filter = |entry: &DirEntry| {
        config.names.is_empty()
            || config
                .names
                .iter()
                .any(|re| {
                    re.is_match(&entry.file_name().to_string_lossy())
                })
    };

    for path in config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                // erros relacionado a interação do WalkDir com arquivos
                // (ex.: permission denied) são mostrados, mas geramos
                // um novo iterator apenas com os resultados sem erros
                // para usar outros metodos de iteradores (filter, map)
                // apenas nos resultados relevantes
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            //
            .filter(type_filter)
            .filter(name_filter)
            // gera novo iterador com os nomes dos caminhos
            .map(|entry| entry.path().display().to_string())
            // coleta os caminhos num vector de strings
            .collect::<Vec<_>>();
        println!("{}", entries.join("\n"));
    }
    Ok(())

    /*
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    if (config.entry_types.is_empty() 
                        || config.entry_types.iter().any(|entry_type| {
                            match entry_type {
                                Link => entry.file_type().is_symlink(),
                                Dir => entry.file_type().is_dir(),
                                File => entry.file_type().is_file(),
                            }
                        }))
                        && (config.names.is_empty()
                            || config.names.iter().any(|re| {
                                re.is_match(
                                    &entry.file_name().to_string_lossy(),
                                )
                            }))
                    {
                        println!("{}", entry.path().display());
                    }
                } 
            }
        }
    }

    Ok(())
    */
}
