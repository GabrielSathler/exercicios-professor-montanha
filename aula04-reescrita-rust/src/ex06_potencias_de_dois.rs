pub fn potencias_de_dois(n: u64) -> Vec<u64> {
    let mut i: u64 = 1;
    let mut resultado: Vec<u64> = Vec::new();
    while i < n {
        resultado.push(i);
        i *= 2;
    }
    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn potencias_menores_que_dez() {
        assert_eq!(potencias_de_dois(10), vec![1, 2, 4, 8]);
    }

    #[test]
    fn potencias_menores_que_dezesseis() {
        assert_eq!(potencias_de_dois(16), vec![1, 2, 4, 8]);
    }

    #[test]
    fn n_um_ou_zero_resulta_vazio() {
        assert_eq!(potencias_de_dois(1), Vec::<u64>::new());
        assert_eq!(potencias_de_dois(0), Vec::<u64>::new());
    }
}
