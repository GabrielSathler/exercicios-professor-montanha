pub fn produto_de_matrizes(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut c: Vec<Vec<i64>> = vec![vec![0i64; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplica_pela_identidade() {
        let a = vec![vec![1i64, 2], vec![3, 4]];
        let identidade = vec![vec![1i64, 0], vec![0, 1]];
        assert_eq!(produto_de_matrizes(&a, &identidade), a);
    }

    #[test]
    fn produto_dois_por_dois() {
        let a = vec![vec![1i64, 2], vec![3, 4]];
        let b = vec![vec![5i64, 6], vec![7, 8]];
        let esperado = vec![vec![19i64, 22], vec![43, 50]];
        assert_eq!(produto_de_matrizes(&a, &b), esperado);
    }

    #[test]
    fn matriz_vazia() {
        let a: Vec<Vec<i64>> = Vec::new();
        let b: Vec<Vec<i64>> = Vec::new();
        assert_eq!(produto_de_matrizes(&a, &b), Vec::<Vec<i64>>::new());
    }
}
