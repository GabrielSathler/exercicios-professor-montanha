mod ex01;
mod ex02;
mod ex03;
mod ex04;
mod ex05;
mod ex06;
mod ex07;
mod ex08;
mod ex09;
mod ex10;
mod ex11;
mod ex12;
mod ex13;
mod ex14;
mod ex15;
mod ex16;
mod ex17;
mod ex18;
mod ex19;
mod ex20;

fn titulo(n: u32, nome: &str) {
    println!("\n===== Exercício {:02} - {} =====", n, nome);
}

fn main() {
    println!("Atividade aula06 - TADs Lineares (Vec, Pilha, Fila, Deque)");

    titulo(1, "Inversão com Vec");
    ex01::demo();
    titulo(2, "Contador de ocorrências");
    ex02::demo();
    titulo(3, "Remoção condicional (remover pares)");
    ex03::demo();
    titulo(4, "Mescla ordenada");
    ex04::demo();

    titulo(5, "Calculadora RPN");
    ex05::demo();
    titulo(6, "Histórico de navegação");
    ex06::demo();
    titulo(7, "Desfazer/Refazer");
    ex07::demo();
    titulo(8, "Símbolos balanceados");
    ex08::demo();
    titulo(9, "Pilha com mínimo O(1)");
    ex09::demo();

    titulo(10, "Simulador de fila de banco");
    ex10::demo();
    titulo(11, "Impressora compartilhada");
    ex11::demo();
    titulo(12, "Buffer de mensagens (fila circular)");
    ex12::demo();
    titulo(13, "Fila de prioridade manual");
    ex13::demo();

    titulo(14, "Palíndromo com VecDeque");
    ex14::demo();
    titulo(15, "Janela deslizante máxima");
    ex15::demo();
    titulo(16, "Fila de tarefas com prioridade de frente");
    ex16::demo();

    titulo(17, "Comparação de desempenho (3 filas)");
    ex17::demo();
    titulo(18, "Quando usar qual TAD?");
    ex18::demo();
    titulo(19, "Fila com iteração controlada (lotes)");
    ex19::demo();
    titulo(20, "Round Robin (fila circular)");
    ex20::demo();

    println!("\nFim das demonstrações.");
}
