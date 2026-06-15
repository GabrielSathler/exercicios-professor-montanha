pub fn pares_com_soma(lista: &[i32], alvo: i32) -> Vec<(i32, i32)> {
    let n = lista.len();
    let mut resultado: Vec<(i32, i32)> = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            if lista[i] + lista[j] == alvo {
                resultado.push((lista[i], lista[j]));
            }
        }
    }

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encontra_pares_com_soma() {
        let v = vec![1, 2, 3, 4, 5];
        let pares = pares_com_soma(&v, 6);
        assert_eq!(pares, vec![(1, 5), (2, 4)]);
    }

    #[test]
    fn sem_pares_retorna_vazio() {
        let v = vec![1, 2, 3];
        assert_eq!(pares_com_soma(&v, 100), Vec::new());
    }

    #[test]
    fn lista_vazia_retorna_vazio() {
        let v: Vec<i32> = Vec::new();
        assert_eq!(pares_com_soma(&v, 5), Vec::new());
    }
}
