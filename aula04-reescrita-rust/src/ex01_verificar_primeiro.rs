pub fn verificar_primeiro(lista: &[i32]) -> Option<i32> {
    lista.first().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retorna_primeiro_quando_ha_elementos() {
        let v = vec![10, 20, 30];
        assert_eq!(verificar_primeiro(&v), Some(10));
    }

    #[test]
    fn retorna_none_quando_vazia() {
        let v: Vec<i32> = Vec::new();
        assert_eq!(verificar_primeiro(&v), None);
    }
}
