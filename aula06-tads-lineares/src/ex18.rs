pub fn respostas() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "(a) Ctrl+Z de um editor",
            "Pilha (LIFO): desfaz o ultimo comando executado primeiro.",
        ),
        (
            "(b) Pedidos de restaurante em ordem",
            "Fila (FIFO): atende na ordem de chegada.",
        ),
        (
            "(c) Verificar tags HTML bem formadas",
            "Pilha: empilha a abertura e casa com o fechamento (aninhamento).",
        ),
        (
            "(d) Navegar arquivos de um diretorio em largura (BFS)",
            "Fila: visita os itens nivel a nivel.",
        ),
        (
            "(e) Sequencia de palavras e palindromo",
            "Deque: compara as duas pontas em direcao ao centro.",
        ),
    ]
}

pub fn demo() {
    for (cenario, resposta) in respostas() {
        println!("  {} -> {}", cenario, resposta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ha_cinco_cenarios() {
        assert_eq!(respostas().len(), 5);
    }

    #[test]
    fn associacoes_principais_estao_corretas() {
        let r = respostas();
        assert!(r[0].1.contains("Pilha"));
        assert!(r[1].1.contains("Fila"));
        assert!(r[4].1.contains("Deque"));
    }
}
