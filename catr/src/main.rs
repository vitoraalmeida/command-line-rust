fn main() {
    // and_then executa a função que foi passada caso o result seja Ok
    // de forma que run só será executado se os argumentos foram parseados
    // da forma esperada
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", e); // print da mensagem de erro no stderr
        std::process::exit(1);
    }
}
