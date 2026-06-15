pub fn imprimir_pares_e_pares(lista: &[i32]) {
    for &elemento in lista {
        println!("Elemento: {}", elemento);
    }

    // aqui entram todos os pares, inclusive (i, i)
    for &(a, b) in &pares_completos(lista) {
        println!("Par: ({}, {})", a, b);
    }
}

pub fn pares_completos(lista: &[i32]) -> Vec<(i32, i32)> {
    let n = lista.len();
    let mut resultado: Vec<(i32, i32)> = Vec::new();

    for i in 0..n {
        for j in 0..n {
            resultado.push((lista[i], lista[j]));
        }
    }

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pares_completos_inclui_diagonal() {
        let v = vec![1, 2];
        let pares = pares_completos(&v);
        assert_eq!(pares, vec![(1, 1), (1, 2), (2, 1), (2, 2)]);
    }

    #[test]
    fn quantidade_de_pares_eh_n_ao_quadrado() {
        let v = vec![5, 6, 7];
        assert_eq!(pares_completos(&v).len(), 9);
    }

    #[test]
    fn lista_vazia_nao_gera_pares() {
        let v: Vec<i32> = Vec::new();
        assert_eq!(pares_completos(&v), Vec::new());
    }
}
