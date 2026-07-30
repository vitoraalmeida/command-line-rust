# Command Line Rust

Este repositório reúne exercícios de estudo da linguagem Rust por meio da
reimplementação simplificada de utilitários comuns de linha de comando. A
proposta é aprender a linguagem em um contexto prático: receber argumentos,
ler arquivos e a entrada padrão, produzir saídas compatíveis com ferramentas
Unix, tratar erros e testar binários de ponta a ponta.

Os programas têm nomes terminados em `r` para diferenciá-los dos comandos
originais (`catr`, `headr`, `wcr` etc.). Eles são projetos educacionais, não
substitutos completos das implementações presentes no sistema operacional.

## Projetos

| Projeto | Comando estudado | Funcionalidades implementadas |
| --- | --- | --- |
| [`hello`](hello/) | `hello`, `true` e `false` | Primeiro programa, múltiplos binários e códigos de saída de processos |
| [`echor`](echor/) | `echo` | Imprime um ou mais argumentos; `-n` omite a quebra de linha final |
| [`catr`](catr/) | `cat` | Lê um ou mais arquivos ou `stdin`; `-n` numera todas as linhas e `-b` apenas as não vazias |
| [`headr`](headr/) | `head` | Mostra as primeiras 10 linhas por padrão; `-n` escolhe linhas e `-c` escolhe bytes |
| [`wcr`](wcr/) | `wc` | Conta linhas (`-l`), palavras (`-w`), bytes (`-c`) e caracteres (`-m`), incluindo totais para vários arquivos |
| [`uniqr`](uniqr/) | `uniq` | Remove linhas repetidas consecutivas, aceita arquivo de saída e exibe contagens com `-c` |
| [`cutr`](cutr/) | `cut` | Seleciona campos (`-f`), bytes (`-b`) ou caracteres (`-c`) por posições e intervalos |
| [`commr`](commr/) | `comm` | Compara dois arquivos ordenados, permite ocultar as colunas `-1`, `-2` e `-3`, comparar sem diferenciar maiúsculas com `-i` e mudar o delimitador |
| [`findr`](findr/) | `find` | Percorre diretórios recursivamente e filtra nomes por regex (`-n`) e tipos (`-t f`, `-t d` ou `-t l`) |

Há também o arquivo [`hello.rs`](hello.rs), a versão mínima e independente do
primeiro “Hello, world!”, que pode ser compilada diretamente com `rustc`.

## Pré-requisitos

É necessário ter uma instalação recente de Rust com `cargo` e `rustc`
disponíveis no `PATH`. A forma mais comum de instalar a toolchain é pelo
`rustup`.

Este repositório não possui um `Cargo.toml` na raiz: cada diretório é uma crate
independente, com seu próprio manifesto e arquivo `Cargo.lock`.

## Como executar

Entre no diretório de um projeto e passe os argumentos do programa depois de
`--`:

```console
$ cd echor
$ cargo run -- Olá Rust
Olá Rust

$ cargo run -- -n "sem quebra de linha"
sem quebra de linha
```

Também é possível executar qualquer crate a partir da raiz:

```console
$ cargo run --manifest-path catr/Cargo.toml -- -n catr/tests/inputs/fox.txt
$ cargo run --manifest-path wcr/Cargo.toml -- -l -w wcr/tests/inputs/fox.txt
$ cargo run --manifest-path findr/Cargo.toml -- findr/tests/inputs -t f -n '.*[.]txt'
```

O valor `-` representa a entrada padrão nos programas que recebem arquivos.
Isso permite compô-los em pipelines:

```console
$ printf 'a\na\nb\n' | cargo run --manifest-path uniqr/Cargo.toml -- -c
   2 a
   1 b
```

Use `-h` ou `--help` para consultar todas as opções de uma ferramenta:

```console
$ cargo run --manifest-path cutr/Cargo.toml -- --help
```

Para compilar uma versão otimizada:

```console
$ cargo build --release --manifest-path findr/Cargo.toml
$ ./findr/target/release/findr . -t f
```

## Exemplos por ferramenta

### `catr`

```console
# Numera todas as linhas.
$ cargo run --manifest-path catr/Cargo.toml -- -n arquivo.txt

# Numera somente linhas não vazias.
$ cargo run --manifest-path catr/Cargo.toml -- -b arquivo.txt

# Concatena vários arquivos.
$ cargo run --manifest-path catr/Cargo.toml -- arquivo1.txt arquivo2.txt
```

As opções `-n` e `-b` são mutuamente exclusivas.

### `headr`

```console
# Primeiras 5 linhas.
$ cargo run --manifest-path headr/Cargo.toml -- -n 5 arquivo.txt

# Primeiros 20 bytes.
$ cargo run --manifest-path headr/Cargo.toml -- -c 20 arquivo.txt
```

As contagens precisam ser inteiros positivos, e linhas e bytes não podem ser
selecionados ao mesmo tempo.

### `wcr`

```console
# Comportamento padrão: linhas, palavras e bytes.
$ cargo run --manifest-path wcr/Cargo.toml -- arquivo.txt

# Apenas caracteres e palavras.
$ cargo run --manifest-path wcr/Cargo.toml -- -m -w arquivo.txt
```

Bytes (`-c`) e caracteres (`-m`) são medidas diferentes e, nesta
implementação, opções mutuamente exclusivas.

### `uniqr`

```console
# Remove repetições adjacentes e mostra quantas vezes cada linha ocorreu.
$ cargo run --manifest-path uniqr/Cargo.toml -- -c entrada.txt

# Escreve o resultado em outro arquivo.
$ cargo run --manifest-path uniqr/Cargo.toml -- entrada.txt saida.txt
```

Assim como o `uniq` tradicional, o programa compara linhas vizinhas. Para
agrupar todas as repetições de um arquivo desordenado, ordene a entrada antes.

### `cutr`

```console
# Seleciona os campos 1 e 3 de um CSV.
$ cargo run --manifest-path cutr/Cargo.toml -- -d ',' -f 1,3 dados.csv

# Seleciona do segundo ao quinto caractere de cada linha.
$ cargo run --manifest-path cutr/Cargo.toml -- -c 2-5 arquivo.txt
```

As posições começam em 1. Listas como `1,3,5-7` são aceitas, mas é obrigatório
escolher exatamente um modo entre campos, bytes e caracteres. O delimitador de
campos deve ocupar um único byte.

### `commr`

```console
# Exibe linhas exclusivas do primeiro arquivo, exclusivas do segundo e comuns.
$ cargo run --manifest-path commr/Cargo.toml -- arquivo1.txt arquivo2.txt

# Oculta as duas primeiras colunas e compara sem diferenciar maiúsculas.
$ cargo run --manifest-path commr/Cargo.toml -- -12 -i arquivo1.txt arquivo2.txt
```

Os dois arquivos devem estar ordenados conforme o mesmo critério usado na
comparação. Apenas um deles pode ser lido de `stdin`.

### `findr`

```console
# Busca arquivos CSV e MP3 em dois caminhos.
$ cargo run --manifest-path findr/Cargo.toml -- pasta1 pasta2 \
    -t f -n '.*[.]csv' -n '.*[.]mp3'
```

Os valores de `--name` são expressões regulares. Portanto, para procurar
arquivos CSV use `.*[.]csv`, e não o glob de shell `*.csv`. Quando vários
nomes ou tipos são informados, basta que uma das alternativas corresponda.

## Testes

Cada crate possui sua própria suíte. Os testes usam principalmente:

- `assert_cmd` para executar o binário compilado;
- `predicates` para validar `stdout`, `stderr` e mensagens de erro;
- arquivos em `tests/inputs/` como fixtures;
- arquivos em `tests/expected/` como saídas de referência;
- testes unitários para regras internas, além dos testes de integração da CLI.

Para testar um projeto:

```console
$ cargo test --manifest-path cutr/Cargo.toml
```

Para executar todas as suítes a partir de um shell compatível com Bash:

```bash
for project in hello echor catr headr wcr uniqr cutr commr findr; do
    cargo test --manifest-path "$project/Cargo.toml" || exit 1
done
```

Na primeira execução, o Cargo poderá precisar acessar a internet para baixar
as dependências registradas nos arquivos `Cargo.lock`.

Os scripts `mk-outs.sh` presentes em alguns projetos foram usados para gerar
arquivos de saída esperada. Como eles podem sobrescrever fixtures de teste,
vale revisá-los antes de executá-los.

## Organização recorrente

Nos exercícios mais completos, o código é dividido desta forma:

```text
<projeto>/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs       # chama a biblioteca, imprime erros e define o exit status
│   └── lib.rs        # argumentos, configuração e regras do programa
└── tests/
    ├── cli.rs        # testes de integração do executável
    ├── inputs/       # dados usados nos testes
    └── expected/     # resultados esperados
```

Essa separação mantém a função `main` pequena e deixa a lógica reutilizável e
mais fácil de testar. `hello` e `echor`, por serem os primeiros exercícios,
ainda concentram a implementação em `main.rs`.

## Conceitos praticados

O conjunto dos projetos cobre uma boa progressão de fundamentos e recursos
idiomáticos de Rust:

- estrutura de crates, compilação com Cargo e múltiplos binários;
- variáveis, tipos, `struct`, `enum`, pattern matching e guards;
- ownership, empréstimos, referências, slices e conversões entre tipos;
- `Option`, `Result`, propagação de erros com `?` e erros dinâmicos com
  `Box<dyn Error>`;
- traits como `Read`, `BufRead` e `Write`, incluindo trait objects;
- leitura incremental com buffers, evitando carregar arquivos inteiros na
  memória;
- iteradores, closures, `map`, `filter`, `flat_map`, `collect`, `transpose` e
  `enumerate`;
- parsing de argumentos e validação de flags com `clap`;
- diferenças entre bytes, caracteres Unicode e texto UTF-8;
- expressões regulares e conversão de posições baseadas em 1 para intervalos
  baseados em 0;
- processamento de CSV/TSV com delimitadores e escaping;
- travessia recursiva do sistema de arquivos;
- `stdin`, `stdout`, `stderr`, pipelines e códigos de saída;
- testes unitários e de integração, fixtures, casos de erro e diferenças entre
  plataformas.

Também há um aprendizado importante de design: ferramentas de linha de comando
precisam se comportar bem quando compostas. Isso inclui escrever dados em
`stdout`, diagnósticos em `stderr`, sinalizar falhas com um código de saída
adequado e processar entradas potencialmente grandes de forma incremental.

## Dependências utilizadas

As implementações priorizam a biblioteca padrão e adicionam crates específicas
quando o problema pede:

| Crate | Uso |
| --- | --- |
| `clap` | Definição e validação da interface de linha de comando |
| `regex` | Parsing de intervalos no `cutr` e filtros de nome no `findr` |
| `csv` | Leitura e escrita de registros delimitados no `cutr` |
| `walkdir` | Percurso recursivo de diretórios no `findr` |
| `assert_cmd` e `predicates` | Execução e validação dos binários nos testes |
| `rand` e `tempfile` | Criação segura de cenários temporários nos testes |

## Escopo e limitações

O objetivo é reproduzir o núcleo de cada comando enquanto se aprende Rust. Por
isso, somente as opções documentadas no `--help` de cada binário são
implementadas; compatibilidade POSIX/GNU completa, desempenho de produção e
todos os casos extremos não fazem parte do escopo. Os comentários no código
registram alternativas de implementação e decisões tomadas durante o estudo,
o que os torna uma parte útil do material.
