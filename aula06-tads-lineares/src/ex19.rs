use std::collections::VecDeque;

pub fn processar_em_lotes(fila: &mut VecDeque<i32>, tamanho_lote: usize) {
    if tamanho_lote == 0 {
        println!("  (tamanho_lote == 0: nada a processar)");
        return;
    }

    let mut numero_lote = 1;
    while !fila.is_empty() {
        let mut lote: Vec<i32> = Vec::with_capacity(tamanho_lote);
        for _ in 0..tamanho_lote {
            match fila.pop_front() {
                Some(x) => lote.push(x),
                None => break,
            }
        }
        println!("  Lote {}: {:?}", numero_lote, lote);
        numero_lote += 1;
    }
}

pub fn coletar_lotes(fila: &mut VecDeque<i32>, tamanho_lote: usize) -> Vec<Vec<i32>> {
    let mut lotes: Vec<Vec<i32>> = Vec::new();
    if tamanho_lote == 0 {
        return lotes;
    }
    while !fila.is_empty() {
        let mut lote: Vec<i32> = Vec::with_capacity(tamanho_lote);
        for _ in 0..tamanho_lote {
            match fila.pop_front() {
                Some(x) => lote.push(x),
                None => break,
            }
        }
        lotes.push(lote);
    }
    lotes
}

pub fn demo() {
    let mut fila: VecDeque<i32> = (1..=10).collect();
    println!("  fila = {:?}, tamanho_lote = 3", fila);
    processar_em_lotes(&mut fila, 3);
    println!("  fila apos processar (deve estar vazia): {:?}", fila);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_em_lotes_com_resto() {
        let mut fila: VecDeque<i32> = (1..=10).collect();
        let lotes = coletar_lotes(&mut fila, 3);
        assert_eq!(
            lotes,
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10]]
        );
        assert!(fila.is_empty());
    }

    #[test]
    fn tamanho_lote_zero_nao_consome() {
        let mut fila: VecDeque<i32> = (1..=3).collect();
        let lotes = coletar_lotes(&mut fila, 0);
        assert!(lotes.is_empty());
        assert_eq!(fila.len(), 3);
    }

    #[test]
    fn lote_maior_que_a_fila_gera_um_unico_lote() {
        let mut fila: VecDeque<i32> = (1..=2).collect();
        let lotes = coletar_lotes(&mut fila, 10);
        assert_eq!(lotes, vec![vec![1, 2]]);
    }
}
