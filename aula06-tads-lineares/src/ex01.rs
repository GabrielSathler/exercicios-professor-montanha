pub fn inverter(mut v: Vec<i32>) -> Vec<i32> {
    let mut resultado: Vec<i32> = Vec::with_capacity(v.len());
    while let Some(x) = v.pop() {
        resultado.push(x);
    }
    resultado
}

pub fn demo() {
    let original: Vec<i32> = vec![1, 2, 3, 4, 5];
    let invertido = inverter(original.clone());
    println!("  original  = {:?}", original);
    println!("  invertido = {:?}", invertido);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverte_vetor_simples() {
        assert_eq!(inverter(vec![1, 2, 3, 4, 5]), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn inverte_vetor_vazio() {
        let vazio: Vec<i32> = Vec::new();
        assert_eq!(inverter(vazio), Vec::<i32>::new());
    }

    #[test]
    fn inverte_um_elemento() {
        assert_eq!(inverter(vec![42]), vec![42]);
    }
}
