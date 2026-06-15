# Aula 04 - Reescrita de Algoritmos em Rust

Os 10 algoritmos da aula 03 reescritos em Rust, num projeto Cargo único com um módulo
por exercício dentro de src/. Cada módulo tem ao menos um #[test]. Usa só a biblioteca
padrão (std), edition 2021.

## Exercício 1 - Verificar o primeiro elemento
Complexidade: O(1)
Lógica: retorna o primeiro elemento quando a lista não está vazia (lista.first().copied())
e None caso contrário.
Justificativa: acessar a primeira posição não depende do tamanho n, o custo é constante.

## Exercício 2 - Somar a lista
Complexidade: O(n)
Lógica: acumula numa variável a soma de todos os elementos e retorna.
Justificativa: um único laço que visita cada um dos n elementos uma vez, então o tempo
cresce linear com n.

## Exercício 3 - Busca binária
Complexidade: O(log n)
Lógica: na lista ordenada, compara o alvo com o elemento do meio e descarta metade do
intervalo a cada passo. Uso isize nos limites porque direita pode chegar a -1.
Justificativa: o espaço de busca cai pela metade a cada iteração, então o número de passos
é da ordem de log2(n).

## Exercício 4 - Pares com soma alvo
Complexidade: O(n^2)
Lógica: dois laços aninhados (i e j > i) testam cada par distinto e coletam num Vec os
pares cuja soma é igual ao alvo.
Justificativa: são uns n*(n-1)/2 pares por causa dos laços aninhados; o termo que domina
é n^2.

## Exercício 5 - Imprimir elementos e todos os pares
Complexidade: O(n^2)
Lógica: o Bloco 1 imprime cada elemento (uma passada). O Bloco 2 gera e imprime todos os
pares (i, j), inclusive (i, i), usando a função auxiliar pares_completos.
Justificativa: Bloco 1 é O(n) e Bloco 2 é O(n^2) (dois laços de tamanho n). O total é O(n^2).

## Exercício 6 - Potências de dois
Complexidade: O(log n)
Lógica: começando em 1, multiplica por 2 a cada passo enquanto o valor for menor que n.
Justificativa: o valor dobra a cada iteração, chegando em n em uns log2(n) passos.

## Exercício 7 - Fibonacci recursivo
Complexidade: O(2^n)
Lógica: a versão recursiva devolve n para n <= 1 e senão soma fib(n-1) + fib(n-2). Tem
também uma versão iterativa O(n) (bônus) pra comparar.
Justificativa: cada chamada gera duas novas e recalcula subproblemas repetidos, formando
uma árvore com número de nós da ordem de 2^n. A versão iterativa faz um laço só e é O(n).

## Exercício 8 - Ordenação por bolha
Complexidade: O(n^2)
Lógica: a cada passada do laço externo, o interno compara vizinhos e troca os que estão
fora de ordem, empurrando o maior pro fim. Trata fatia vazia sem panic.
Justificativa: dois laços aninhados que no total fazem da ordem de n^2 comparações/trocas.

## Exercício 9 - Produto de matrizes
Complexidade: O(n^3)
Lógica: para matrizes n x n, três laços aninhados (i, j, k) acumulam em c[i][j] a soma dos
produtos a[i][k] * b[k][j]. Uso i64 pra não estourar.
Justificativa: três laços de tamanho n dão n*n*n operações, ou seja O(n^3).

## Exercício 10 - Merge sort
Complexidade: O(n log n)
Lógica: divide a lista ao meio recursivamente até o caso base (0 ou 1 elemento) e depois
intercala (merge) as metades ordenadas.
Justificativa: a divisão dá uns log2(n) níveis e em cada nível a intercalação percorre os
n elementos, dando O(n log n).

## Como executar

Dentro da pasta aula04-reescrita-rust/:

```
cargo run    # roda o main e demonstra cada função
cargo test   # roda os testes (deve dar "test result: ok")
```

O projeto é um crate binário. O src/main.rs declara um módulo por exercício
(mod ex01_verificar_primeiro; ... mod ex10_merge_sort;) e o main chama cada função pelo
módulo, ex. ex03_busca_binaria::busca_binaria(&v, 7). Cada módulo tem suas funções e seu
próprio bloco de testes.
