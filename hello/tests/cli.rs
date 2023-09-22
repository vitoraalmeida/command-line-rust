// use std::process::Command;
use assert_cmd::Command;

#[test]
fn runs() {
    // Executar comandos pelo nome sem que estejam no PATH
    // não funciona, ainda que estejamos no diretório que contém
    // o comando. Isso pelo fato de que incluir o diretório atual no PATH
    // possibilitaria que alguns comandos pudessem ser executados sem a
    // ciência do usuário, tendo sua chamada oculta em algum local de um
    // programa que usuário deseja executar.
    // A crate "assert-cmd" procura o programa no diretório da crate atual
    // para que possa ser executado
    //
    //let mut cmd = Command::new("hello");
    //let res = cmd.output();
    //assert!(res.is_ok());
    
    // unwrap() causa panic caso o Result retornado não seja Ok.
    // panic nesse caso pode ser usado pois o teste não funciona se o binário
    // não for encontrado
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n"); 
    // testa se o comando foi executado com sucesso, ou seja se o exit status
    // enviado para o sistema operacional pelo programa indica sucesso.
    // O status de sucesso é 0 (zero erros) e de 1 a 255 são status de falha
    // E se o reusultado produzido no stdout foi o valor passado
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
