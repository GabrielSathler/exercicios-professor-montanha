use std::time::Instant;

fn busca_sequencial_simples(vetor: &[i32], alvo: i32) -> (Option<usize>, usize) {
    let mut posicao: Option<usize> = None;
    let mut operacoes: usize = 0;

    for (indice, &valor) in vetor.iter().enumerate() {
        operacoes += 1;
        if valor == alvo {
            posicao = Some(indice);
        }
    }

    (posicao, operacoes)
}

fn busca_sequencial_interrompida(vetor: &[i32], alvo: i32) -> (Option<usize>, usize) {
    let mut operacoes: usize = 0;

    for (indice, &valor) in vetor.iter().enumerate() {
        operacoes += 1;
        if valor == alvo {
            return (Some(indice), operacoes);
        }
    }

    (None, operacoes)
}

fn gerar_vetor(tamanho: usize) -> Vec<i32> {
    (1..=tamanho as i32).collect()
}

fn busca_sequencial_str(vetor: &[&str], alvo: &str) -> (Option<usize>, usize) {
    let mut operacoes: usize = 0;

    for (indice, &palavra) in vetor.iter().enumerate() {
        operacoes += 1;
        if palavra == alvo {
            return (Some(indice), operacoes);
        }
    }

    (None, operacoes)
}

fn contar_ocorrencias(vetor: &[i32], alvo: i32) -> usize {
    let mut total: usize = 0;

    for &valor in vetor {
        if valor == alvo {
            total += 1;
        }
    }

    total
}

fn tabela_operacoes() -> Vec<(usize, &'static str, usize, usize)> {
    let tamanhos: [usize; 3] = [1000, 10000, 100000];
    let posicoes: [&'static str; 4] = ["inicio", "meio", "final", "inexistente"];

    let mut linhas: Vec<(usize, &'static str, usize, usize)> = Vec::new();

    for &tamanho in &tamanhos {
        let vetor = gerar_vetor(tamanho);

        for &posicao in &posicoes {
            let alvo: i32 = match posicao {
                "inicio" => 5,
                "meio" => (tamanho / 2) as i32,
                "final" => tamanho as i32,
                _ => -1,
            };

            let (_, ops_simples) = busca_sequencial_simples(&vetor, alvo);
            let (_, ops_interrompida) = busca_sequencial_interrompida(&vetor, alvo);

            linhas.push((tamanho, posicao, ops_simples, ops_interrompida));
        }
    }

    linhas
}

fn busca_todas_posicoes(vetor: &[i32], alvo: i32) -> Vec<usize> {
    let mut posicoes: Vec<usize> = Vec::new();

    for (indice, &valor) in vetor.iter().enumerate() {
        if valor == alvo {
            posicoes.push(indice);
        }
    }

    posicoes
}

fn formatar_posicao(posicao: Option<usize>) -> String {
    match posicao {
        Some(indice) => format!("índice {}", indice),
        None => String::from("não encontrado"),
    }
}

fn demonstrar_cenario(rotulo: &str, vetor: &[i32], alvo: i32) {
    println!("Cenário: {} (alvo = {})", rotulo, alvo);

    let inicio_simples = Instant::now();
    let (pos_s, ops_s) = busca_sequencial_simples(vetor, alvo);
    let tempo_simples = inicio_simples.elapsed();

    let inicio_interrompida = Instant::now();
    let (pos_i, ops_i) = busca_sequencial_interrompida(vetor, alvo);
    let tempo_interrompida = inicio_interrompida.elapsed();

    println!(
        "  Simples......: posição = {:<16} | operações = {:>7} | tempo = {:?}",
        formatar_posicao(pos_s),
        ops_s,
        tempo_simples
    );
    println!(
        "  Interrompida.: posição = {:<16} | operações = {:>7} | tempo = {:?}",
        formatar_posicao(pos_i),
        ops_i,
        tempo_interrompida
    );
    println!();
}

fn main() {
    println!("===========================================================");
    println!("  Aula 02 - Busca Sequencial: simples vs. interrompida");
    println!("===========================================================\n");

    // evita 1_000_000 pra não demorar
    let tamanhos_demo: [usize; 2] = [1_000, 100_000];

    for &tamanho in &tamanhos_demo {
        let vetor = gerar_vetor(tamanho);
        println!("-----------------------------------------------------------");
        println!("Vetor de tamanho {}", tamanho);
        println!("-----------------------------------------------------------");

        demonstrar_cenario("início", &vetor, 5);
        demonstrar_cenario("meio", &vetor, (tamanho / 2) as i32);
        demonstrar_cenario("final", &vetor, tamanho as i32);
        demonstrar_cenario("inexistente", &vetor, -1);
    }

    println!("===========================================================");
    println!("  Exercícios Propostos (seção 9)");
    println!("===========================================================\n");

    let frutas: [&str; 5] = ["maçã", "banana", "laranja", "uva", "manga"];
    let (pos_str, ops_str) = busca_sequencial_str(&frutas, "uva");
    println!("Exercício 1 - Busca em strings:");
    println!(
        "  Procurando \"uva\" em {:?} -> {} (operações = {})",
        frutas,
        formatar_posicao(pos_str),
        ops_str
    );
    let (pos_str_n, ops_str_n) = busca_sequencial_str(&frutas, "kiwi");
    println!(
        "  Procurando \"kiwi\" em {:?} -> {} (operações = {})\n",
        frutas,
        formatar_posicao(pos_str_n),
        ops_str_n
    );

    let vetor_rep: Vec<i32> = vec![1, 2, 2, 3, 2, 4, 2, 5];
    let qtd = contar_ocorrencias(&vetor_rep, 2);
    println!("Exercício 2 - Contar ocorrências:");
    println!("  O valor 2 aparece {} vezes em {:?}\n", qtd, vetor_rep);

    println!("Exercício 3 - Tabela tamanho x operações:");
    let linhas = tabela_operacoes();
    println!(
        "  {:>8} | {:<12} | {:>12} | {:>16}",
        "tamanho", "posicao", "ops_simples", "ops_interrompida"
    );
    for &(tamanho, posicao, ops_simples, ops_interrompida) in &linhas {
        println!(
            "  {:>8} | {:<12} | {:>12} | {:>16}",
            tamanho, posicao, ops_simples, ops_interrompida
        );
    }

    let mut csv = String::from("tamanho,posicao,ops_simples,ops_interrompida\n");
    for &(tamanho, posicao, ops_simples, ops_interrompida) in &linhas {
        csv.push_str(&format!(
            "{},{},{},{}\n",
            tamanho, posicao, ops_simples, ops_interrompida
        ));
    }

    std::fs::create_dir_all("prints_execucao").ok();
    std::fs::write("prints_execucao/operacoes_por_tamanho.csv", csv)
        .expect("Falha ao gravar o arquivo CSV de operações");
    println!("\n  CSV gravado em: prints_execucao/operacoes_por_tamanho.csv\n");

    let vetor_todas: Vec<i32> = vec![7, 1, 7, 3, 7, 9, 7];
    let posicoes = busca_todas_posicoes(&vetor_todas, 7);
    println!("Exercício 4 - Todas as posições:");
    println!(
        "  O valor 7 aparece nos índices {:?} de {:?}",
        posicoes, vetor_todas
    );

    println!("\nFim da demonstração. Execute 'cargo test' para validar os testes.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn busca_simples_encontra_e_conta_n_operacoes() {
        let vetor = gerar_vetor(10);
        let (posicao, operacoes) = busca_sequencial_simples(&vetor, 4);
        assert_eq!(posicao, Some(3));
        assert_eq!(operacoes, vetor.len());
    }

    #[test]
    fn busca_simples_nao_encontra_mas_conta_n_operacoes() {
        let vetor = gerar_vetor(10);
        let (posicao, operacoes) = busca_sequencial_simples(&vetor, 999);
        assert_eq!(posicao, None);
        assert_eq!(operacoes, vetor.len());
    }

    #[test]
    fn busca_simples_retorna_ultima_ocorrencia() {
        let vetor: Vec<i32> = vec![1, 2, 3, 2, 5, 2];
        let (posicao, operacoes) = busca_sequencial_simples(&vetor, 2);
        assert_eq!(posicao, Some(5));
        assert_eq!(operacoes, vetor.len());
    }

    #[test]
    fn busca_interrompida_retorna_cedo_e_acha_primeira_ocorrencia() {
        let vetor: Vec<i32> = vec![1, 2, 3, 2, 5, 2];
        let (posicao, operacoes) = busca_sequencial_interrompida(&vetor, 2);
        assert_eq!(posicao, Some(1));
        assert!(operacoes <= vetor.len());
        assert_eq!(operacoes, 2);
    }

    #[test]
    fn busca_interrompida_nao_encontra_faz_n_operacoes() {
        let vetor = gerar_vetor(10);
        let (posicao, operacoes) = busca_sequencial_interrompida(&vetor, -1);
        assert_eq!(posicao, None);
        assert_eq!(operacoes, vetor.len());
    }

    #[test]
    fn gerar_vetor_produz_intervalo_correto() {
        assert_eq!(gerar_vetor(5), vec![1, 2, 3, 4, 5]);
        assert_eq!(gerar_vetor(0), Vec::<i32>::new());
    }

    #[test]
    fn contar_ocorrencias_com_repeticoes() {
        let vetor: Vec<i32> = vec![1, 2, 2, 3, 2, 4, 2, 5];
        assert_eq!(contar_ocorrencias(&vetor, 2), 4);
        assert_eq!(contar_ocorrencias(&vetor, 5), 1);
        assert_eq!(contar_ocorrencias(&vetor, 99), 0);
    }

    #[test]
    fn busca_todas_posicoes_retorna_todos_os_indices() {
        let vetor: Vec<i32> = vec![7, 1, 7, 3, 7, 9, 7];
        assert_eq!(busca_todas_posicoes(&vetor, 7), vec![0, 2, 4, 6]);
        assert_eq!(busca_todas_posicoes(&vetor, 42), Vec::<usize>::new());
    }

    #[test]
    fn busca_sequencial_str_acha_palavra() {
        let vetor: [&str; 4] = ["sol", "lua", "estrela", "cometa"];
        let (posicao, operacoes) = busca_sequencial_str(&vetor, "estrela");
        assert_eq!(posicao, Some(2));
        assert_eq!(operacoes, 3);
        let (pos_n, ops_n) = busca_sequencial_str(&vetor, "nuvem");
        assert_eq!(pos_n, None);
        assert_eq!(ops_n, vetor.len());
    }

    #[test]
    fn tabela_operacoes_tem_linhas_esperadas() {
        let linhas = tabela_operacoes();
        assert_eq!(linhas.len(), 12);

        let (tam, pos, ops_s, ops_i) = linhas[0];
        assert_eq!(tam, 1000);
        assert_eq!(pos, "inicio");
        assert_eq!(ops_s, 1000);
        assert_eq!(ops_i, 5);

        let (tam_x, pos_x, ops_sx, ops_ix) = linhas[3];
        assert_eq!(tam_x, 1000);
        assert_eq!(pos_x, "inexistente");
        assert_eq!(ops_sx, 1000);
        assert_eq!(ops_ix, 1000);
    }
}
