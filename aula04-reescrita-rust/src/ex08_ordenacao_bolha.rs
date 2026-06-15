pub fn ordenacao_bolha(lista: &mut [i32]) {
    let n = lista.len();
    // com n == 0 o laco externo nem roda, entao n - i - 1 nao da underflow
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if lista[j] > lista[j + 1] {
                lista.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordena_lista_desordenada() {
        let mut v = vec![5, 2, 9, 1, 5, 6];
        ordenacao_bolha(&mut v);
        assert_eq!(v, vec![1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn lista_ja_ordenada_permanece() {
        let mut v = vec![1, 2, 3, 4];
        ordenacao_bolha(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn lista_vazia_nao_da_panic() {
        let mut v: Vec<i32> = Vec::new();
        ordenacao_bolha(&mut v);
        assert_eq!(v, Vec::<i32>::new());
    }

    #[test]
    fn lista_de_um_elemento() {
        let mut v = vec![42];
        ordenacao_bolha(&mut v);
        assert_eq!(v, vec![42]);
    }
}
