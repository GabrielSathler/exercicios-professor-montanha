# Aula 06 - TADs Lineares (Vec, Pilha, Fila, Deque)

Projeto Cargo (binário) com 20 exercícios em Rust, organizados em módulos src/ex01.rs até
src/ex20.rs. Cada módulo tem uma pub fn demo() (demonstração com println!) e um bloco de
testes. O src/main.rs declara os 20 módulos e chama exNN::demo() em cada um.

Só biblioteca padrão (std), sem dependências externas. Onde precisa de "aleatoriedade"
(ex. fila de banco) uso um LCG determinístico próprio.

## Como executar

```
cd aula06-tads-lineares
cargo run    # roda as 20 demonstrações
cargo test   # roda os testes (deve dar "test result: ok")
```

## Lista dos 20 exercícios

Pra cada um: estrutura/TAD usada, complexidade e uma linha de explicação.

Grupo 1 - Vec

1. Inversão com Vec - Vec (push/pop). O(n) tempo e espaço. Tira com pop() e empurra num
   Vec novo, sem usar .reverse().
2. Contador de ocorrências - Vec + HashMap. O(n) tempo, O(k) espaço (k = chars distintos).
   Itera a fatia e incrementa o contador.
3. Remoção condicional (remover pares) - Vec. O(n) tempo e espaço. Empurra só os ímpares
   num Vec novo, sem .retain().
4. Mescla ordenada - Vec com dois índices. O(n+m). É a etapa de merge do merge sort:
   compara os topos e avança o índice do menor.

Grupo 2 - Pilha

5. Calculadora RPN - pilha (Vec<f64>). O(n). Números empilham; operadores desempilham dois
   e empilham o resultado; retorna o topo (ou None se inválida).
6. Histórico de navegação - duas pilhas (Vec<String>). O(1) por operação. back guarda o
   passado, forward o futuro desfeito; visitar limpa o forward.
7. Desfazer/Refazer (editor) - duas pilhas de snapshots. O(k) por operação (k = tamanho do
   texto, por causa do clone). digitar salva snapshot em desfazer e limpa refazer.
8. Símbolos balanceados - pilha (Vec<char>). O(n). Empilha o fechamento esperado e confere
   com o topo a cada fechamento.
9. Pilha com mínimo O(1) - pilha + pilha auxiliar de mínimos. push/pop/min em O(1).

Grupo 3 - Fila

10. Simulador de fila de banco - fila (VecDeque). O(n). Um caixa, eventos por chegada;
    espera = max(0, fim_anterior - chegada); retorna a espera média. Dados gerados por LCG.
11. Impressora compartilhada - fila (VecDeque<Trabalho>). O(n). Processa em ordem de
    chegada com pop_front.
12. Buffer de mensagens (fila circular) - Vec<Option<i32>> + índices. inserir O(1).
    Sobrescreve o mais antigo quando cheia.
13. Fila de prioridade manual - Vec<(u8, i32)> com busca linear. inserir O(1), remover O(n).
    Remove o de maior prioridade; empate por FIFO.

Grupo 4 - Deque

14. Palíndromo com VecDeque<char> - deque. O(n). Normaliza e compara pop_front com pop_back
    até o centro.
15. Janela deslizante máxima - deque de índices (monotônico). O(n). O deque guarda índices
    em ordem decrescente de valor; tira os de fora da janela pela frente e os menores pelo fundo.
16. Fila de tarefas com prioridade de frente - VecDeque<String>. push O(1), remover O(1).
    urgente = push_front, normal = push_back, proxima = pop_front.

Grupo 5 - Reflexão e Análise

17. Comparação de desempenho (3 filas) - FilaVecIngenua (remove(0)), VecDeque e FilaCircular.
    A ingênua é O(n) por dequeue (desloca todos os elementos), as outras O(1). Mede com
    std::time::Instant.
18. Quando usar qual TAD? - reflexão (ver seção abaixo). demo() imprime as respostas.
19. Fila com iteração controlada (lotes) - fila (VecDeque<i32>). O(n). Remove e imprime em
    lotes de tamanho fixo até esvaziar (trata tamanho_lote == 0).
20. Round Robin (fila circular) - fila (VecDeque<(String, u32)>). O(total_exec / quantum).
    A cada fatia executa min(quantum, restante); se sobrar tempo volta pro fim, senão
    registra a conclusão.

## Exercício 18 - Quando usar qual TAD?

A escolha depende da ordem em que os elementos precisam sair:

(a) Ctrl+Z de um editor -> Pilha (LIFO). O "desfazer" reverte sempre o último comando
primeiro, que é o comportamento last-in first-out da pilha.

(b) Pedidos de restaurante em ordem -> Fila (FIFO). Quem pede primeiro é atendido primeiro,
preservando a ordem de chegada.

(c) Tags HTML bem formadas -> Pilha. Ao ler uma abertura, empilha; ao ler um fechamento,
ele tem que casar com a última aberta ainda não fechada (topo da pilha). É o mesmo
princípio do ex08.

(d) Navegar arquivos de um diretório em largura (BFS) -> Fila. A busca em largura visita
todos os nós de um nível antes de descer; a fila garante que os descobertos primeiro sejam
expandidos primeiro. (Com pilha viraria busca em profundidade.)

(e) Sequência de palavras é palíndromo -> Deque. Palíndromo se lê igual do início e do fim;
o deque deixa comparar pelas duas pontas (pop_front vs pop_back) indo pro centro, em O(n).

Resumindo: pilha quando importa o último (reversão, aninhamento), fila quando importa o
primeiro (ordem de chegada, BFS), deque quando se trabalha pelas duas pontas.

## Estrutura

```
aula06-tads-lineares/
  Cargo.toml
  README.md
  prints_execucao/
    saida_esperada.txt
  src/
    main.rs
    ex01.rs ... ex20.rs
```
