pub fn mesclar_ordenado(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut resultado: Vec<i32> = Vec::with_capacity(a.len() + b.len());
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            resultado.push(a[i]);
            i += 1;
        } else {
            resultado.push(b[j]);
            j += 1;
        }
    }
    // copia a sobra (só um dos dois laços roda)
    while i < a.len() {
        resultado.push(a[i]);
        i += 1;
    }
    while j < b.len() {
        resultado.push(b[j]);
        j += 1;
    }
    resultado
}

pub fn demo() {
    let a: Vec<i32> = vec![1, 3, 5, 7];
    let b: Vec<i32> = vec![2, 4, 6, 8, 10];
    let mesclado = mesclar_ordenado(&a, &b);
    println!("  a        = {:?}", a);
    println!("  b        = {:?}", b);
    println!("  mesclado = {:?}", mesclado);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mescla_intercalado() {
        assert_eq!(
            mesclar_ordenado(&[1, 3, 5], &[2, 4, 6]),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn um_lado_vazio() {
        assert_eq!(mesclar_ordenado(&[], &[1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(mesclar_ordenado(&[1, 2, 3], &[]), vec![1, 2, 3]);
    }

    #[test]
    fn tamanhos_diferentes() {
        assert_eq!(
            mesclar_ordenado(&[1, 10], &[2, 3, 4, 5]),
            vec![1, 2, 3, 4, 5, 10]
        );
    }
}
