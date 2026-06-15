pub fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    let mut esquerda: isize = 0;
    // isize porque direita pode chegar a -1
    let mut direita: isize = lista.len() as isize - 1;

    while esquerda <= direita {
        let meio: isize = (esquerda + direita) / 2;
        let valor = lista[meio as usize];

        if valor == alvo {
            return Some(meio as usize);
        } else if valor < alvo {
            esquerda = meio + 1;
        } else {
            direita = meio - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encontra_elemento_existente() {
        let v = vec![1, 3, 5, 7, 9, 11];
        assert_eq!(busca_binaria(&v, 7), Some(3));
    }

    #[test]
    fn encontra_primeiro_e_ultimo() {
        let v = vec![1, 3, 5, 7, 9, 11];
        assert_eq!(busca_binaria(&v, 1), Some(0));
        assert_eq!(busca_binaria(&v, 11), Some(5));
    }

    #[test]
    fn nao_encontra_retorna_none() {
        let v = vec![1, 3, 5, 7, 9, 11];
        assert_eq!(busca_binaria(&v, 4), None);
        assert_eq!(busca_binaria(&v, 0), None);
        assert_eq!(busca_binaria(&v, 99), None);
    }

    #[test]
    fn lista_vazia_retorna_none() {
        let v: Vec<i32> = Vec::new();
        assert_eq!(busca_binaria(&v, 1), None);
    }
}
