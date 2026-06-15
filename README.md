# Exercícios - Estruturas de Dados e Análise de Algoritmos

Estudante: Gabriel Sathler
Turma: Estruturas de Dados e Análise de Algoritmos
Professor: Alexandre de Oliveira

Repositório com os exercícios práticos da disciplina, organizados por aula.

## Organização

Cada aula está em uma pasta, com o código comentado, a estrutura de dados usada e a
análise de complexidade (Big-O) quando o enunciado pede. Cada pasta tem seu próprio README.

- aula02-busca-sequencial - busca sequencial simples x com interrupção, mais 4 exercícios
  propostos do tutorial (Rust).
- aula03-analise-bigo - análise de complexidade Big-O de 10 algoritmos (Python + documento
  de análise).
- aula04-reescrita-rust - os mesmos 10 algoritmos da aula 03 reescritos em Rust.
- aula06-tads-lineares - 20 exercícios de TADs lineares: Vec, pilha, fila e deque (Rust).

## Como executar

As atividades em Rust são projetos Cargo. Precisa do Rust instalado
(https://www.rust-lang.org/tools/install).

```
cd aula04-reescrita-rust
cargo run     # roda as demonstrações
cargo test    # roda os testes
```

A aula 03 é um documento de análise (README.md). Os scripts Python podem ser rodados com,
por exemplo:

```
cd aula03-analise-bigo
python python/ex03_busca_binaria.py
```

Cada pasta tem um README com a análise de complexidade detalhada e, quando faz sentido,
uma pasta prints_execucao com a saída esperada.
