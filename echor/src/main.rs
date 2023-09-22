use clap::{App, Arg}; // importa struct App

// a falta de um " -> Tipo " depois de main() indica que main retorna o tipo
// unit, representado com (), que indica a falta de algum valor significativo
// para retornar
fn main() {
    // estrutura básica de um programa cli usando clap
    // entrega um menu de uso com flags -h e -v para help e version
    // com descrição de informações sobre o programa
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Vitor Almeida")
        .about("Versão simplificada do programa echo (unix) escrita em go")
        // Opções de linhas de comando normalmente são indicado com --opcao ou -o
        // de forma que um hífen indica a forma curta da opção e dois hífens o
        // nome completo. Quando uma opção ativa ou desativa um comportamento do
        // comando/programa, chamamos de "flag"
        // Outros argumentos de linhas de comando são chamados de posicionais, pois
        // a posição deles relativamente ao nome do programa indica o significado
        // No comando chmod mode file, o primeiro argumento a ser passado deve ser
        // a permissão do arquivo e o segundo o arquivo que está sendo modificado
        .arg(
            Arg::with_name("text") // nome do argumento
                .value_name("TEXT") // valor que será mostrado no help
                .help("Input text") // descrição do argumento
                .required(true) // gera erro se não for passado pelo usuário
                .min_values(1), // erros são enviados para o STDERR
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    // {:?} indica para usarmos uma representação para debbuging do valor passado
    // adicionando # temos um versão mais simples de ler
    //println!("{:#?}", matches);

    // values_of_lossy retorna um Option<Vec<String>>
    // alternativa = values_of -> Option<Values> | values = interator dos args
    // como text é obrigatório, o programa não chega nesse ponto sem valores
    // então unwrap() não corre o risco de encontrar None e resultar em panic
    let text = matches.values_of_lossy("text").unwrap(); 
    // retorna um bool que indica a presença da flag nos argumentos
    let omit_newline = matches.is_present("omit_newline");

    // assume o padrão de adicionar uma quebra de linha
    //let mut ending = "\n";
    //if omit_newline {
    //    ending = "";
    //}
    
    let ending = if omit_newline { "" } else { "\n" };

    // Vec::join recebe uma string que será posicionada entre os elementos do vec
    print!("{}{}", text.join(" "), ending); 
}
