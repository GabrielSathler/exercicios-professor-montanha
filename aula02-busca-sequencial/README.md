# Aula 02 - Busca Sequencial em Rust

Projeto Cargo que implementa e compara dois algoritmos de busca sequencial em vetores de
inteiros: a busca simples (que percorre o vetor inteiro mesmo depois de achar o alvo) e a
busca interrompida (que para na primeira ocorrência). A ideia é comparar na prática o
número de operações e o tempo em vários cenários (alvo no início, no meio, no final e
inexistente) para tamanhos diferentes de vetor.

Além disso, resolve os 4 exercícios propostos da seção 9 do tutorial: busca em vetores de
strings, contagem de ocorrências, tabela tamanho x operações (exportada pra CSV) e busca
de todas as posições do alvo. Tudo com a biblioteca padrão (std).

## Funções e complexidade

- busca_sequencial_simples: percorre o vetor todo. O(n) sempre.
- busca_sequencial_interrompida: para na primeira ocorrência. Melhor caso O(1), pior caso O(n).
- gerar_vetor: cria [1, 2, ..., tamanho]. O(n).
- busca_sequencial_str: busca interrompida em vetor de &str. O(n).
- contar_ocorrencias: conta quantas vezes o alvo aparece. O(n).
- busca_todas_posicoes: retorna todos os índices do alvo. O(n).
- tabela_operacoes: gera as linhas (tamanho, posição, ops). O(n) por tamanho.

Na busca simples o número de operações é sempre vetor.len(), porque ela nunca interrompe o
laço (daí o O(n) garantido). Na busca interrompida o melhor caso é O(1) (alvo no início) e
o pior é O(n) (alvo no final ou inexistente); o Big-O do pior caso é O(n).

## Resultados esperados

Número de operações por cenário (traçado a partir do código):

| Tamanho | Posição     | ops_simples | ops_interrompida |
|---------|-------------|-------------|------------------|
| 1000    | início      | 1000        | 5                |
| 1000    | meio        | 1000        | 500              |
| 1000    | final       | 1000        | 1000             |
| 1000    | inexistente | 1000        | 1000             |
| 100000  | início      | 100000      | 5                |
| 100000  | meio        | 100000      | 50000            |
| 100000  | final       | 100000      | 100000           |
| 100000  | inexistente | 100000      | 100000           |

A coluna "meio" usa alvo = tamanho / 2 (índice tamanho/2 - 1), então a busca interrompida
faz tamanho/2 operações. A coluna "início" usa alvo = 5 (índice 4), então 5 operações.

## Gráfico tamanho x operações

Ao rodar, o programa grava prints_execucao/operacoes_por_tamanho.csv (cabeçalho
tamanho,posicao,ops_simples,ops_interrompida) com os tamanhos 1000, 10000 e 100000 nos
quatro cenários. Pra fazer o gráfico é só abrir o CSV numa planilha (Excel, LibreOffice
ou Google Sheets) e inserir um gráfico de linhas com as colunas tamanho, ops_simples e
ops_interrompida - dá pra ver as operações crescendo linear com o tamanho (O(n)).

## Como executar

Dentro da pasta aula02-busca-sequencial/:

```
cargo run    # roda o experimento, imprime os resultados e gera o CSV
cargo test   # roda os testes (deve dar "test result: ok")
```

Importante: rode o cargo run dentro da pasta do projeto, porque o CSV é gravado em caminho
relativo (prints_execucao/operacoes_por_tamanho.csv).

## Estrutura

```
aula02-busca-sequencial/
  Cargo.toml
  src/main.rs
  README.md
  prints_execucao/
    saida_esperada.txt
    operacoes_por_tamanho.csv
```
