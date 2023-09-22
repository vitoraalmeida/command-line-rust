use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// alias para um result que o Ok é unity e o erro um valor que implementa a
// trait Error. Unity pois retornamos para o ferramental de testes e ele não
// vai usar o valor de fato, somente verificar se ocorreu um erro que indica
// que o teste falhou
// dyn indica que o tipo concreto que implementa a Trait é definido em tempo 
// de execução, então será criado na heap e para que possamos acessar precisamos
// de um ponteiro "Box"
type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}


#[test]
fn dies_no_args() -> TestResult {
    //let mut cmd = Command::cargo_bin("echor").unwrap();
    // ? é um syntax sugar para verificar o valor do Result e caso seja um Ok
    // busca o valor interno, se não, propaga o erro para quem chamou
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        // stderr vai chamar internamente a função do predicado
        // que compara o predicado registrado com o valor que foi
        // inserido no stderr pelo comando
        .stderr(predicate::str::contains("USAGE"));
        // predicate implementa predicados lógicos, ou seja,
        // asserções que podem ser verdadeiras ou falsas com base no 
        // valor das variáveis passadas
    Ok(())
}


#[test]
fn hello1() -> TestResult {
    // fs::read_to_string para arquivos pequenos, pois lê o conteúdo inteiro
    let expected = fs::read_to_string("tests/expected/hello1.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
