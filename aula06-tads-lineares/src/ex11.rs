use std::collections::VecDeque;

pub struct Trabalho {
    pub nome: String,
    pub paginas: u32,
}

pub fn imprimir_fila(trabalhos: Vec<Trabalho>) -> Vec<String> {
    let mut fila: VecDeque<Trabalho> = VecDeque::from(trabalhos);
    let mut saida: Vec<String> = Vec::with_capacity(fila.len());
    while let Some(t) = fila.pop_front() {
        saida.push(format!("Imprimindo '{}' ({} páginas)", t.nome, t.paginas));
    }
    saida
}

pub fn demo() {
    let trabalhos = vec![
        Trabalho { nome: "relatorio.pdf".to_string(), paginas: 12 },
        Trabalho { nome: "foto.png".to_string(), paginas: 1 },
        Trabalho { nome: "contrato.docx".to_string(), paginas: 5 },
    ];
    for linha in imprimir_fila(trabalhos) {
        println!("  {}", linha);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imprime_em_ordem_de_chegada() {
        let trabalhos = vec![
            Trabalho { nome: "a".to_string(), paginas: 2 },
            Trabalho { nome: "b".to_string(), paginas: 7 },
        ];
        let saida = imprimir_fila(trabalhos);
        assert_eq!(
            saida,
            vec![
                "Imprimindo 'a' (2 páginas)".to_string(),
                "Imprimindo 'b' (7 páginas)".to_string(),
            ]
        );
    }

    #[test]
    fn fila_vazia() {
        let saida = imprimir_fila(Vec::new());
        assert!(saida.is_empty());
    }
}
