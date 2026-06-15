pub fn remover_pares(v: &[i32]) -> Vec<i32> {
    let mut resultado: Vec<i32> = Vec::new();
    for &x in v {
        if x % 2 != 0 {
            resultado.push(x);
        }
    }
    resultado
}

pub fn demo() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let impares = remover_pares(&v);
    println!("  entrada = {:?}", v);
    println!("  ímpares = {:?}", impares);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_pares_basico() {
        assert_eq!(remover_pares(&[1, 2, 3, 4, 5, 6]), vec![1, 3, 5]);
    }

    #[test]
    fn lida_com_negativos() {
        // -4 é par, -3 é ímpar.
        assert_eq!(remover_pares(&[-4, -3, 0, 7]), vec![-3, 7]);
    }

    #[test]
    fn todos_pares_resulta_vazio() {
        assert_eq!(remover_pares(&[2, 4, 6]), Vec::<i32>::new());
    }
}
