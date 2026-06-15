mod ex01_verificar_primeiro;
mod ex02_somar_lista;
mod ex03_busca_binaria;
mod ex04_pares_com_soma;
mod ex05_imprimir_pares_e_pares;
mod ex06_potencias_de_dois;
mod ex07_fibonacci_recursivo;
mod ex08_ordenacao_bolha;
mod ex09_produto_de_matrizes;
mod ex10_merge_sort;

fn main() {
    println!("== Exercicio 01 - Verificar primeiro - O(1) ==");
    let v1 = vec![10, 20, 30];
    println!("Primeiro de {:?}: {:?}", v1, ex01_verificar_primeiro::verificar_primeiro(&v1));
    let vazia: Vec<i32> = Vec::new();
    println!("Primeiro de {:?}: {:?}", vazia, ex01_verificar_primeiro::verificar_primeiro(&vazia));
    println!();

    println!("== Exercicio 02 - Somar lista - O(n) ==");
    let v2 = vec![1, 2, 3, 4, 5];
    println!("Soma de {:?}: {}", v2, ex02_somar_lista::somar_lista(&v2));
    println!();

    println!("== Exercicio 03 - Busca binaria - O(log n) ==");
    let v3 = vec![1, 3, 5, 7, 9, 11];
    println!("Buscar 7 em {:?}: {:?}", v3, ex03_busca_binaria::busca_binaria(&v3, 7));
    println!("Buscar 4 em {:?}: {:?}", v3, ex03_busca_binaria::busca_binaria(&v3, 4));
    println!();

    println!("== Exercicio 04 - Pares com soma - O(n^2) ==");
    let v4 = vec![1, 2, 3, 4, 5];
    let pares = ex04_pares_com_soma::pares_com_soma(&v4, 6);
    println!("Pares de {:?} que somam 6:", v4);
    for (a, b) in &pares {
        println!("  ({}, {})", a, b);
    }
    println!();

    println!("== Exercicio 05 - Imprimir elementos e pares - O(n^2) ==");
    let v5 = vec![1, 2, 3];
    ex05_imprimir_pares_e_pares::imprimir_pares_e_pares(&v5);
    println!();

    println!("== Exercicio 06 - Potencias de dois - O(log n) ==");
    let limite: u64 = 100;
    let potencias = ex06_potencias_de_dois::potencias_de_dois(limite);
    println!("Potencias de 2 menores que {}:", limite);
    for p in &potencias {
        println!("  {}", p);
    }
    println!();

    println!("== Exercicio 07 - Fibonacci - O(2^n) recursivo / O(n) iterativo ==");
    let n_fib: u64 = 10;
    println!(
        "fibonacci_recursivo({}) = {}",
        n_fib,
        ex07_fibonacci_recursivo::fibonacci_recursivo(n_fib)
    );
    println!(
        "fibonacci_iterativo({}) = {}",
        n_fib,
        ex07_fibonacci_recursivo::fibonacci_iterativo(n_fib)
    );
    println!();

    println!("== Exercicio 08 - Ordenacao bolha - O(n^2) ==");
    let mut v8 = vec![5, 2, 9, 1, 5, 6];
    println!("Antes:  {:?}", v8);
    ex08_ordenacao_bolha::ordenacao_bolha(&mut v8);
    println!("Depois: {:?}", v8);
    println!();

    println!("== Exercicio 09 - Produto de matrizes - O(n^3) ==");
    let a = vec![vec![1i64, 2], vec![3, 4]];
    let b = vec![vec![5i64, 6], vec![7, 8]];
    let c = ex09_produto_de_matrizes::produto_de_matrizes(&a, &b);
    println!("A = {:?}", a);
    println!("B = {:?}", b);
    println!("A x B = {:?}", c);
    println!();

    println!("== Exercicio 10 - Merge sort - O(n log n) ==");
    let v10 = vec![5, 2, 9, 1, 5, 6];
    println!("Antes:  {:?}", v10);
    let ordenada = ex10_merge_sort::merge_sort(v10);
    println!("Depois: {:?}", ordenada);
}
