use std::collections::VecDeque;

pub fn round_robin(processos: Vec<(String, u32)>, quantum: u32) -> Vec<(String, u32)> {
    // quantum 0 vira 1 pra nao travar o laco
    let quantum = quantum.max(1);

    let mut fila: VecDeque<(String, u32)> = processos.into_iter().collect();
    let mut relogio: u32 = 0;
    let mut conclusoes: Vec<(String, u32)> = Vec::new();

    while let Some((nome, restante)) = fila.pop_front() {
        if restante == 0 {
            conclusoes.push((nome, relogio));
            continue;
        }

        let fatia = quantum.min(restante);
        relogio += fatia;
        let novo_restante = restante - fatia;

        if novo_restante > 0 {
            fila.push_back((nome, novo_restante));
        } else {
            conclusoes.push((nome, relogio));
        }
    }

    conclusoes
}

pub fn demo() {
    let processos = vec![
        ("P1".to_string(), 5),
        ("P2".to_string(), 3),
        ("P3".to_string(), 8),
    ];
    let quantum = 2;
    println!("  processos = [(P1,5), (P2,3), (P3,8)], quantum = {}", quantum);
    let resultado = round_robin(processos, quantum);
    for (nome, conclusao) in &resultado {
        println!("  {} concluiu no tempo {}", nome, conclusao);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn um_unico_processo_conclui_no_seu_tempo_total() {
        let r = round_robin(vec![("A".to_string(), 7)], 3);
        assert_eq!(r, vec![("A".to_string(), 7)]);
    }

    #[test]
    fn exemplo_tres_processos() {
        let processos = vec![
            ("P1".to_string(), 5),
            ("P2".to_string(), 3),
            ("P3".to_string(), 8),
        ];
        let r = round_robin(processos, 2);
        assert_eq!(
            r,
            vec![
                ("P2".to_string(), 9),
                ("P1".to_string(), 12),
                ("P3".to_string(), 16),
            ]
        );
    }

    #[test]
    fn quantum_maior_que_tudo_vira_fifo_simples() {
        let processos = vec![
            ("X".to_string(), 4),
            ("Y".to_string(), 2),
            ("Z".to_string(), 6),
        ];
        let r = round_robin(processos, 100);
        assert_eq!(
            r,
            vec![
                ("X".to_string(), 4),
                ("Y".to_string(), 6),
                ("Z".to_string(), 12),
            ]
        );
    }

    #[test]
    fn processo_com_tempo_zero_conclui_no_instante_atual() {
        let r = round_robin(vec![("vazio".to_string(), 0)], 2);
        assert_eq!(r, vec![("vazio".to_string(), 0)]);
    }
}
