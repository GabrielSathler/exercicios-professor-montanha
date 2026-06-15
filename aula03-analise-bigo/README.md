# Aula 03 - Análise de Complexidade (Big-O)

Análise da complexidade de tempo (Big-O) de 10 algoritmos. Para cada um coloquei a
lógica e a justificativa da classe de complexidade. Os códigos originais em Python
estão na pasta [python/](./python).

Os mesmos 10 algoritmos foram reescritos em Rust na pasta
[../aula04-reescrita-rust](../aula04-reescrita-rust).

Big-O descreve como o tempo (ou o número de operações) cresce em função do tamanho n
da entrada, no pior caso, ignorando constantes.

## Exercício 1 - verificar_primeiro

Complexidade: O(1)

Lógica: se a lista estiver vazia retorna None; senão retorna o primeiro elemento
(acesso direto ao índice 0).

Justificativa: não tem laço. O len, a comparação e o acesso lista[0] são operações de
tempo constante, não importa o tamanho da lista. Por isso O(1).

## Exercício 2 - somar_lista

Complexidade: O(n)

Lógica: começa o total em 0 e percorre a lista somando cada elemento, depois retorna a soma.

Justificativa: um único laço que roda n vezes (uma por elemento), cada uma com trabalho
constante. O tempo cresce proporcional a n, então é O(n).

## Exercício 3 - busca_binaria

Complexidade: O(log n)

Lógica: na lista ordenada, compara o alvo com o elemento do meio. Se for igual achou; se
o alvo é maior, descarta a metade da esquerda; se é menor, descarta a da direita. Repete
até achar ou acabar o intervalo.

Justificativa: a cada passo o espaço de busca cai pela metade (n -> n/2 -> n/4 -> ... -> 1).
São cerca de log2(n) passos até sobrar 1 elemento, com trabalho constante por passo. Logo O(log n).

## Exercício 4 - pares_com_soma

Complexidade: O(n^2)

Lógica: para cada elemento i percorre todos os seguintes j (com j > i) e imprime o par
quando lista[i] + lista[j] é igual ao alvo.

Justificativa: dois laços aninhados que percorrem todos os pares distintos, cerca de
n*(n-1)/2 combinações. Isso cresce proporcional a n^2, então O(n^2).

## Exercício 5 - imprimir_pares_e_soma

Complexidade: O(n^2)

Lógica: o Bloco 1 imprime cada elemento (um laço simples). O Bloco 2 imprime todos os
pares possíveis com dois laços aninhados que percorrem a lista inteira nos dois níveis.

Justificativa: o Bloco 1 é O(n) e o Bloco 2 é O(n^2) (n x n iterações). Somando, O(n) + O(n^2),
e para n grande o termo que domina é o quadrático. Resultado: O(n^2).

## Exercício 6 - potencias_de_dois

Complexidade: O(log n)

Lógica: começa com i = 1 e, enquanto i < n, imprime i e multiplica por 2. Vai passando
pelas potências de dois (1, 2, 4, 8, ...) menores que n.

Justificativa: i dobra a cada iteração (1, 2, 4, ..., 2^k). O laço para quando 2^k >= n,
ou seja, depois de uns log2(n) passos. Trabalho constante por passo, então O(log n).

## Exercício 7 - fibonacci_recursivo

Complexidade: O(2^n) (exponencial; mais exatamente O(phi^n), com phi ~ 1.618)

Lógica: calcula o n-ésimo Fibonacci de forma recursiva: para n <= 1 retorna n, senão
retorna fib(n-1) + fib(n-2).

Justificativa: cada chamada gera duas novas chamadas, formando uma árvore de recursão que
quase dobra a cada nível. Os mesmos subproblemas são recalculados várias vezes (sem
memoização). O número de chamadas cresce exponencialmente, da ordem de O(2^n).

## Exercício 8 - ordenacao_bolha

Complexidade: O(n^2)

Lógica: a cada passada compara elementos vizinhos e troca os que estão fora de ordem,
empurrando o maior para o fim. A cada passada o maior que falta vai pra posição certa.

Justificativa: dois laços aninhados; o de dentro faz uns n - i - 1 comparações por passada.
Somando todas dá cerca de n^2/2 comparações no pior caso. Ignorando a constante, O(n^2).

## Exercício 9 - produto_de_matrizes

Complexidade: O(n^3)

Lógica: multiplica duas matrizes n x n de forma ingênua: para cada posição (i, j) do
resultado, soma os produtos A[i][k] * B[k][j] passando por todos os k.

Justificativa: três laços aninhados de tamanho n, dando n * n * n = n^3 operações. O tempo
cresce com o cubo de n, ou seja O(n^3).

Obs.: a tabela da aula não lista O(n^3), mas esse algoritmo é claramente cúbico; por isso
acrescentei a linha O(n^3) na tabela do fim.

## Exercício 10 - merge_sort

Complexidade: O(n log n)

Lógica: ordenação por intercalação (dividir para conquistar): divide a lista ao meio
recursivamente até sobrarem sublistas de tamanho 1 e depois junta (merge) os pares de
sublistas ordenadas comparando elemento a elemento.

Justificativa: a divisão pela metade gera uns log2(n) níveis. Em cada nível a fusão
percorre os n elementos no total, custando O(n) por nível. Níveis vezes custo por nível
dá O(n log n).

## Tabela de referência

| Notação    | Nome         | Quando aparece |
|------------|--------------|----------------|
| O(1)       | constante    | acesso direto, ex. lista[0] (Ex. 1) |
| O(log n)   | logarítmica  | busca binária (Ex. 3), potências de dois (Ex. 6) |
| O(n)       | linear       | somar uma lista (Ex. 2) |
| O(n log n) | linearítmica | merge sort (Ex. 10) |
| O(n^2)     | quadrática   | bubble sort (Ex. 8), pares (Ex. 4 e 5) |
| O(n^3)     | cúbica       | multiplicação ingênua de matrizes (Ex. 9) |
| O(2^n)     | exponencial  | fibonacci recursivo (Ex. 7) |
| O(n!)      | fatorial     | gerar todas as permutações de uma lista |

A linha O(n^3) não estava na tabela original da aula; coloquei porque o Exercício 9 cai
nessa classe.
