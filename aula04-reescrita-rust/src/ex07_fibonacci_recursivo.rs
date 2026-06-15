// cuidado: exponencial, nao usar com n grande
pub fn fibonacci_recursivo(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)
}

pub fn fibonacci_iterativo(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let mut anterior: u64 = 0;
    let mut atual: u64 = 1;
    for _ in 2..=n {
        let proximo = anterior + atual;
        anterior = atual;
        atual = proximo;
    }
    atual
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valores_base() {
        assert_eq!(fibonacci_recursivo(0), 0);
        assert_eq!(fibonacci_recursivo(1), 1);
        assert_eq!(fibonacci_iterativo(0), 0);
        assert_eq!(fibonacci_iterativo(1), 1);
    }

    #[test]
    fn valores_conhecidos() {
        assert_eq!(fibonacci_iterativo(10), 55);
        assert_eq!(fibonacci_iterativo(20), 6765);
    }

    #[test]
    fn recursivo_e_iterativo_batem_para_n_pequeno() {
        for n in 0..=30u64 {
            assert_eq!(fibonacci_recursivo(n), fibonacci_iterativo(n));
        }
    }
}
