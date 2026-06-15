use std::collections::VecDeque;

pub fn janela_maxima(v: &[i32], k: usize) -> Vec<i32> {
    if k == 0 || k > v.len() {
        return Vec::new();
    }

    let mut resultado: Vec<i32> = Vec::with_capacity(v.len() - k + 1);
    // deque guarda indices, valores em ordem decrescente da frente ao fundo
    let mut deque: VecDeque<usize> = VecDeque::new();

    for i in 0..v.len() {
        if let Some(&frente) = deque.front() {
            if frente + k <= i {
                deque.pop_front();
            }
        }

        while let Some(&ultimo) = deque.back() {
            if v[ultimo] <= v[i] {
                deque.pop_back();
            } else {
                break;
            }
        }

        deque.push_back(i);

        if i + 1 >= k {
            let idx_max = *deque.front().unwrap();
            resultado.push(v[idx_max]);
        }
    }

    resultado
}

pub fn demo() {
    let v = [1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    println!("  v = {:?}, k = {}", v, k);
    println!("  janela_maxima = {:?}", janela_maxima(&v, k));
    println!("  k = 0  -> {:?}", janela_maxima(&v, 0));
    println!("  k = 99 -> {:?}", janela_maxima(&v, 99));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemplo_do_brief() {
        let v = [1, 3, -1, -3, 5, 3, 6, 7];
        assert_eq!(janela_maxima(&v, 3), vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn k_igual_um_devolve_o_proprio_vetor() {
        let v = [4, 2, 9, 1];
        assert_eq!(janela_maxima(&v, 1), vec![4, 2, 9, 1]);
    }

    #[test]
    fn casos_de_borda_retornam_vazio() {
        let v = [1, 2, 3];
        assert!(janela_maxima(&v, 0).is_empty());
        assert!(janela_maxima(&v, 4).is_empty());
        assert!(janela_maxima(&[], 1).is_empty());
    }
}
