pub fn somar_lista(lista: &[i32]) -> i32 {
    let mut total = 0;
    for &elemento in lista {
        total += elemento;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soma_lista_nao_vazia() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(somar_lista(&v), 15);
    }

    #[test]
    fn soma_lista_vazia_eh_zero() {
        let v: Vec<i32> = Vec::new();
        assert_eq!(somar_lista(&v), 0);
    }

    #[test]
    fn soma_com_negativos() {
        let v = vec![10, -3, -7];
        assert_eq!(somar_lista(&v), 0);
    }
}
